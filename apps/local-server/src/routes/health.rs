use axum::Json;
use shared_types::api::HealthResponse;

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_owned(),
        service: "prompt-middleware-local-server".to_owned(),
        version: env!("CARGO_PKG_VERSION").to_owned(),
        engine_version: env!("CARGO_PKG_VERSION").to_owned(),
    })
}
