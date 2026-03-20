pub mod auth;
pub mod handler;
pub mod tools;

use std::{path::PathBuf, sync::Arc};

use anyhow::Context;
use clorinde::deadpool_postgres::Pool;
use rmcp::{
    ServerHandler,
    handler::server::router::Router as McpRouter,
    model::{Implementation, InitializeResult, ServerCapabilities},
};
use tool_runtime::{monty_python::RunPython, openapi_actions::OpenApiRegistry};

use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub pool: Pool,
    pub openapi_registry: Arc<OpenApiRegistry>,
    pub run_python: RunPython,
}

impl AppState {
    pub fn new(config: Config, pool: Pool) -> anyhow::Result<Self> {
        let openapi_registry = Arc::new(load_openapi_registry()?);
        let run_python = RunPython::new(openapi_registry.clone());

        Ok(Self {
            config,
            pool,
            openapi_registry,
            run_python,
        })
    }
}

#[derive(Clone)]
pub struct McpServer {
    pub state: AppState,
    pub principal: auth::McpPrincipal,
}

impl McpServer {
    pub fn new(state: AppState, principal: auth::McpPrincipal) -> Self {
        Self { state, principal }
    }

    pub fn service(self) -> McpRouter<Self> {
        McpRouter::new(self).with_tools(tools::router())
    }
}

impl ServerHandler for McpServer {
    fn get_info(&self) -> InitializeResult {
        InitializeResult::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(
                Implementation::new("one-runtime", env!("CARGO_PKG_VERSION"))
                    .with_title("One Runtime MCP")
                    .with_description("One Runtime MCP server for integration discovery and sandboxed execution.")
                    .with_website_url(self.state.config.app_base_url.clone()),
            )
            .with_instructions(
                "Use tool_search to discover integration functions, tool_help to inspect usage, run_python for sandboxed execution, and get_tool_connect_url when a user needs to connect an integration.",
            )
    }
}

fn load_openapi_registry() -> anyhow::Result<OpenApiRegistry> {
    let candidate_dirs = [
        PathBuf::from("crates/one-runtime-com/open-api-specs"),
        PathBuf::from("wireframe/openapi/open-api-specs"),
    ];

    let mut specs = Vec::new();
    for dir in candidate_dirs {
        let loaded = OpenApiRegistry::load_specs_from_dir(&dir)
            .with_context(|| format!("failed to load OpenAPI specs from {}", dir.display()))?;
        specs.extend(loaded);
    }

    Ok(OpenApiRegistry::from_specs(&specs))
}
