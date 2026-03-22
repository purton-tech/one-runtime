use crate::CustomError;
use clorinde::queries::auth;
use clorinde::tokio_postgres::Transaction;
use serde_json::Value;

pub fn supports_api_key_auth(spec_json: &str) -> Result<bool, CustomError> {
    let spec: Value = serde_json::from_str(spec_json)
        .map_err(|err| CustomError::FaultySetup(format!("Invalid OpenAPI spec: {err}")))?;

    let Some(schemes) = spec
        .pointer("/components/securitySchemes")
        .and_then(Value::as_object)
    else {
        return Ok(false);
    };

    Ok(schemes.values().any(|scheme| {
        scheme
            .get("type")
            .and_then(Value::as_str)
            .map(|value| value.eq_ignore_ascii_case("apiKey"))
            .unwrap_or(false)
    }))
}

pub async fn set_request_claims(
    transaction: &Transaction<'_>,
    issuer: &str,
    sub: &str,
) -> Result<(), CustomError> {
    auth::set_request_claim_iss()
        .bind(transaction, &issuer)
        .one()
        .await?;
    auth::set_request_claim_sub()
        .bind(transaction, &sub)
        .one()
        .await?;
    Ok(())
}
