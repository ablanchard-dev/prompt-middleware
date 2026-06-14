use crate::{
    config::EngineConfig,
    domain::{intent::Intent, prompt::PromptTemplate, prompt_domain::PromptDomain},
};

/// Pick the template for a domain, falling back to the configured general
/// template when the domain has no dedicated entry.
pub fn select_template(
    domain: PromptDomain,
    _intent: Intent,
    config: &EngineConfig,
) -> &PromptTemplate {
    config
        .templates
        .get(&domain)
        .unwrap_or(&config.fallback_template)
}
