use axum::{
    Extension, Form,
    response::{Html, IntoResponse, Redirect, Response},
};
use clorinde::{deadpool_postgres::Pool, queries::auth};
use rand::Rng;
use serde::Deserialize;

use crate::{CustomError, Jwt, authz, handlers, mcp::auth::hash_api_key_secret};

use web_ui::api_keys::page::CreatedApiKeyState;
use web_ui::routes;

#[derive(Debug, Deserialize, Clone)]
pub struct CreateApiKeyForm {
    pub label: String,
}

fn generate_api_key_parts() -> (String, String) {
    let mut prefix_bytes = [0_u8; 4];
    let mut secret_bytes = [0_u8; 16];
    let mut rng = rand::rng();
    rng.fill_bytes(&mut prefix_bytes);
    rng.fill_bytes(&mut secret_bytes);

    (hex::encode(prefix_bytes), hex::encode(secret_bytes))
}

async fn render_index(
    pool: &Pool,
    current_user: &Jwt,
    org_id: String,
    draft_label: Option<String>,
    error_message: Option<String>,
    created_key: Option<CreatedApiKeyState>,
) -> Result<Response, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let context = authz::init_request(&transaction, current_user).await?;
    if context.org_id != org_id {
        return Err(CustomError::FaultySetup(
            "Requested org_id is not available for current user".to_string(),
        ));
    }

    let api_keys = auth::list_org_api_keys()
        .bind(&transaction, &org_id)
        .all()
        .await?;
    let balance_label = handlers::load_balance_label(&transaction, &org_id).await?;

    transaction.commit().await?;

    let html = web_ui::api_keys::page::page(
        org_id,
        balance_label,
        api_keys,
        draft_label,
        error_message,
        created_key,
    );

    Ok(Html(html).into_response())
}

pub async fn action_create(
    routes::api_keys::Create { org_id }: routes::api_keys::Create,
    Extension(pool): Extension<Pool>,
    current_user: Jwt,
    Form(form): Form<CreateApiKeyForm>,
) -> Result<Response, CustomError> {
    let label = form.label.trim().to_string();
    if label.is_empty() {
        return render_index(
            &pool,
            &current_user,
            org_id,
            Some(form.label),
            Some("API key label is required".to_string()),
            None,
        )
        .await;
    }

    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let context = authz::init_request(&transaction, &current_user).await?;
    if context.org_id != org_id {
        return Err(CustomError::FaultySetup(
            "Requested org_id is not available for current user".to_string(),
        ));
    }

    let user = auth::get_current_user().bind(&transaction).one().await?;
    let (prefix, secret) = generate_api_key_parts();
    let key_prefix = format!("oru_{prefix}");
    let secret_hash = hash_api_key_secret(&secret);

    auth::create_api_key()
        .bind(
            &transaction,
            &user.id,
            &org_id,
            &label,
            &key_prefix,
            &secret_hash,
        )
        .one()
        .await?;

    let api_keys = auth::list_org_api_keys()
        .bind(&transaction, &org_id)
        .all()
        .await?;
    let balance_label = handlers::load_balance_label(&transaction, &org_id).await?;

    transaction.commit().await?;

    let html = web_ui::api_keys::page::page(
        org_id,
        balance_label,
        api_keys,
        None,
        None,
        Some(CreatedApiKeyState {
            label,
            token: format!("{key_prefix}_{secret}"),
        }),
    );

    Ok(Html(html).into_response())
}

pub async fn action_revoke(
    routes::api_keys::Revoke { org_id, id }: routes::api_keys::Revoke,
    Extension(pool): Extension<Pool>,
    current_user: Jwt,
) -> Result<Response, CustomError> {
    let api_key_id = uuid::Uuid::parse_str(&id)
        .map_err(|err| CustomError::FaultySetup(format!("Invalid API key id: {err}")))?;

    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let context = authz::init_request(&transaction, &current_user).await?;
    if context.org_id != org_id {
        return Err(CustomError::FaultySetup(
            "Requested org_id is not available for current user".to_string(),
        ));
    }

    auth::revoke_api_key()
        .bind(&transaction, &api_key_id, &org_id)
        .await?;

    transaction.commit().await?;

    let href = routes::api_keys::Index { org_id }.to_string();
    Ok(Redirect::to(&href).into_response())
}

#[cfg(test)]
mod tests {
    use super::generate_api_key_parts;

    #[test]
    fn generated_parts_are_separator_safe() {
        let (prefix, secret) = generate_api_key_parts();
        assert!(!prefix.contains('_'));
        assert!(!secret.contains('_'));
        assert!(!prefix.is_empty());
        assert!(!secret.is_empty());
    }
}
