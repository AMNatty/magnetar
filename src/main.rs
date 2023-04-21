pub mod config;
pub mod nodeinfo;
pub mod util;
pub mod webfinger;

use crate::nodeinfo::{handle_nodeinfo, handle_nodeinfo_20, handle_nodeinfo_21};
use anyhow::anyhow;
use axum::routing::get;
use axum::Router;
use dotenvy::dotenv;
use magnetar_calckey_model::{CalckeyModel, ConnectorConfig};
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

    let config = &*Box::leak::<'static>(Box::new(config::load_config()?));

    let db = CalckeyModel::new(ConnectorConfig {
        url: config.data.database_url.clone(),
    })
    .await?;

    let well_known_router = Router::new()
        .route(
            "/webfinger",
            get(webfinger::handle_webfinger).with_state((config, db)),
        )
        .route("/nodeinfo", get(handle_nodeinfo));

    let nodeinfo_router = Router::new()
        .with_state(config)
        .route("/2.0", get(handle_nodeinfo_20))
        .route("/2.1", get(handle_nodeinfo_21));

    let app = Router::new()
        .nest("/.well-known", well_known_router)
        .nest("/nodeinfo", nodeinfo_router)
        .with_state(config)
        .layer(
            CorsLayer::new()
                .allow_headers(Any)
                .allow_methods(Any)
                .allow_origin(Any),
        )
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from((config.networking.bind_addr, config.networking.port));
    info!("Serving on: {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .map_err(|e| anyhow!("Error running server: {}", e))
}
