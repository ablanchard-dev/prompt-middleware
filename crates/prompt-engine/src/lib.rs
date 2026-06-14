pub mod builder;
pub mod classification;
pub mod config;
pub mod detection;
pub mod domain;
pub mod quality;
pub mod safety;
pub mod templates;
pub mod types;

use builder::build_prompt;
use classification::domain_classifier::classify_domain;
use config::EngineConfig;
use detection::{intent_detector::detect_intent, language_detector::detect_language};
use quality::quality_checker::check_quality;
use shared_types::api::{OptimizeRequest, OptimizeResponse};
use templates::selector::select_template;
use types::EngineError;

pub fn optimize_prompt(
    request: OptimizeRequest,
    config: &EngineConfig,
) -> Result<OptimizeResponse, EngineError> {
    safety::input::validate_input(&request.raw_user_input)?;

    let detected_language = detect_language(&request.raw_user_input, request.language);
    let detected_intent = detect_intent(&request.raw_user_input);
    let detected_domain = classify_domain(&request.raw_user_input);
    let template = select_template(detected_domain, detected_intent, config);
    let optimized_prompt = build_prompt(
        &request,
        detected_language,
        detected_domain,
        detected_intent,
        template,
    );
    let quality = check_quality(&request.raw_user_input, &optimized_prompt);

    Ok(OptimizeResponse {
        optimized_prompt,
        detected_language: detected_language.as_str().to_owned(),
        detected_domain: detected_domain.as_str().to_owned(),
        detected_intent: detected_intent.as_str().to_owned(),
        confidence: 0.78,
        quality_score: quality.score,
        warnings: quality.warnings,
        needs_clarification: quality.needs_clarification,
        clarification_questions: quality.clarification_questions,
    })
}

#[cfg(test)]
mod tests {
    use shared_types::{
        api::{OptimizeRequest, UserPreferences},
        domain::{DetailLevel, OptimizeMode, RequestedLanguage, TargetPlatform},
    };

    use super::{config::EngineConfig, optimize_prompt};

    #[test]
    fn optimizes_a_simple_french_learning_prompt() {
        let request = OptimizeRequest {
            raw_user_input: "Fais moi un plan pour apprendre Rust".to_owned(),
            target_platform: TargetPlatform::Chatgpt,
            language: RequestedLanguage::Fr,
            mode: OptimizeMode::Preview,
            user_preferences: UserPreferences {
                tone: Some("pedagogique".to_owned()),
                detail_level: DetailLevel::Detailed,
            },
        };

        let response = optimize_prompt(request, &EngineConfig::default()).unwrap();

        assert_eq!(response.detected_language, "fr");
        assert_eq!(response.detected_domain, "apprentissage");
        assert_eq!(response.detected_intent, "planifier");
        assert!(response.optimized_prompt.contains("pedagogue expert"));
        assert!(response.optimized_prompt.contains("Demande utilisateur"));
    }

    #[test]
    fn optimizes_an_english_code_prompt_in_auto_mode() {
        let request = OptimizeRequest {
            raw_user_input: "fix the bug in my python code".to_owned(),
            target_platform: TargetPlatform::Chatgpt,
            language: RequestedLanguage::Auto,
            mode: OptimizeMode::Preview,
            user_preferences: UserPreferences {
                tone: None,
                detail_level: DetailLevel::Normal,
            },
        };

        let response = optimize_prompt(request, &EngineConfig::default()).unwrap();

        assert_eq!(response.detected_language, "en");
        assert_eq!(response.detected_domain, "code");
        assert_eq!(response.detected_intent, "corriger");
    }
}
