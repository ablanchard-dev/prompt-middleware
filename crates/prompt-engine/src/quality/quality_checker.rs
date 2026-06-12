use crate::domain::scoring::QualityAssessment;
use shared_types::api::QualityScore;

pub fn check_quality(raw_input: &str, optimized_prompt: &str) -> QualityAssessment {
    let needs_context = raw_input.len() < 40;
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
