use crate::types::EngineError;

pub fn validate_input(input: &str) -> Result<(), EngineError> {
    if input.trim().is_empty() {
        return Err(EngineError::EmptyInput);
    }

    if input.chars().count() > 20_000 {
        return Err(EngineError::InputTooLarge);
    }

    Ok(())
}
