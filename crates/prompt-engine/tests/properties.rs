//! Property-based tests: instead of fixed inputs, these assert invariants that
//! must hold for *any* input. They guard against panics and out-of-range
//! scores that example-based tests would miss.

use proptest::prelude::*;

use prompt_engine::config::EngineConfig;
use prompt_engine::optimize_prompt;
use shared_types::api::{OptimizeRequest, UserPreferences};
use shared_types::domain::{DetailLevel, OptimizeMode, RequestedLanguage, TargetPlatform};

fn request(input: String) -> OptimizeRequest {
    OptimizeRequest {
        raw_user_input: input,
        target_platform: TargetPlatform::Chatgpt,
        language: RequestedLanguage::Auto,
        mode: OptimizeMode::Preview,
        user_preferences: UserPreferences {
            tone: None,
            detail_level: DetailLevel::Normal,
        },
    }
}

proptest! {
    /// Any non-empty, reasonably sized input is accepted, never panics, and
    /// yields a non-empty prompt with a known language tag and in-range scores.
    #[test]
    fn non_empty_input_always_optimizes_cleanly(
        input in "[a-zA-Z0-9 àâéèêëîïôûùç,.?!'-]{1,300}"
    ) {
        prop_assume!(!input.trim().is_empty());

        let response = optimize_prompt(request(input), &EngineConfig::default()).unwrap();

        prop_assert!(!response.optimized_prompt.is_empty());
        prop_assert!(["fr", "en", "unknown"].contains(&response.detected_language.as_str()));

        let q = &response.quality_score;
        for score in [q.clarity, q.context, q.constraints, q.format, q.overall] {
            prop_assert!((0.0..=1.0).contains(&score), "score out of range: {score}");
        }
    }

    /// Oversized input is rejected with an error, not a panic.
    #[test]
    fn oversized_input_is_rejected(extra in 1usize..200) {
        let oversized = "a".repeat(20_000 + extra);
        prop_assert!(optimize_prompt(request(oversized), &EngineConfig::default()).is_err());
    }
}
