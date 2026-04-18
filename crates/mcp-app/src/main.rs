use std::{net::SocketAddr, sync::Arc};

use axum::{Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Clone, Default)]
struct AppState;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    init_tracing();

    let app = app(Arc::new(AppState));
    let addr: SocketAddr = "0.0.0.0:8080".parse().expect("invalid listen address");

    println!("listening on http://{addr}/mcp");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind TCP listener");
    axum::serve(listener, app)
        .await
        .expect("failed to serve MCP app");
}

fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with_target(false)
        .try_init();
}

fn app(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/mcp", post(mcp_handler))
        .with_state(state)
}

#[derive(Debug, Deserialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    #[serde(default)]
    params: Value,
}

#[derive(Debug, Serialize)]
struct JsonRpcResponse {
    jsonrpc: &'static str,
    id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<Value>,
}

async fn mcp_handler(
    State(_state): State<Arc<AppState>>,
    axum::Json(req): axum::Json<JsonRpcRequest>,
) -> impl IntoResponse {
    if req.jsonrpc != "2.0" {
        return (
            StatusCode::BAD_REQUEST,
            axum::Json(err(req.id, -32600, "invalid jsonrpc version")),
        );
    }

    let response = match req.method.as_str() {
        "initialize" => ok(
            req.id,
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
        ),
        "notifications/initialized" => ok(req.id, json!({})),
        "tools/list" => ok(
            req.id,
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
                                "resourceUri": "ui://app/hello.html"
                            },
                            "openai/outputTemplate": "ui://app/hello.html"
                        }
                    }
                ]
            }),
        ),
        "resources/read" => match requested_uri(&req.params) {
            Some("ui://app/hello.html") => ok(
                req.id,
                json!({
                    "contents": [
                        {
                            "uri": "ui://app/hello.html",
                            "mimeType": "text/html",
                            "text": hello_html()
                        }
                    ]
                }),
            ),
            _ => err(req.id, -32002, "resource not found"),
        },
        "tools/call" => match requested_tool_name(&req.params) {
            Some("show_hello_app") => {
                let who = requested_name_argument(&req.params).unwrap_or("world");
                ok(
                    req.id,
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
            _ => err(req.id, -32601, "unknown tool"),
        },
        _ => err(req.id, -32601, "method not found"),
    };

    (StatusCode::OK, axum::Json(response))
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

fn ok(id: Option<Value>, result: Value) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: "2.0",
        id,
        result: Some(result),
        error: None,
    }
}

fn err(id: Option<Value>, code: i64, message: &str) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: "2.0",
        id,
        result: None,
        error: Some(json!({
            "code": code,
            "message": message,
        })),
    }
}

fn hello_html() -> String {
    r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width,initial-scale=1" />
    <title>Hello App</title>
    <style>
      :root { color-scheme: light dark; }
      body {
        font-family: ui-sans-serif, system-ui, sans-serif;
        margin: 0;
        padding: 16px;
      }
      .card {
        border: 1px solid #8884;
        border-radius: 14px;
        padding: 16px;
      }
      h1 { margin: 0 0 8px; font-size: 18px; }
      p { margin: 0; opacity: 0.85; }
    </style>
  </head>
  <body>
    <div class="card">
      <h1 id="message">Hello</h1>
      <p id="subtitle">Waiting for tool result...</p>
    </div>

    <script>
      const message = document.getElementById("message");
      const subtitle = document.getElementById("subtitle");

      function applyPayload(payload) {
        const sc = payload?.structuredContent ?? payload ?? {};
        message.textContent = sc.message ?? "Hello";
        subtitle.textContent = sc.subtitle ?? "No subtitle";
      }

      window.addEventListener("message", (event) => {
        const msg = event.data;
        if (!msg || msg.jsonrpc !== "2.0") return;

        if (msg.method === "ui/notifications/tool-result") {
          applyPayload(msg.params);
        }
      });
    </script>
  </body>
</html>"#
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::app;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use serde_json::{Value, json};
    use std::sync::Arc;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn tools_call_returns_structured_content() {
        let response = app(Arc::new(super::AppState))
            .oneshot(
                Request::post("/mcp")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        json!({
                            "jsonrpc": "2.0",
                            "id": 1,
                            "method": "tools/call",
                            "params": {
                                "name": "show_hello_app",
                                "arguments": {
                                    "name": "Alice"
                                }
                            }
                        })
                        .to_string(),
                    ))
                    .expect("request should build"),
            )
            .await
            .expect("request should succeed");

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("body should read");
        let payload: Value = serde_json::from_slice(&body).expect("JSON response should parse");

        assert_eq!(
            payload["result"]["structuredContent"]["message"],
            json!("Hello, Alice!")
        );
    }

    #[tokio::test]
    async fn resources_read_returns_html() {
        let response = app(Arc::new(super::AppState))
            .oneshot(
                Request::post("/mcp")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        json!({
                            "jsonrpc": "2.0",
                            "id": 2,
                            "method": "resources/read",
                            "params": {
                                "uri": "ui://app/hello.html"
                            }
                        })
                        .to_string(),
                    ))
                    .expect("request should build"),
            )
            .await
            .expect("request should succeed");

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("body should read");
        let payload: Value = serde_json::from_slice(&body).expect("JSON response should parse");
        let html = payload["result"]["contents"][0]["text"]
            .as_str()
            .expect("HTML should be a string");

        assert!(html.contains("Waiting for tool result..."));
    }
}
