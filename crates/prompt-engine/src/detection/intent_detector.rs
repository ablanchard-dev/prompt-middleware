use crate::domain::intent::Intent;

pub fn detect_intent(input: &str) -> Intent {
    let lower = input.to_lowercase();

    if contains_any(&lower, &["corrige", "fix", "debug", "erreur"]) {
        Intent::Corriger
    } else if contains_any(&lower, &["code", "coder", "implemente", "function"]) {
        Intent::Coder
    } else if contains_any(&lower, &["plan", "roadmap", "organise", "planning"]) {
        Intent::Planifier
    } else if contains_any(&lower, &["compare", "versus", "difference"]) {
        Intent::Comparer
    } else if contains_any(&lower, &["resume", "summarize", "synthese"]) {
        Intent::Resumer
    } else if contains_any(&lower, &["audit", "securite", "vulnerabilite"]) {
        Intent::Auditer
    } else if contains_any(&lower, &["redige", "mail", "email", "article"]) {
        Intent::Rediger
    } else if contains_any(&lower, &["transforme", "reformule", "convertis"]) {
        Intent::Transformer
    } else if contains_any(&lower, &["analyse", "diagnostic"]) {
        Intent::Analyser
    } else {
        Intent::Expliquer
    }
}

fn contains_any(input: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| input.contains(needle))
}
