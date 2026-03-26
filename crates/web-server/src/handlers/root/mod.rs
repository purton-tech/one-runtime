mod loaders;

use std::sync::Arc;

use axum::{Router, routing::get};

use crate::mcp::AppState;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/", get(loaders::home))
}
