mod actions;
mod loaders;

use std::sync::Arc;

use axum::Router;
use axum_extra::routing::RouterExt;

use crate::mcp::AppState;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .typed_get(loaders::loader)
        .typed_get(loaders::loader_new)
        .typed_get(loaders::loader_edit)
        .typed_post(actions::action_upsert)
        .typed_post(actions::action_delete)
}
