use axum::{
    routing::{get, post},
    Router,
};

use crate::state::AppState;

pub mod health;
pub mod optimize;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health::health))
        .route("/optimize", post(optimize::optimize))
}
