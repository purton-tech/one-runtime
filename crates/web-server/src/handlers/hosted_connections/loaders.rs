use axum::{
    Extension,
    extract::Query,
    http::{HeaderValue, StatusCode, header},
    response::{Html, Response},
};
use serde::Deserialize;
use serde_json::Value;
use tracing::{debug, warn};
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

#[derive(Debug, Deserialize)]
pub struct PublicIntegrationListQuery {
    pub end_user_id: String,
}

fn redact_token(token: &str) -> String {
    let trimmed = token.trim();
    if trimmed.len() <= 12 {
        return trimmed.to_string();
    }
    format!("{}...{}", &trimmed[..8], &trimmed[trimmed.len() - 4..])
}

pub async fn loader_sdk() -> Response {
    const SDK_JS: &str = r##"const MODAL_ID = "one-runtime-connect-modal";
const RESULT_SOURCE = "one-runtime";
const FRAME_ID = "one-runtime-connect-frame";

function removeModal() {
  const existing = document.getElementById(MODAL_ID);
  if (existing) existing.remove();
}

function createModal(connectUrl) {
  removeModal();

  const root = document.createElement("div");
  root.id = MODAL_ID;
  root.style.position = "fixed";
  root.style.inset = "0";
  root.style.zIndex = "9999";
  root.style.display = "flex";
  root.style.alignItems = "center";
  root.style.justifyContent = "center";
  root.style.padding = "24px";
  root.style.background = "rgba(15, 23, 42, 0.55)";

  const panel = document.createElement("div");
  panel.style.width = "min(100%, 560px)";
  panel.style.height = "min(92vh, 820px)";
  panel.style.background = "white";
  panel.style.borderRadius = "20px";
  panel.style.overflow = "hidden";
  panel.style.boxShadow = "0 25px 80px rgba(15, 23, 42, 0.35)";
  panel.style.border = "1px solid rgba(148, 163, 184, 0.35)";
  panel.style.display = "flex";
  panel.style.flexDirection = "column";

  const topbar = document.createElement("div");
  topbar.style.display = "flex";
  topbar.style.justifyContent = "flex-end";
  topbar.style.padding = "12px 12px 0 12px";

  const close = document.createElement("button");
  close.type = "button";
  close.textContent = "Close";
  close.setAttribute("aria-label", "Close");
  close.style.border = "0";
  close.style.background = "transparent";
  close.style.cursor = "pointer";
  close.style.font = "600 14px system-ui, sans-serif";
  close.style.color = "#475569";
  topbar.appendChild(close);

  const frame = document.createElement("iframe");
  frame.id = FRAME_ID;
  frame.src = connectUrl;
  frame.title = "Connect integration";
  frame.style.width = "100%";
  frame.style.height = "100%";
  frame.style.border = "0";
  frame.style.background = "white";

  panel.appendChild(topbar);
  panel.appendChild(frame);
  root.appendChild(panel);
  document.body.appendChild(root);

  return { root, close };
}

function openConnection(baseUrl, sessionToken, connectUrlOverride) {
  const requestId = crypto.randomUUID();
  const connectUrl = connectUrlOverride ? new URL(connectUrlOverride, window.location.href) : new URL("/connect", baseUrl);
  connectUrl.searchParams.set("session_token", sessionToken);
  connectUrl.searchParams.set("request_id", requestId);
  const modal = createModal(connectUrl.toString());

  const origin = new URL(baseUrl, window.location.href).origin;

  return new Promise((resolve) => {
    let settled = false;

    const cleanup = () => {
      window.removeEventListener("message", onMessage);
      window.removeEventListener("keydown", onKeydown);
      modal.close.removeEventListener("click", onCancel);
      removeModal();
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

    const onCancel = () => {
      finish({ source: RESULT_SOURCE, requestId, status: "cancelled" });
    };

    const onKeydown = (event) => {
      if (event.key === "Escape") onCancel();
    };

    window.addEventListener("message", onMessage);
    window.addEventListener("keydown", onKeydown);
    modal.close.addEventListener("click", onCancel);
  });
}

export function createOneRuntime({ baseUrl } = {}) {
  const resolvedBaseUrl = baseUrl || window.location.origin;

  return {
    connections: {
      open({ sessionToken, connectUrl }) {
        if (!sessionToken) {
          return Promise.reject(new Error("sessionToken is required"));
        }
        return openConnection(resolvedBaseUrl, sessionToken, connectUrl);
      },
    },
  };
}
"##;

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

pub async fn loader_tester(_: routes::public_connect::Tester) -> Html<String> {
    Html(web_ui::public_connect::tester::page())
}

pub async fn loader_popup(
    _: routes::hosted_connections::Popup,
    Query(query): Query<HostedConnectionPopupQuery>,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let session_token = query.session_token.trim().to_string();
    if session_token.is_empty() {
        warn!("hosted connection popup missing session token");
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

    debug!(
        request_id = %request_id,
        session_token = %redact_token(&session_token),
        "loading hosted connection popup"
    );

    let client = pool.get().await?;
    let session = clorinde::queries::hosted_connections::get_hosted_connection_session()
        .bind(&client, &session_token)
        .opt()
        .await?;

    let Some(session) = session else {
        warn!(
            request_id = %request_id,
            session_token = %redact_token(&session_token),
            "hosted connection popup session lookup failed"
        );
        let html = web_ui::hosted_connections::page::error_page(
            "Connection unavailable".to_string(),
            "This hosted connection session is not valid.".to_string(),
        );
        return Ok(Html(html));
    };

    if session.expired || session.used || session.auth_type != "api_key" {
        warn!(
            request_id = %request_id,
            session_token = %redact_token(&session_token),
            integration_slug = %session.integration_slug,
            expired = session.expired,
            used = session.used,
            auth_type = %session.auth_type,
            "hosted connection popup session rejected"
        );
        let html = web_ui::hosted_connections::page::error_page(
            "Connection unavailable".to_string(),
            "This hosted connection session can no longer be used.".to_string(),
        );
        return Ok(Html(html));
    }

    debug!(
        request_id = %request_id,
        session_token = %redact_token(&session_token),
        integration_slug = %session.integration_slug,
        "hosted connection popup loaded"
    );

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

pub fn public_json_response(payload: Value, status: StatusCode) -> Response {
    let mut response = Response::builder()
        .status(status)
        .header(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=utf-8"),
        )
        .body(axum::body::Body::from(payload.to_string()))
        .unwrap();
    apply_public_api_headers(response.headers_mut());
    response
}

pub fn public_empty_response(status: StatusCode) -> Response {
    let mut response = Response::builder()
        .status(status)
        .body(axum::body::Body::empty())
        .unwrap();
    apply_public_api_headers(response.headers_mut());
    response
}

pub fn apply_public_api_headers(headers: &mut axum::http::HeaderMap) {
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_ORIGIN,
        HeaderValue::from_static("*"),
    );
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_HEADERS,
        HeaderValue::from_static("Authorization, Content-Type"),
    );
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_METHODS,
        HeaderValue::from_static("GET, POST, OPTIONS"),
    );
    headers.insert(
        header::ACCESS_CONTROL_MAX_AGE,
        HeaderValue::from_static("86400"),
    );
}
