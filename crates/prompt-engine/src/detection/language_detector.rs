use shared_types::domain::RequestedLanguage;

use crate::domain::language::DetectedLanguage;

/// Common French function words used as language markers.
const FRENCH_MARKERS: &[&str] = &[
    "le", "la", "les", "un", "une", "des", "du", "de", "et", "ou", "pour", "avec", "dans", "sur",
    "mon", "ma", "mes", "ton", "ta", "moi", "fais", "corrige", "explique", "ecris", "redige",
    "pourquoi", "comment", "qui", "que", "quoi", "est", "sont", "plante",
];

/// Common English function words used as language markers.
const ENGLISH_MARKERS: &[&str] = &[
    "the", "and", "or", "for", "with", "in", "on", "my", "your", "fix", "write", "explain", "why",
    "how", "what", "is", "are", "this", "that", "please", "about", "of", "to",
];

pub fn detect_language(input: &str, requested: RequestedLanguage) -> DetectedLanguage {
    match requested {
        RequestedLanguage::Fr => DetectedLanguage::Fr,
        RequestedLanguage::En => DetectedLanguage::En,
        RequestedLanguage::Auto => auto_detect(input),
    }
}

fn auto_detect(input: &str) -> DetectedLanguage {
    let lower = input.to_lowercase();

    // A French accent anywhere is a strong, position-independent signal.
    if lower.chars().any(is_french_accent) {
        return DetectedLanguage::Fr;
    }

    let mut french = 0_usize;
    let mut english = 0_usize;
    for word in lower
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
    {
        if FRENCH_MARKERS.contains(&word) {
            french += 1;
        }
        if ENGLISH_MARKERS.contains(&word) {
            english += 1;
        }
    }

    match (french, english) {
        (0, 0) => DetectedLanguage::Unknown,
        _ if french >= english => DetectedLanguage::Fr,
        _ => DetectedLanguage::En,
    }
}

fn is_french_accent(c: char) -> bool {
    matches!(
        c,
        'é' | 'è' | 'ê' | 'ë' | 'à' | 'â' | 'ä' | 'î' | 'ï' | 'ô' | 'ö' | 'û' | 'ù' | 'ü' | 'ç'
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_french_from_accents() {
        assert_eq!(
            detect_language("résume ce texte", RequestedLanguage::Auto),
            DetectedLanguage::Fr
        );
    }

    #[test]
    fn detects_french_from_markers_without_accents() {
        // Regression: a verb-first French sentence used to be misread as English
        // because markers were matched with surrounding spaces and missed the
        // first word of the input.
        assert_eq!(
            detect_language(
                "corrige mon code python qui plante",
                RequestedLanguage::Auto
            ),
            DetectedLanguage::Fr
        );
    }
}
