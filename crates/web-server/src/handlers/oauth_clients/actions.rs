use crate::{CustomError, Jwt, authz, handlers};
use axum::{Extension, Form, response::IntoResponse};
use clorinde::deadpool_postgres::Pool;
use serde::Deserialize;
use web_ui::routes;

#[derive(Debug, Deserialize, Clone)]
pub struct CreateOAuthClientForm {
    pub provider: String,
    pub client_id: String,
    pub client_secret: String,
}

pub async fn action_create(
    routes::oauth_clients::Create { org_id }: routes::oauth_clients::Create,
    Extension(pool): Extension<Pool>,
    current_user: Jwt,
    Form(CreateOAuthClientForm {
        provider,
        client_id,
        client_secret,
    }): Form<CreateOAuthClientForm>,
) -> Result<impl IntoResponse, CustomError> {
    let provider = provider.trim().to_string();
    let client_id = client_id.trim().to_string();
    let client_secret = client_secret.trim().to_string();
    let href = routes::oauth_clients::Index {
        org_id: org_id.clone(),
    }
    .to_string();

    if provider.is_empty() || client_id.is_empty() || client_secret.is_empty() {
        return handlers::redirect_and_snackbar(&href, "Invalid form submission");
    }

    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let context = authz::init_request(&transaction, &current_user).await?;
    if context.org_id != org_id {
        return Err(CustomError::FaultySetup(
            "Requested org_id is not available for current user".to_string(),
        ));
    }

    let create_result = clorinde::queries::oauth_clients::create_oauth_client()
        .bind(&transaction, &org_id, &provider, &client_id, &client_secret)
        .one()
        .await;

    let inserted = match create_result {
        Ok(inserted) => inserted,
        Err(err) => {
            if let Some(db_error) = err.as_db_error()
                && db_error.code().code() == "23505"
            {
                transaction.rollback().await?;
                return handlers::redirect_and_snackbar(&href, "OAuth client already exists");
            }
            return Err(err.into());
        }
    };

    if !inserted.changed {
        return Err(CustomError::FaultySetup(
            "OAuth client was not created".to_string(),
        ));
    }

    transaction.commit().await?;

    handlers::redirect_and_snackbar(&href, "OAuth client created")
}
