use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Request, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use rmcp::transport::{
    StreamableHttpServerConfig, StreamableHttpService,
    streamable_http_server::session::never::NeverSessionManager,
};
use serde_json::json;

use super::{AppState, McpServer, auth};

pub async fn handle_mcp(State(state): State<Arc<AppState>>, request: Request<Body>) -> Response {
    let principal = match auth::authenticate(state.as_ref(), request.headers()).await {
        Ok(principal) => principal,
        Err(err) => return jsonrpc_error(StatusCode::UNAUTHORIZED, err.message),
    };

    let service = StreamableHttpService::new(
        {
            let state = state.as_ref().clone();
            move || Ok(McpServer::new(state.clone(), principal.clone()).service())
        },
        Arc::new(NeverSessionManager::default()),
        StreamableHttpServerConfig {
            stateful_mode: false,
            json_response: true,
            ..Default::default()
        },
    );

    service.handle(request).await.into_response()
}

fn jsonrpc_error(status: StatusCode, message: impl Into<String>) -> Response {
    let body = axum::Json(json!({
        "jsonrpc": "2.0",
        "id": null,
        "error": {
            "code": -32001,
            "message": message.into(),
        }
    }));

    (status, body).into_response()
}
