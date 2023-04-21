pub mod activity_pub;
pub mod config;
pub mod webfinger;

use anyhow::anyhow;
use axum::routing::get;
use axum::Router;
use dotenvy::dotenv;
use std::net::SocketAddr;
use std::sync::Arc;
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

    let config = Arc::new(config::load_config()?);

    let well_known_router = Router::new().route("/webfinger", get(webfinger::handle_webfinger));

    /*
    let activity_pub_router = Router::new()
        .route("/@!:id/outbox", get(activity_pub::handle_actor_get))
        .route("/@:name/outbox", get(activity_pub::handle_actor_get))
        .route("/@!:id", get(activity_pub::handle_actor_get))
        .route("/@:name", get(activity_pub::handle_actor_get));
    */
    let app = Router::new()
        .nest("/.well-known", well_known_router)
        //.nest("/", activity_pub_router)
        .with_state(config.clone())
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
