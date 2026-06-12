#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromptTemplate {
    pub id: &'static str,
    pub role: &'static str,
    pub task: &'static str,
    pub output_format: &'static str,
}
