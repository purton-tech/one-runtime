use axum::http::HeaderMap;
use clorinde::queries::auth;
use rmcp::ErrorData;
use serde::Serialize;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use super::AppState;

#[derive(Clone, Debug, Serialize)]
pub struct McpPrincipal {
    pub api_key_id: Uuid,
    pub user_id: Uuid,
    pub org_id: Uuid,
    pub org_public_id: String,
    pub email: String,
    pub issuer: String,
    pub sub: String,
    pub label: String,
    pub key_prefix: String,
}

pub async fn authenticate(
    state: &AppState,
    headers: &HeaderMap,
) -> Result<McpPrincipal, ErrorData> {
    let raw_key = bearer_token(headers)?;
    let parsed = parse_api_key(raw_key)?;
    let secret_hash = hash_api_key_secret(parsed.secret);

    let client = state
        .pool
        .get()
        .await
        .map_err(|err| ErrorData::internal_error(err.to_string(), None))?;

    let lookup = auth::get_api_key_for_auth()
        .bind(&client, &parsed.key_prefix)
        .opt()
        .await
        .map_err(|err| ErrorData::internal_error(err.to_string(), None))?
        .ok_or_else(|| unauthorized("Invalid API key"))?;

    if lookup.secret_hash != secret_hash {
        return Err(unauthorized("Invalid API key"));
    }

    auth::touch_api_key_last_used()
        .bind(&client, &lookup.id)
        .await
        .map_err(|err| ErrorData::internal_error(err.to_string(), None))?;

    Ok(McpPrincipal {
        api_key_id: lookup.id,
        user_id: lookup.user_id,
        org_id: lookup.org_id,
        org_public_id: lookup.org_public_id,
        email: lookup.email,
        issuer: lookup.issuer,
        sub: lookup.sub,
        label: lookup.label,
        key_prefix: lookup.key_prefix,
    })
}

fn bearer_token(headers: &HeaderMap) -> Result<&str, ErrorData> {
    let value = headers
        .get(axum::http::header::AUTHORIZATION)
        .ok_or_else(|| unauthorized("Missing Authorization header"))?;
    let value = value
        .to_str()
        .map_err(|_| unauthorized("Authorization header is not valid UTF-8"))?;
    value
        .strip_prefix("Bearer ")
        .ok_or_else(|| unauthorized("Authorization header must use Bearer auth"))
}

struct ParsedApiKey<'a> {
    key_prefix: String,
    secret: &'a str,
}

fn parse_api_key(raw_key: &str) -> Result<ParsedApiKey<'_>, ErrorData> {
    let remainder = raw_key
        .strip_prefix("oru_")
        .ok_or_else(|| unauthorized("API keys must start with 'oru_'"))?;
    let (prefix, secret) = remainder
        .split_once('_')
        .ok_or_else(|| unauthorized("API key format must be 'oru_<prefix>_<secret>'"))?;
    if prefix.is_empty() || secret.is_empty() {
        return Err(unauthorized(
            "API key prefix and secret must both be present",
        ));
    }

    Ok(ParsedApiKey {
        key_prefix: format!("oru_{prefix}"),
        secret,
    })
}

pub fn hash_api_key_secret(secret: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(secret.as_bytes());
    hex::encode(hasher.finalize())
}

fn unauthorized(message: &str) -> ErrorData {
    ErrorData::invalid_params(message.to_string(), None)
}

#[cfg(test)]
mod tests {
    use super::{hash_api_key_secret, parse_api_key};

    #[test]
    fn parses_expected_api_key_shape() {
        let parsed = parse_api_key("oru_demo_deadbeef").expect("api key should parse");
        assert_eq!(parsed.key_prefix, "oru_demo");
        assert_eq!(parsed.secret, "deadbeef");
    }

    #[test]
    fn hashes_secret_stably() {
        assert_eq!(
            hash_api_key_secret("deadbeef"),
            "2baf1f40105d9501fe319a8ec463fdf4325a2a5df445adf3f572f626253678c9"
        );
    }
}
