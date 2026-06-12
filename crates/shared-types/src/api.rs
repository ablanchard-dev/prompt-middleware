use serde::{Deserialize, Serialize};

use crate::domain::{DetailLevel, OptimizeMode, RequestedLanguage, TargetPlatform};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserPreferences {
    pub tone: Option<String>,
    pub detail_level: DetailLevel,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OptimizeRequest {
    pub raw_user_input: String,
    pub target_platform: TargetPlatform,
    pub language: RequestedLanguage,
    pub mode: OptimizeMode,
    pub user_preferences: UserPreferences,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QualityScore {
    pub clarity: f32,
    pub context: f32,
    pub constraints: f32,
    pub format: f32,
    pub overall: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OptimizeResponse {
    pub optimized_prompt: String,
    pub detected_language: String,
    pub detected_domain: String,
    pub detected_intent: String,
    pub confidence: f32,
    pub quality_score: QualityScore,
    pub warnings: Vec<String>,
    pub needs_clarification: bool,
    pub clarification_questions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub version: String,
    pub engine_version: String,
}
