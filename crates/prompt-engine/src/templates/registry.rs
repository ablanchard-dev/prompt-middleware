use std::collections::HashMap;

use crate::domain::{prompt::PromptTemplate, prompt_domain::PromptDomain};

/// The built-in templates keyed by domain. Domains without an entry fall back
/// to [`general_template`]. Callers can clone, extend, or replace this map on
/// [`crate::config::EngineConfig`] to customize the engine without code changes.
pub fn builtin_templates() -> HashMap<PromptDomain, PromptTemplate> {
    let mut templates = HashMap::new();
    templates.insert(PromptDomain::Code, code_template());
    templates.insert(PromptDomain::Business, business_template());
    templates.insert(PromptDomain::Learning, learning_template());
    templates
}

pub fn general_template() -> PromptTemplate {
    PromptTemplate {
        id: "general.default.v1".to_owned(),
        role: "Tu es un assistant expert, clair et structure.".to_owned(),
        task: "Transforme la demande utilisateur en prompt precis, contextualise et actionnable."
            .to_owned(),
        output_format:
            "Retourne une reponse structuree avec hypotheses, contraintes et format attendu."
                .to_owned(),
    }
}

fn code_template() -> PromptTemplate {
    PromptTemplate {
        id: "code.default.v1".to_owned(),
        role: "Tu es un ingenieur logiciel senior.".to_owned(),
        task: "Analyse la demande technique et produis un prompt utile pour obtenir une reponse fiable."
            .to_owned(),
        output_format: "Structure la sortie en diagnostic, solution, exemple et tests.".to_owned(),
    }
}

fn business_template() -> PromptTemplate {
    PromptTemplate {
        id: "business.default.v1".to_owned(),
        role: "Tu es un expert en communication professionnelle et business.".to_owned(),
        task: "Clarifie l'objectif, le contexte, le ton et le resultat attendu.".to_owned(),
        output_format: "Retourne un livrable pret a utiliser avec variantes si utile.".to_owned(),
    }
}

fn learning_template() -> PromptTemplate {
    PromptTemplate {
        id: "learning.default.v1".to_owned(),
        role: "Tu es un pedagogue expert.".to_owned(),
        task: "Construis un plan d'apprentissage progressif et actionnable.".to_owned(),
        output_format:
            "Retourne objectifs, parcours, exercices, ressources et criteres de progression."
                .to_owned(),
    }
}
