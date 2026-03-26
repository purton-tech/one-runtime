mod actions;
mod loaders;
mod support;

use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};
use axum_extra::routing::RouterExt;

use crate::mcp::AppState;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/connect.js", get(loaders::loader_sdk))
        .route("/connect/submit.json", post(actions::action_submit_json))
        .route(
            "/api/public/integrations",
            get(actions::action_list_integrations).options(actions::options_public_api),
        )
        .route(
            "/api/public/hosted-connection-sessions",
            post(actions::action_create_session_public).options(actions::options_public_api),
        )
        .route(
            "/api/public/disconnect",
            post(actions::action_disconnect_public).options(actions::options_public_api),
        )
        .typed_get(loaders::loader_popup)
        .typed_get(loaders::loader_tester)
        .typed_post(actions::action_create_session)
        .typed_post(actions::action_submit)
}
