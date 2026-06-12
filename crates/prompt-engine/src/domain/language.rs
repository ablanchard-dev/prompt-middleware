#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DetectedLanguage {
    Fr,
    En,
    Unknown,
}

impl DetectedLanguage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Fr => "fr",
            Self::En => "en",
            Self::Unknown => "unknown",
        }
    }
}
