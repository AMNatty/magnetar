pub mod webfinger;

use anyhow::{anyhow, Context};
use axum::routing::get;
use axum::Router;
use dotenvy::dotenv;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::fmt()
        .with_env_filter(filter_layer)
        .with_test_writer()
        .init();

    let port: u16 = std::env::var("SERVER_PORT")
        .unwrap_or_else(|_| "4939".to_string())
        .parse()
        .context("SERVER_PORT not a number")?;

    let well_known_router = Router::new().route("/webfinger", get(webfinger::handle_webfinger));

    let app = Router::new()
        .nest("/.well-known", well_known_router)
        .layer(
            CorsLayer::new()
                .allow_headers(Any)
                .allow_methods(Any)
                .allow_origin(Any),
        )
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving on: {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .map_err(|e| anyhow!("Error running server: {}", e))
}
