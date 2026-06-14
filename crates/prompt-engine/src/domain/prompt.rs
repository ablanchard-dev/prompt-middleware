/// A prompt template: the building blocks rendered into the final optimized
/// prompt. Templates are plain data so they can be supplied or overridden
/// through [`crate::config::EngineConfig`] without touching the engine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromptTemplate {
    pub id: String,
    pub role: String,
    pub task: String,
    pub output_format: String,
}
