use http::{HeaderValue, Method};
use tower_http::cors::CorsLayer;

use crate::config::ServerConfig;

pub fn cors_layer(config: &ServerConfig) -> CorsLayer {
    let origins = config
        .allowed_origins
        .iter()
        .filter_map(|origin| HeaderValue::from_str(origin).ok())
        .collect::<Vec<_>>();

    CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([http::header::CONTENT_TYPE])
        .allow_origin(origins)
}
