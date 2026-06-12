use thiserror::Error;

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("input is empty")]
    EmptyInput,
    #[error("input is too large")]
    InputTooLarge,
}
