use crate::types::EngineError;

/// Reject empty/whitespace-only input and input longer than `max_chars`
/// characters. The limit comes from `EngineConfig::max_input_chars`.
pub fn validate_input(input: &str, max_chars: usize) -> Result<(), EngineError> {
    if input.trim().is_empty() {
        return Err(EngineError::EmptyInput);
    }

    if input.chars().count() > max_chars {
        return Err(EngineError::InputTooLarge);
    }

    Ok(())
}
