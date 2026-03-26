use axum::{
    Json,
    extract::{Query, State},
    http::header,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use clorinde::deadpool_postgres::Pool;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tracing::{debug, warn};
use web_ui::routes;

use crate::{
    CustomError,
    mcp::{AppState, auth::ApiKeyAuthError},
};

use super::{
    loaders::{PublicIntegrationListQuery, public_empty_response, public_json_response},
    support::{set_request_claims, supports_api_key_auth},
};

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

#[derive(Debug, Serialize)]
pub struct PublicHostedIntegrationResponse {
    pub id: String,
    pub slug: String,
    pub name: String,
    pub description: String,
    pub logo_url: Option<String>,
    pub category: Option<String>,
    pub status: String,
    pub supported_auth_types: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct PublicCatalogIntegrationResponse {
    pub id: String,
    pub slug: String,
    pub name: String,
    pub description: String,
    pub logo_url: Option<String>,
    pub category: Option<String>,
    pub supported_auth_types: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct PublicHostedIntegrationListResponse {
    pub end_user_id: String,
    pub integrations: Vec<PublicHostedIntegrationResponse>,
}

#[derive(Debug, Serialize)]
pub struct PublicCatalogIntegrationListResponse {
    pub integrations: Vec<PublicCatalogIntegrationResponse>,
}

#[derive(Debug, Deserialize)]
pub struct DisconnectHostedConnectionsRequest {
    pub integration_slug: String,
    pub end_user_id: String,
}

#[derive(Debug, Serialize)]
pub struct DisconnectHostedConnectionsResponse {
    pub status: String,
    pub integration_slug: String,
    pub end_user_id: String,
    pub deleted_count: i64,
}

#[derive(Debug, Deserialize)]
pub struct SubmitHostedConnectionForm {
    pub session_token: String,
    pub request_id: String,
    pub connection_name: String,
    pub api_key: String,
}

#[derive(Debug, Serialize)]
pub struct SubmitHostedConnectionResponse {
    pub source: String,
    #[serde(rename = "requestId")]
    pub request_id: String,
    pub status: String,
    #[serde(rename = "connectionId")]
    pub connection_id: Option<String>,
    #[serde(rename = "integrationSlug")]
    pub integration_slug: String,
    #[serde(rename = "connectionName")]
    pub connection_name: Option<String>,
    pub error: Option<String>,
}

fn generate_session_token() -> String {
    let mut bytes = [0_u8; 24];
    rand::rng().fill_bytes(&mut bytes);
    format!("orcs_{}", hex::encode(bytes))
}

fn redact_token(token: &str) -> String {
    let trimmed = token.trim();
    if trimmed.len() <= 12 {
        return trimmed.to_string();
    }
    format!("{}...{}", &trimmed[..8], &trimmed[trimmed.len() - 4..])
}

fn public_json_error(status: StatusCode, message: impl Into<String>) -> Response {
    public_json_response(json!({ "error": message.into() }), status)
}

fn completion_payload(
    request_id: &str,
    status: &str,
    connection_id: Option<String>,
    integration_slug: &str,
    connection_name: Option<String>,
) -> SubmitHostedConnectionResponse {
    SubmitHostedConnectionResponse {
        source: "one-runtime".to_string(),
        request_id: request_id.to_string(),
        status: status.to_string(),
        connection_id,
        integration_slug: integration_slug.to_string(),
        connection_name,
        error: None,
    }
}

fn submit_json_error(
    status: StatusCode,
    request_id: String,
    integration_slug: String,
    message: impl Into<String>,
) -> Response {
    let payload = SubmitHostedConnectionResponse {
        source: "one-runtime".to_string(),
        request_id,
        status: "error".to_string(),
        connection_id: None,
        integration_slug,
        connection_name: None,
        error: Some(message.into()),
    };

    (status, Json(payload)).into_response()
}

fn map_api_key_auth_error(err: ApiKeyAuthError) -> Response {
    match err {
        ApiKeyAuthError::Unauthorized(message) => {
            public_json_error(StatusCode::UNAUTHORIZED, message)
        }
        ApiKeyAuthError::Internal(message) => {
            public_json_error(StatusCode::INTERNAL_SERVER_ERROR, message)
        }
    }
}

async fn create_session_response(
    state: &std::sync::Arc<AppState>,
    headers: &HeaderMap,
    body: CreateHostedConnectionSessionRequest,
) -> Result<Response, CustomError> {
    let principal = match crate::mcp::auth::authenticate_api_key(state.as_ref(), headers).await {
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
        return Ok(public_json_error(
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
        return Ok(public_json_error(
            StatusCode::NOT_FOUND,
            "Integration not found",
        ));
    };

    if !supports_api_key_auth(&integration.openapi_spec)? {
        transaction.rollback().await?;
        return Ok(public_json_error(
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

    debug!(
        integration_slug = %integration_slug,
        end_user_id = %end_user_id,
        session_token = %redact_token(&session.token),
        "created hosted connection session"
    );

    let connect_url = format!(
        "{}/connect?session_token={}",
        state.config.app_base_url.trim_end_matches('/'),
        session.token
    );

    Ok(public_json_response(
        json!(CreateHostedConnectionSessionResponse {
            session_token: session.token,
            connect_url,
        }),
        StatusCode::OK,
    ))
}

pub async fn action_create_session(
    _: routes::hosted_connections::CreateSession,
    State(state): State<std::sync::Arc<AppState>>,
    headers: HeaderMap,
    Json(body): Json<CreateHostedConnectionSessionRequest>,
) -> Result<Response, CustomError> {
    let mut response = create_session_response(&state, &headers, body).await?;
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        axum::http::HeaderValue::from_static("application/json; charset=utf-8"),
    );
    Ok(response)
}

pub async fn action_create_session_public(
    State(state): State<std::sync::Arc<AppState>>,
    headers: HeaderMap,
    Json(body): Json<CreateHostedConnectionSessionRequest>,
) -> Result<Response, CustomError> {
    create_session_response(&state, &headers, body).await
}

pub async fn action_list_integrations(
    State(state): State<std::sync::Arc<AppState>>,
    headers: HeaderMap,
    Query(query): Query<PublicIntegrationListQuery>,
) -> Result<Response, CustomError> {
    let principal = match crate::mcp::auth::authenticate_api_key(state.as_ref(), &headers).await {
        Ok(principal) => principal,
        Err(err) => return Ok(map_api_key_auth_error(err)),
    };

    let end_user_id = query.end_user_id.trim().to_string();
    if end_user_id.is_empty() {
        return Ok(public_json_error(
            StatusCode::UNPROCESSABLE_ENTITY,
            "end_user_id is required",
        ));
    }

    let mut client = state.pool.get().await?;
    let transaction = client.transaction().await?;
    set_request_claims(&transaction, &principal.issuer, &principal.sub).await?;

    let integrations = clorinde::queries::hosted_connections::list_public_hosted_integrations()
        .bind(&transaction, &principal.org_public_id, &end_user_id)
        .all()
        .await?;

    transaction.commit().await?;

    let integrations = integrations
        .into_iter()
        .filter_map(|integration| match build_public_integration(integration) {
            Ok(Some(integration)) => Some(Ok(integration)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(public_json_response(
        json!(PublicHostedIntegrationListResponse {
            end_user_id,
            integrations,
        }),
        StatusCode::OK,
    ))
}

pub async fn action_list_catalog_integrations(
    State(state): State<std::sync::Arc<AppState>>,
) -> Result<Response, CustomError> {
    let client = state.pool.get().await?;

    let integrations = clorinde::queries::hosted_connections::list_public_catalog_integrations()
        .bind(&client)
        .all()
        .await?;

    let integrations = integrations
        .into_iter()
        .filter_map(
            |integration| match build_public_catalog_integration(integration) {
                Ok(Some(integration)) => Some(Ok(integration)),
                Ok(None) => None,
                Err(err) => Some(Err(err)),
            },
        )
        .collect::<Result<Vec<_>, _>>()?;

    Ok(public_json_response(
        json!(PublicCatalogIntegrationListResponse { integrations }),
        StatusCode::OK,
    ))
}

pub async fn action_disconnect_public(
    State(state): State<std::sync::Arc<AppState>>,
    headers: HeaderMap,
    Json(body): Json<DisconnectHostedConnectionsRequest>,
) -> Result<Response, CustomError> {
    let principal = match crate::mcp::auth::authenticate_api_key(state.as_ref(), &headers).await {
        Ok(principal) => principal,
        Err(err) => return Ok(map_api_key_auth_error(err)),
    };

    let integration_slug = body.integration_slug.trim().to_ascii_lowercase();
    let end_user_id = body.end_user_id.trim().to_string();

    debug!(
        integration_slug = %integration_slug,
        end_user_id = %end_user_id,
        "received public disconnect request"
    );

    if integration_slug.is_empty() || end_user_id.is_empty() {
        warn!(
            integration_slug = %integration_slug,
            end_user_id = %end_user_id,
            "public disconnect request missing required fields"
        );
        return Ok(public_json_error(
            StatusCode::UNPROCESSABLE_ENTITY,
            "integration_slug and end_user_id are required",
        ));
    }

    let mut client = state.pool.get().await?;
    let transaction = client.transaction().await?;
    set_request_claims(&transaction, &principal.issuer, &principal.sub).await?;

    let result = clorinde::queries::hosted_connections::disconnect_public_hosted_integrations()
        .bind(
            &transaction,
            &principal.org_public_id,
            &integration_slug,
            &end_user_id,
        )
        .one()
        .await?;

    transaction.commit().await?;

    debug!(
        integration_slug = %integration_slug,
        end_user_id = %end_user_id,
        deleted_count = result.deleted_count,
        "public disconnect completed"
    );

    Ok(public_json_response(
        json!(DisconnectHostedConnectionsResponse {
            status: if result.deleted_count > 0 {
                "disconnected".to_string()
            } else {
                "not_found".to_string()
            },
            integration_slug,
            end_user_id,
            deleted_count: result.deleted_count,
        }),
        StatusCode::OK,
    ))
}

pub async fn options_public_api() -> Response {
    public_empty_response(StatusCode::NO_CONTENT)
}

async fn submit_connection_response(
    pool: &Pool,
    form: SubmitHostedConnectionForm,
) -> Result<Response, CustomError> {
    let session_token = form.session_token.trim().to_string();
    let request_id = form.request_id.trim().to_string();
    let connection_name = form.connection_name.trim().to_string();
    let api_key = form.api_key.trim().to_string();

    debug!(
        request_id = %request_id,
        session_token = %redact_token(&session_token),
        connection_name = %connection_name,
        "received hosted connection submit"
    );

    if session_token.is_empty() {
        return Ok(submit_json_error(
            StatusCode::UNPROCESSABLE_ENTITY,
            request_id,
            String::new(),
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
        warn!(
            request_id = %request_id,
            session_token = %redact_token(&session_token),
            "hosted connection submit session lookup failed"
        );
        return Ok(submit_json_error(
            StatusCode::GONE,
            request_id,
            String::new(),
            "This hosted connection session is not valid.",
        ));
    };

    if session.expired || session.used || session.auth_type != "api_key" {
        transaction.rollback().await?;
        warn!(
            request_id = %request_id,
            session_token = %redact_token(&session_token),
            integration_slug = %session.integration_slug,
            expired = session.expired,
            used = session.used,
            auth_type = %session.auth_type,
            "hosted connection submit session rejected"
        );
        return Ok(submit_json_error(
            StatusCode::GONE,
            request_id,
            session.integration_slug,
            "This hosted connection session can no longer be used.",
        ));
    }

    if connection_name.is_empty() || api_key.is_empty() {
        transaction.rollback().await?;
        return Ok(submit_json_error(
            StatusCode::UNPROCESSABLE_ENTITY,
            request_id,
            session.integration_slug,
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

    debug!(
        request_id = %request_id,
        session_token = %redact_token(&session_token),
        integration_slug = %session.integration_slug,
        connection_id = %created.id,
        "hosted connection submit succeeded"
    );

    let payload = completion_payload(
        &request_id,
        "connected",
        Some(created.id.to_string()),
        &session.integration_slug,
        Some(created.name),
    );

    Ok(Json(payload).into_response())
}

pub async fn action_submit(
    _: routes::hosted_connections::Submit,
    axum::Extension(pool): axum::Extension<Pool>,
    axum::Form(form): axum::Form<SubmitHostedConnectionForm>,
) -> Response {
    let request_id = form.request_id.trim().to_string();
    match submit_connection_response(&pool, form).await {
        Ok(response) => response,
        Err(err) => submit_json_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            request_id,
            String::new(),
            err.to_string(),
        ),
    }
}

pub async fn action_submit_json(
    axum::Extension(pool): axum::Extension<Pool>,
    Json(form): Json<SubmitHostedConnectionForm>,
) -> Response {
    let request_id = form.request_id.trim().to_string();
    match submit_connection_response(&pool, form).await {
        Ok(response) => response,
        Err(err) => submit_json_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            request_id,
            String::new(),
            err.to_string(),
        ),
    }
}

fn build_public_integration(
    integration: clorinde::queries::hosted_connections::PublicHostedIntegration,
) -> Result<Option<PublicHostedIntegrationResponse>, CustomError> {
    let Some(metadata) = build_public_catalog_metadata(&integration.openapi_spec)? else {
        return Ok(None);
    };

    Ok(Some(PublicHostedIntegrationResponse {
        id: integration.id.to_string(),
        slug: integration.slug,
        name: integration.name,
        description: integration.description,
        logo_url: metadata.logo_url,
        category: metadata.category,
        status: if integration.connected {
            "connected".to_string()
        } else {
            "not_connected".to_string()
        },
        supported_auth_types: metadata.supported_auth_types,
    }))
}

fn build_public_catalog_integration(
    integration: clorinde::queries::hosted_connections::HostedIntegration,
) -> Result<Option<PublicCatalogIntegrationResponse>, CustomError> {
    let Some(metadata) = build_public_catalog_metadata(&integration.openapi_spec)? else {
        return Ok(None);
    };

    Ok(Some(PublicCatalogIntegrationResponse {
        id: integration.id.to_string(),
        slug: integration.slug,
        name: integration.name,
        description: integration.description,
        logo_url: metadata.logo_url,
        category: metadata.category,
        supported_auth_types: metadata.supported_auth_types,
    }))
}

struct PublicCatalogMetadata {
    logo_url: Option<String>,
    category: Option<String>,
    supported_auth_types: Vec<String>,
}

fn build_public_catalog_metadata(
    openapi_spec: &str,
) -> Result<Option<PublicCatalogMetadata>, CustomError> {
    if !supports_api_key_auth(openapi_spec)? {
        return Ok(None);
    }

    let spec: Value = serde_json::from_str(openapi_spec).map_err(|err| {
        CustomError::FaultySetup(format!(
            "Invalid OpenAPI spec stored for integration: {err}"
        ))
    })?;

    Ok(Some(PublicCatalogMetadata {
        logo_url: info_string(&spec, "x-logo").or_else(|| {
            spec.pointer("/info/x-logo/url")
                .and_then(Value::as_str)
                .map(ToString::to_string)
        }),
        category: info_string(&spec, "x-category"),
        supported_auth_types: vec!["api_key".to_string()],
    }))
}

fn info_string(spec: &Value, key: &str) -> Option<String> {
    spec.pointer(&format!("/info/{key}"))
        .and_then(Value::as_str)
        .map(ToString::to_string)
        .filter(|value| !value.trim().is_empty())
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
