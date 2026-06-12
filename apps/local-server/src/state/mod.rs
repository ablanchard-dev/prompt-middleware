use prompt_engine::config::EngineConfig;

#[derive(Debug, Clone)]
pub struct AppState {
    pub engine_config: EngineConfig,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            engine_config: EngineConfig::default(),
        }
    }
}
