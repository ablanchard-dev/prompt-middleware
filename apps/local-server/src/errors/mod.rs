use axum::{http::StatusCode, response::IntoResponse, Json};
use prompt_engine::types::EngineError;
use shared_types::errors::{ApiErrorBody, ApiErrorDetail};

#[derive(Debug)]
pub enum ApiError {
    InvalidRequest(String),
    InputTooLarge,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, code, message, retryable) = match self {
            Self::InvalidRequest(message) => {
                (StatusCode::BAD_REQUEST, "INVALID_REQUEST", message, false)
            }
            Self::InputTooLarge => (
                StatusCode::PAYLOAD_TOO_LARGE,
                "INPUT_TOO_LARGE",
                "The prompt is too large.".to_owned(),
                false,
            ),
        };

        let body = ApiErrorBody {
            error: ApiErrorDetail {
                code: code.to_owned(),
                message,
                retryable,
            },
        };

        (status, Json(body)).into_response()
    }
}

impl From<EngineError> for ApiError {
    fn from(value: EngineError) -> Self {
        match value {
            EngineError::EmptyInput => Self::InvalidRequest("raw_user_input is empty.".to_owned()),
            EngineError::InputTooLarge => Self::InputTooLarge,
        }
    }
}
