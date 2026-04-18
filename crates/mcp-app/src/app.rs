use std::sync::Arc;

use axum::{Router, routing::post};

use crate::handler::mcp_handler;

#[derive(Clone, Default)]
pub struct AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/mcp", post(mcp_handler))
        .with_state(Arc::new(state))
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use serde_json::{Value, json};
    use tower::util::ServiceExt;

    use super::{AppState, router};

    #[tokio::test]
    async fn tools_call_returns_structured_content() {
        let response = router(AppState)
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
    async fn resources_read_returns_embedded_html() {
        let response = router(AppState)
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

        assert_eq!(html, crate::widget::hello_html());
        assert!(html.contains("Waiting for tool result..."));
    }
}
