use crate::domain::prompt_domain::PromptDomain;

pub fn classify_domain(input: &str) -> PromptDomain {
    let lower = input.to_lowercase();

    if contains_any(&lower, &["apprendre", "cours", "formation", "pedagog"]) {
        PromptDomain::Learning
    } else if contains_any(&lower, &["rust", "typescript", "api", "bug", "code"]) {
        PromptDomain::Code
    } else if contains_any(&lower, &["client", "vente", "business", "email", "mail"]) {
        PromptDomain::Business
    } else if contains_any(&lower, &["securite", "vulnerabilite", "owasp", "audit"]) {
        PromptDomain::Cybersecurity
    } else if contains_any(&lower, &["trading", "finance", "investissement"]) {
        PromptDomain::FinanceTrading
    } else if contains_any(&lower, &["recrutement", "rh", "cv", "entretien"]) {
        PromptDomain::Hr
    } else if contains_any(&lower, &["architecture", "technique", "projet"]) {
        PromptDomain::TechnicalProject
    } else if contains_any(&lower, &["redaction", "article", "texte"]) {
        PromptDomain::Redaction
    } else {
        PromptDomain::General
    }
}

fn contains_any(input: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| input.contains(needle))
}
