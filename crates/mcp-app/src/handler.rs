use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse};
use serde_json::{Value, json};
use tracing::{debug, warn};

use crate::{
    app::AppState,
    protocol::{JsonRpcRequest, JsonRpcResponse, err, ok},
    widget::{HELLO_WIDGET_URI, hello_html},
};

pub async fn mcp_handler(
    State(_state): State<Arc<AppState>>,
    axum::Json(req): axum::Json<JsonRpcRequest>,
) -> impl IntoResponse {
    debug!(
        method = %req.method,
        id = ?req.id,
        params = %req.params,
        "received MCP request"
    );

    if req.jsonrpc != "2.0" {
        warn!(
            jsonrpc = %req.jsonrpc,
            id = ?req.id,
            "rejecting request with invalid jsonrpc version"
        );
        return (
            StatusCode::BAD_REQUEST,
            axum::Json(err(req.id, -32600, "invalid jsonrpc version")),
        );
    }

    let response = match req.method.as_str() {
        "initialize" => handle_initialize(req.id),
        "notifications/initialized" => handle_initialized(req.id),
        "tools/list" => handle_tools_list(req.id),
        "resources/read" => handle_resources_read(req.id, &req.params),
        "tools/call" => handle_tools_call(req.id, &req.params),
        _ => {
            warn!(method = %req.method, "method not found");
            err(req.id, -32601, "method not found")
        }
    };

    debug!(
        method = %req.method,
        has_error = response.error.is_some(),
        "sending MCP response"
    );

    (StatusCode::OK, axum::Json(response))
}

fn handle_initialize(id: Option<Value>) -> JsonRpcResponse {
    debug!("handling initialize");
    ok(
        id,
        json!({
            "protocolVersion": "2025-11-25",
            "capabilities": {
                "tools": {},
                "resources": {}
            },
            "serverInfo": {
                "name": "hello-mcp-app",
                "version": "0.1.0"
            }
        }),
    )
}

fn handle_initialized(id: Option<Value>) -> JsonRpcResponse {
    debug!("handling initialized notification");
    ok(id, json!({}))
}

fn handle_tools_list(id: Option<Value>) -> JsonRpcResponse {
    debug!("listing tools");
    ok(
        id,
        json!({
            "tools": [
                {
                    "name": "show_hello_app",
                    "title": "Show Hello App",
                    "description": "Open a tiny app in the iframe and greet the supplied name.",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "name": {
                                "type": "string",
                                "description": "The person to greet"
                            }
                        },
                        "required": ["name"],
                        "additionalProperties": false
                    },
                    "_meta": {
                        "ui": {
                            "resourceUri": HELLO_WIDGET_URI
                        },
                        "openai/outputTemplate": HELLO_WIDGET_URI
                    }
                }
            ]
        }),
    )
}

fn handle_resources_read(id: Option<Value>, params: &Value) -> JsonRpcResponse {
    match requested_uri(params) {
        Some(HELLO_WIDGET_URI) => {
            debug!(uri = HELLO_WIDGET_URI, "serving resource");
            ok(
                id,
                json!({
                    "contents": [
                        {
                            "uri": HELLO_WIDGET_URI,
                            "mimeType": "text/html",
                            "text": hello_html()
                        }
                    ]
                }),
            )
        }
        _ => {
            warn!(params = %params, "resource not found");
            err(id, -32002, "resource not found")
        }
    }
}

fn handle_tools_call(id: Option<Value>, params: &Value) -> JsonRpcResponse {
    match requested_tool_name(params) {
        Some("show_hello_app") => {
            let who = requested_name_argument(params).unwrap_or("world");
            debug!(tool = "show_hello_app", who, "handling tool call");
            ok(
                id,
                json!({
                    "content": [
                        {
                            "type": "text",
                            "text": format!("Opened the hello app for {who}.")
                        }
                    ],
                    "structuredContent": {
                        "message": format!("Hello, {who}!"),
                        "subtitle": "This came from your Rust MCP server."
                    }
                }),
            )
        }
        _ => {
            warn!(params = %params, "unknown tool requested");
            err(id, -32601, "unknown tool")
        }
    }
}

fn requested_uri(params: &Value) -> Option<&str> {
    params.get("uri").and_then(Value::as_str)
}

fn requested_tool_name(params: &Value) -> Option<&str> {
    params.get("name").and_then(Value::as_str)
}

fn requested_name_argument(params: &Value) -> Option<&str> {
    params
        .get("arguments")
        .and_then(|arguments| arguments.get("name"))
        .and_then(Value::as_str)
}
