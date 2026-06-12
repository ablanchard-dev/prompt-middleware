use crate::domain::prompt::PromptTemplate;

pub fn render_template(
    template: &PromptTemplate,
    raw_user_input: &str,
    tone: &str,
    detail_level: &str,
) -> String {
    format!(
        "{role}\n\nDemande utilisateur :\n{raw_user_input}\n\nObjectif :\n{task}\n\nContraintes :\n- Ton souhaite : {tone}\n- Niveau de detail : {detail_level}\n- Ne pas inventer d'informations absentes.\n- Signaler les informations manquantes.\n\nFormat attendu :\n{output_format}\n",
        role = template.role,
        task = template.task,
        output_format = template.output_format,
    )
}
