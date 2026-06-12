use axum::{extract::State, Json};
use shared_types::api::{OptimizeRequest, OptimizeResponse};

use crate::{errors::ApiError, state::AppState};

pub async fn optimize(
    State(state): State<AppState>,
    Json(request): Json<OptimizeRequest>,
) -> Result<Json<OptimizeResponse>, ApiError> {
    let response = prompt_engine::optimize_prompt(request, &state.engine_config)?;
    Ok(Json(response))
}
