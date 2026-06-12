use crate::{
    config::EngineConfig,
    domain::{intent::Intent, prompt::PromptTemplate, prompt_domain::PromptDomain},
    templates::registry::{BUSINESS_TEMPLATE, CODE_TEMPLATE, GENERAL_TEMPLATE, LEARNING_TEMPLATE},
};

pub fn select_template(
    domain: PromptDomain,
    _intent: Intent,
    _config: &EngineConfig,
) -> &'static PromptTemplate {
    match domain {
        PromptDomain::Code => &CODE_TEMPLATE,
        PromptDomain::Business => &BUSINESS_TEMPLATE,
        PromptDomain::Learning => &LEARNING_TEMPLATE,
        _ => &GENERAL_TEMPLATE,
    }
}
