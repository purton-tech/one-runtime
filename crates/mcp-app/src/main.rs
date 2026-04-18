mod app;
mod handler;
mod protocol;
mod widget;

use std::net::SocketAddr;

use app::AppState;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    init_tracing();

    let app = app::router(AppState);
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
