use shared_types::api::OptimizeRequest;

use crate::{
    domain::{
        intent::Intent, language::DetectedLanguage, prompt::PromptTemplate,
        prompt_domain::PromptDomain,
    },
    templates::renderer::render_template,
};

pub fn build_prompt(
    request: &OptimizeRequest,
    _language: DetectedLanguage,
    _domain: PromptDomain,
    _intent: Intent,
    template: &PromptTemplate,
) -> String {
    let tone = request
        .user_preferences
        .tone
        .as_deref()
        .unwrap_or("neutral");
    let detail_level = format!("{:?}", request.user_preferences.detail_level).to_lowercase();
    render_template(template, &request.raw_user_input, tone, &detail_level)
}
