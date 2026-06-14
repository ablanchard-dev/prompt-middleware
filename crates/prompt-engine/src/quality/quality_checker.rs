use crate::domain::scoring::QualityAssessment;
use shared_types::api::QualityScore;

pub fn check_quality(raw_input: &str, optimized_prompt: &str) -> QualityAssessment {
    // Count characters, not bytes, so accented input is measured consistently.
    let needs_context = raw_input.trim().chars().count() < 40;
    let warnings = if needs_context {
        vec!["La demande initiale contient peu de contexte.".to_owned()]
    } else {
        Vec::new()
    };

    QualityAssessment {
        score: QualityScore {
            clarity: 0.82,
            context: if needs_context { 0.55 } else { 0.78 },
            constraints: 0.80,
            format: if optimized_prompt.contains("Format attendu") {
                0.90
            } else {
                0.50
            },
            overall: if needs_context { 0.74 } else { 0.82 },
        },
        warnings,
        needs_clarification: needs_context,
        clarification_questions: if needs_context {
            vec!["Quel contexte ou objectif final doit etre pris en compte ?".to_owned()]
        } else {
            Vec::new()
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flags_short_input_as_needing_context() {
        let assessment = check_quality("plante", "prompt");
        assert!(assessment.needs_clarification);
        assert_eq!(assessment.warnings.len(), 1);
        assert!(!assessment.clarification_questions.is_empty());
    }

    #[test]
    fn does_not_flag_a_detailed_input() {
        let raw = "Explique comment fonctionne l'ownership en Rust avec des exemples concrets";
        let assessment = check_quality(raw, "prompt");
        assert!(!assessment.needs_clarification);
        assert!(assessment.warnings.is_empty());
    }

    #[test]
    fn rewards_a_format_section_in_the_output() {
        let with_format = check_quality("court", "Format attendu : structure");
        let without_format = check_quality("court", "aucune section de format");
        assert!(with_format.score.format > without_format.score.format);
    }
}
