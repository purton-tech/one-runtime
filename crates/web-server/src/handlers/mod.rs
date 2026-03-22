pub mod api_keys;
pub mod integrations;
pub mod oauth_clients;
pub mod root;

use crate::CustomError;
use axum::{
    http::{HeaderValue, header::SET_COOKIE},
    response::{IntoResponse, Redirect, Response},
};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use clorinde::queries::orgs;
use clorinde::tokio_postgres::Transaction;

const FLASH_COOKIE_NAME: &str = "flash_aargh";

pub async fn load_balance_label(
    transaction: &Transaction<'_>,
    org_id: &str,
) -> Result<String, CustomError> {
    let balance = orgs::get_org_balance()
        .bind(transaction, &org_id)
        .one()
        .await?;
    Ok(format_balance_microcents(balance.balance_microcents))
}

pub fn format_balance_microcents(balance_microcents: i64) -> String {
    let is_negative = balance_microcents < 0;
    let abs_microcents = balance_microcents.unsigned_abs();
    let cents = abs_microcents / 1_000_000;
    let dollars = cents / 100;
    let cents_remainder = cents % 100;

    if is_negative {
        format!("-${dollars}.{cents_remainder:02}")
    } else {
        format!("${dollars}.{cents_remainder:02}")
    }
}

pub fn redirect_and_snackbar(
    url: &str,
    message: impl Into<String>,
) -> Result<Response, CustomError> {
    let mut response = Redirect::to(url).into_response();
    let encoded_message = URL_SAFE_NO_PAD.encode(message.into());
    let cookie_value =
        format!("{FLASH_COOKIE_NAME}={encoded_message}; Path=/; HttpOnly; SameSite=Lax");
    response.headers_mut().insert(
        SET_COOKIE,
        HeaderValue::from_str(&cookie_value)
            .map_err(|err| CustomError::FaultySetup(format!("Invalid flash cookie: {err}")))?,
    );
    Ok(response)
}

pub fn redirect(url: &str) -> Result<Response, CustomError> {
    Ok(Redirect::to(url).into_response())
}

#[cfg(test)]
mod tests {
    use super::redirect_and_snackbar;
    use axum::http::header::SET_COOKIE;
    use axum::response::IntoResponse;

    #[test]
    fn flash_cookie_is_set() {
        let response = redirect_and_snackbar("/next", "OAuth client created")
            .unwrap()
            .into_response();
        let cookie_header = response
            .headers()
            .get(SET_COOKIE)
            .unwrap()
            .to_str()
            .unwrap();
        assert!(cookie_header.contains("flash_aargh="));
        assert!(cookie_header.contains("Path=/"));
    }
}
