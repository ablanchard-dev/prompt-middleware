//! Tests d'intégration du moteur pur : détection langue/intention/domaine + validation.
//! Basés sur la logique réelle des heuristiques (pas de mocks).

use prompt_engine::classification::domain_classifier::classify_domain;
use prompt_engine::detection::intent_detector::detect_intent;
use prompt_engine::detection::language_detector::detect_language;
use prompt_engine::domain::intent::Intent;
use prompt_engine::domain::language::DetectedLanguage;
use prompt_engine::domain::prompt_domain::PromptDomain;
use prompt_engine::safety::input::validate_input;
use shared_types::domain::RequestedLanguage;

// --- detect_language --------------------------------------------------------

#[test]
fn requested_language_overrides_detection() {
    assert_eq!(
        detect_language("anything at all", RequestedLanguage::Fr),
        DetectedLanguage::Fr
    );
    assert_eq!(
        detect_language("peu importe le texte", RequestedLanguage::En),
        DetectedLanguage::En
    );
}

#[test]
fn auto_detects_french_from_tokens() {
    assert_eq!(
        detect_language("fais le resume pour moi", RequestedLanguage::Auto),
        DetectedLanguage::Fr
    );
}

#[test]
fn auto_detects_english_for_plain_ascii() {
    assert_eq!(
        detect_language("write a function please", RequestedLanguage::Auto),
        DetectedLanguage::En
    );
}

#[test]
fn auto_unknown_for_non_ascii_non_french() {
    assert_eq!(
        detect_language("これはテキストです", RequestedLanguage::Auto),
        DetectedLanguage::Unknown
    );
}

#[test]
fn detected_language_as_str() {
    assert_eq!(DetectedLanguage::Fr.as_str(), "fr");
    assert_eq!(DetectedLanguage::En.as_str(), "en");
    assert_eq!(DetectedLanguage::Unknown.as_str(), "unknown");
}

// --- detect_intent ----------------------------------------------------------

#[test]
fn intent_corriger_from_fix_and_corrige() {
    assert_eq!(detect_intent("corrige cette erreur"), Intent::Corriger);
    assert_eq!(detect_intent("please fix this"), Intent::Corriger);
}

#[test]
fn intent_coder() {
    assert_eq!(detect_intent("write some code"), Intent::Coder);
}

#[test]
fn intent_planifier() {
    assert_eq!(detect_intent("make a plan for the launch"), Intent::Planifier);
}

#[test]
fn intent_comparer() {
    assert_eq!(detect_intent("compare A and B"), Intent::Comparer);
}

#[test]
fn intent_resumer() {
    assert_eq!(detect_intent("summarize this document"), Intent::Resumer);
}

#[test]
fn intent_auditer() {
    assert_eq!(detect_intent("audit my contract"), Intent::Auditer);
}

#[test]
fn intent_rediger() {
    assert_eq!(detect_intent("redige un email pro"), Intent::Rediger);
}

#[test]
fn intent_transformer() {
    assert_eq!(detect_intent("transforme ce texte en liste"), Intent::Transformer);
}

#[test]
fn intent_analyser() {
    assert_eq!(detect_intent("analyse ces chiffres"), Intent::Analyser);
}

#[test]
fn intent_defaults_to_expliquer() {
    assert_eq!(detect_intent("hello there friend"), Intent::Expliquer);
}

#[test]
fn intent_priority_fix_before_code() {
    // "fix" est testé avant "code" -> Corriger gagne
    assert_eq!(detect_intent("fix this code"), Intent::Corriger);
}

// --- classify_domain --------------------------------------------------------

#[test]
fn domain_code() {
    assert_eq!(classify_domain("write rust code"), PromptDomain::Code);
}

#[test]
fn domain_learning() {
    assert_eq!(classify_domain("je veux apprendre le piano"), PromptDomain::Learning);
}

#[test]
fn domain_business() {
    assert_eq!(classify_domain("envoie un mail au client"), PromptDomain::Business);
}

#[test]
fn domain_cybersecurity() {
    assert_eq!(classify_domain("audit owasp securite"), PromptDomain::Cybersecurity);
}

#[test]
fn domain_finance_trading() {
    assert_eq!(classify_domain("strategie de trading"), PromptDomain::FinanceTrading);
}

#[test]
fn domain_hr() {
    assert_eq!(classify_domain("entretien de recrutement"), PromptDomain::Hr);
}

#[test]
fn domain_general_fallback() {
    assert_eq!(classify_domain("bonjour comment vas tu"), PromptDomain::General);
}

#[test]
fn domain_as_str_values() {
    assert_eq!(PromptDomain::Code.as_str(), "code");
    assert_eq!(PromptDomain::Learning.as_str(), "apprentissage");
    assert_eq!(PromptDomain::FinanceTrading.as_str(), "finance_trading");
}

// --- validate_input ---------------------------------------------------------

#[test]
fn validate_rejects_empty() {
    assert!(validate_input("").is_err());
}

#[test]
fn validate_rejects_whitespace_only() {
    assert!(validate_input("   \n\t  ").is_err());
}

#[test]
fn validate_accepts_normal_input() {
    assert!(validate_input("optimise mon prompt s'il te plait").is_ok());
}

#[test]
fn validate_rejects_too_large() {
    let big = "a".repeat(20_001);
    assert!(validate_input(&big).is_err());
}

#[test]
fn validate_accepts_at_size_limit() {
    let at_limit = "a".repeat(20_000);
    assert!(validate_input(&at_limit).is_ok());
}
