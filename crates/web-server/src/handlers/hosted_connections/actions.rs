use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse, Response},
};
use clorinde::deadpool_postgres::Pool;
use rand::Rng;
use serde::{Deserialize, Serialize};
use web_ui::routes;

use crate::{
    CustomError,
    mcp::{AppState, auth::ApiKeyAuthError},
};

use super::support::{set_request_claims, supports_api_key_auth};

#[derive(Debug, Deserialize)]
pub struct CreateHostedConnectionSessionRequest {
    pub integration_slug: String,
    pub end_user_id: String,
    pub end_user_name: Option<String>,
    pub end_user_email: Option<String>,
    pub suggested_connection_name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateHostedConnectionSessionResponse {
    pub session_token: String,
    pub connect_url: String,
}

#[derive(Debug, Deserialize)]
pub struct SubmitHostedConnectionForm {
    pub session_token: String,
    pub request_id: String,
    pub connection_name: String,
    pub api_key: String,
}

fn generate_session_token() -> String {
    let mut bytes = [0_u8; 24];
    rand::rng().fill_bytes(&mut bytes);
    format!("orcs_{}", hex::encode(bytes))
}

fn json_error(status: StatusCode, message: impl Into<String>) -> Response {
    (status, Json(serde_json::json!({ "error": message.into() }))).into_response()
}

fn popup_completion_page(
    request_id: &str,
    status: &str,
    connection_id: Option<String>,
    integration_slug: &str,
    connection_name: Option<String>,
) -> String {
    let payload = serde_json::json!({
        "source": "one-runtime",
        "requestId": request_id,
        "status": status,
        "connectionId": connection_id,
        "integrationSlug": integration_slug,
        "connectionName": connection_name,
    });

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Connection complete</title>
  </head>
  <body>
    <script>
      const payload = {payload};
      if (window.opener) {{
        window.opener.postMessage(payload, "*");
      }}
      window.close();
    </script>
    <p>You can close this window.</p>
  </body>
</html>"#,
        payload = payload
    )
}

fn invalid_popup_page(message: &str) -> Html<String> {
    Html(web_ui::hosted_connections::page::error_page(
        "Connection unavailable".to_string(),
        message.to_string(),
    ))
}

fn form_error_page(
    session: clorinde::queries::hosted_connections::HostedConnectionSessionContext,
    session_token: String,
    request_id: String,
    connection_name: String,
    error_message: &str,
) -> Html<String> {
    Html(web_ui::hosted_connections::page::page(
        web_ui::hosted_connections::page::HostedConnectionPageModel {
            integration_name: session.integration_name,
            session_token,
            request_id,
            suggested_connection_name: connection_name,
            end_user_id: session.end_user_id,
            end_user_name: session.end_user_name,
            end_user_email: session.end_user_email,
            error_message: Some(error_message.to_string()),
        },
    ))
}

fn map_api_key_auth_error(err: ApiKeyAuthError) -> Response {
    match err {
        ApiKeyAuthError::Unauthorized(message) => json_error(StatusCode::UNAUTHORIZED, message),
        ApiKeyAuthError::Internal(message) => {
            json_error(StatusCode::INTERNAL_SERVER_ERROR, message)
        }
    }
}

