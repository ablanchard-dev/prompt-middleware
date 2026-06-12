use shared_types::api::QualityScore;

#[derive(Debug, Clone, PartialEq)]
pub struct QualityAssessment {
    pub score: QualityScore,
    pub warnings: Vec<String>,
    pub needs_clarification: bool,
    pub clarification_questions: Vec<String>,
}
