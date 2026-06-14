use std::collections::HashMap;

use crate::domain::{prompt::PromptTemplate, prompt_domain::PromptDomain};
use crate::templates::registry::{builtin_templates, general_template};

/// Engine configuration. Everything the engine needs is passed in here, so the
/// engine stays pure (no IO, no globals): callers build a config in memory and
/// can override the maximum input size or the templates used per domain.
#[derive(Debug, Clone)]
pub struct EngineConfig {
    pub max_input_chars: usize,
    pub templates: HashMap<PromptDomain, PromptTemplate>,
    pub fallback_template: PromptTemplate,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            max_input_chars: 20_000,
            templates: builtin_templates(),
            fallback_template: general_template(),
        }
    }
}
