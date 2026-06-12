use shared_types::domain::RequestedLanguage;

use crate::domain::language::DetectedLanguage;

pub fn detect_language(input: &str, requested: RequestedLanguage) -> DetectedLanguage {
    match requested {
        RequestedLanguage::Fr => DetectedLanguage::Fr,
        RequestedLanguage::En => DetectedLanguage::En,
        RequestedLanguage::Auto => {
            let lower = input.to_lowercase();
            if [
                " le ",
                " la ",
                " les ",
                " une ",
                " pour ",
                " fais ",
                " corrige ",
            ]
            .iter()
            .any(|token| lower.contains(token))
            {
                DetectedLanguage::Fr
            } else if lower.is_ascii() {
                DetectedLanguage::En
            } else {
                DetectedLanguage::Unknown
            }
        }
    }
}
