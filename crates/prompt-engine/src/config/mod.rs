#[derive(Debug, Clone)]
pub struct EngineConfig {
    pub max_input_chars: usize,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            max_input_chars: 20_000,
        }
    }
}