pub async fn action_create_session(
    _: routes::hosted_connections::CreateSession,
    State(state): State<std::sync::Arc<AppState>>,
    headers: HeaderMap,
    Json(body): Json<CreateHostedConnectionSessionRequest>,
) -> Result<Response, CustomError> {
    let principal = match crate::mcp::auth::authenticate_api_key(state.as_ref(), &headers).await {
        Ok(principal) => principal,
        Err(err) => return Ok(map_api_key_auth_error(err)),
    };

    let integration_slug = body.integration_slug.trim().to_ascii_lowercase();
    let end_user_id = body.end_user_id.trim().to_string();
    let end_user_name = body.end_user_name.unwrap_or_default().trim().to_string();
    let end_user_email = body.end_user_email.unwrap_or_default().trim().to_string();
    let suggested_connection_name = body
        .suggested_connection_name
        .unwrap_or_default()
        .trim()
        .to_string();

    if integration_slug.is_empty() || end_user_id.is_empty() {
        return Ok(json_error(
            StatusCode::UNPROCESSABLE_ENTITY,
            "integration_slug and end_user_id are required",
        ));
    }

    let mut client = state.pool.get().await?;
    let transaction = client.transaction().await?;

    let integration = clorinde::queries::hosted_connections::get_system_integration_by_slug()
        .bind(&transaction, &integration_slug)
        .opt()
        .await?;

    let Some(integration) = integration else {
        transaction.rollback().await?;
        return Ok(json_error(StatusCode::NOT_FOUND, "Integration not found"));
    };

    if !supports_api_key_auth(&integration.openapi_spec)? {
        transaction.rollback().await?;
        return Ok(json_error(
            StatusCode::UNPROCESSABLE_ENTITY,
            "Integration does not support API-key hosted connections",
        ));
    }

    let session_token = generate_session_token();
    let expires_at = (chrono::Utc::now() + chrono::Duration::minutes(10)).fixed_offset();

    let session = clorinde::queries::hosted_connections::create_hosted_connection_session()
        .bind(
            &transaction,
            &principal.org_id,
            &principal.user_id,
            &principal.api_key_id,
            &integration.id,
            &integration_slug,
            &end_user_id,
            &end_user_name,
            &end_user_email,
            &suggested_connection_name,
            &clorinde::types::IntegrationAuthType::api_key,
            &session_token,
            &expires_at,
        )
        .one()
        .await?;

    transaction.commit().await?;

    let connect_url = format!(
        "{}/connect?session_token={}",
        state.config.app_base_url.trim_end_matches('/'),
        session.token
    );

    Ok(Json(CreateHostedConnectionSessionResponse {
        session_token: session.token,
        connect_url,
    })
    .into_response())
}

pub async fn action_submit(
    _: routes::hosted_connections::Submit,
    axum::Extension(pool): axum::Extension<Pool>,
    axum::Form(form): axum::Form<SubmitHostedConnectionForm>,
) -> Result<Html<String>, CustomError> {
    let session_token = form.session_token.trim().to_string();
    let request_id = form.request_id.trim().to_string();
    let connection_name = form.connection_name.trim().to_string();
    let api_key = form.api_key.trim().to_string();

    if session_token.is_empty() {
        return Ok(invalid_popup_page(
            "The hosted connection session is missing.",
        ));
    }

    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let session = clorinde::queries::hosted_connections::get_hosted_connection_session_for_update()
        .bind(&transaction, &session_token)
        .opt()
        .await?;

    let Some(session) = session else {
        transaction.rollback().await?;
        return Ok(invalid_popup_page(
            "This hosted connection session is not valid.",
        ));
    };

    if session.expired || session.used || session.auth_type != "api_key" {
        transaction.rollback().await?;
        return Ok(invalid_popup_page(
            "This hosted connection session can no longer be used.",
        ));
    }

    if connection_name.is_empty() || api_key.is_empty() {
        transaction.rollback().await?;
        return Ok(form_error_page(
            session,
            session_token,
            request_id,
            connection_name,
            "Invalid form submission",
        ));
    }

    set_request_claims(
        &transaction,
        &session.created_by_issuer,
        &session.created_by_sub,
    )
    .await?;

    let created = clorinde::queries::hosted_connections::create_api_key_integration_connection()
        .bind(
            &transaction,
            &session.org_public_id,
            &session.integration_id,
            &connection_name,
            &api_key,
            &session.end_user_id,
            &session.end_user_name,
            &session.end_user_email,
        )
        .one()
        .await?;

    clorinde::queries::hosted_connections::mark_hosted_connection_session_used()
        .bind(&transaction, &session_token)
        .await?;

    transaction.commit().await?;

    let html = popup_completion_page(
        &request_id,
        "connected",
        Some(created.id.to_string()),
        &session.integration_slug,
        Some(created.name),
    );

    Ok(Html(html))
}

#[cfg(test)]
mod tests {
    use super::generate_session_token;

    #[test]
    fn generated_session_token_has_expected_prefix() {
        let token = generate_session_token();
        assert!(token.starts_with("orcs_"));
        assert!(!token.contains(' '));
    }
}
