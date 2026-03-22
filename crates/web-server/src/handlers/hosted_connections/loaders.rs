use axum::{
    Extension,
    extract::Query,
    http::{HeaderValue, StatusCode, header},
    response::{Html, Response},
};
use serde::Deserialize;
use web_ui::routes;

use crate::CustomError;
use clorinde::deadpool_postgres::Pool;

#[derive(Debug, Default, Deserialize)]
pub struct HostedConnectionPopupQuery {
    #[serde(default)]
    pub session_token: String,
    #[serde(default)]
    pub request_id: String,
}

pub async fn loader_sdk() -> Response {
    const SDK_JS: &str = r#"const DEFAULT_POPUP_WIDTH = 520;
const DEFAULT_POPUP_HEIGHT = 760;
const RESULT_SOURCE = "one-runtime";

function centerPopup(width, height) {
  const dualScreenLeft = window.screenLeft ?? window.screenX ?? 0;
  const dualScreenTop = window.screenTop ?? window.screenY ?? 0;
  const viewportWidth = window.innerWidth || document.documentElement.clientWidth || screen.width;
  const viewportHeight = window.innerHeight || document.documentElement.clientHeight || screen.height;
  const left = Math.max(0, dualScreenLeft + (viewportWidth - width) / 2);
  const top = Math.max(0, dualScreenTop + (viewportHeight - height) / 2);
  return `popup=yes,width=${width},height=${height},left=${left},top=${top}`;
}

function openConnection(baseUrl, sessionToken) {
  const requestId = crypto.randomUUID();
  const connectUrl = new URL("/connect", baseUrl);
  connectUrl.searchParams.set("session_token", sessionToken);
  connectUrl.searchParams.set("request_id", requestId);

  const popup = window.open(
    connectUrl.toString(),
    `one-runtime-connect-${requestId}`,
    centerPopup(DEFAULT_POPUP_WIDTH, DEFAULT_POPUP_HEIGHT),
  );

  if (!popup) {
    return Promise.reject(new Error("Failed to open hosted connection popup"));
  }

  const origin = new URL(baseUrl, window.location.href).origin;

  return new Promise((resolve) => {
    let settled = false;

    const cleanup = () => {
      window.removeEventListener("message", onMessage);
      window.clearInterval(intervalId);
    };

    const finish = (payload) => {
      if (settled) return;
      settled = true;
      cleanup();
      resolve(payload);
    };

    const onMessage = (event) => {
      if (event.origin !== origin) return;
      const data = event.data;
      if (!data || data.source !== RESULT_SOURCE || data.requestId !== requestId) return;
      finish(data);
    };

    window.addEventListener("message", onMessage);

    const intervalId = window.setInterval(() => {
      if (popup.closed) {
        finish({ source: RESULT_SOURCE, requestId, status: "cancelled" });
      }
    }, 250);
  });
}

export function createOneRuntime({ baseUrl } = {}) {
  const resolvedBaseUrl = baseUrl || window.location.origin;

  return {
    connections: {
      open({ sessionToken }) {
        if (!sessionToken) {
          return Promise.reject(new Error("sessionToken is required"));
        }
        return openConnection(resolvedBaseUrl, sessionToken);
      },
    },
  };
}
"#;

    Response::builder()
        .status(StatusCode::OK)
        .header(
            header::CONTENT_TYPE,
            HeaderValue::from_static("text/javascript; charset=utf-8"),
        )
        .header(
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
            HeaderValue::from_static("*"),
        )
        .body(axum::body::Body::from(SDK_JS))
        .unwrap()
}

pub async fn loader_popup(
    _: routes::hosted_connections::Popup,
    Query(query): Query<HostedConnectionPopupQuery>,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let session_token = query.session_token.trim().to_string();
    if session_token.is_empty() {
        let html = web_ui::hosted_connections::page::error_page(
            "Connection unavailable".to_string(),
            "The hosted connection session is missing.".to_string(),
        );
        return Ok(Html(html));
    }

    let request_id = if query.request_id.trim().is_empty() {
        uuid::Uuid::new_v4().to_string()
    } else {
        query.request_id.trim().to_string()
    };

    let client = pool.get().await?;
    let session = clorinde::queries::hosted_connections::get_hosted_connection_session()
        .bind(&client, &session_token)
        .opt()
        .await?;

    let Some(session) = session else {
        let html = web_ui::hosted_connections::page::error_page(
            "Connection unavailable".to_string(),
            "This hosted connection session is not valid.".to_string(),
        );
        return Ok(Html(html));
    };

    if session.expired || session.used || session.auth_type != "api_key" {
        let html = web_ui::hosted_connections::page::error_page(
            "Connection unavailable".to_string(),
            "This hosted connection session can no longer be used.".to_string(),
        );
        return Ok(Html(html));
    }

    let html = web_ui::hosted_connections::page::page(
        web_ui::hosted_connections::page::HostedConnectionPageModel {
            integration_name: session.integration_name,
            session_token,
            request_id,
            suggested_connection_name: session.suggested_connection_name,
            end_user_id: session.end_user_id,
            end_user_name: session.end_user_name,
            end_user_email: session.end_user_email,
            error_message: None,
        },
    );
    Ok(Html(html))
}
