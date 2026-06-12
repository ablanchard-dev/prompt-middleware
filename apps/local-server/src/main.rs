mod config;
mod errors;
mod middleware;
mod routes;
mod state;

use std::net::SocketAddr;

use axum::Router;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{config::ServerConfig, state::AppState};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            "local_server=info,tower_http=info",
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = ServerConfig::default();
    let state = AppState::new();
    let app = Router::new()
        .merge(routes::router())
        .layer(middleware::cors::cors_layer(&config))
        .with_state(state);

    let addr = SocketAddr::from((config.host, config.port));
    tracing::info!(%addr, "starting local server");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("bind local server");
    axum::serve(listener, app)
        .await
        .expect("serve local server");
}
