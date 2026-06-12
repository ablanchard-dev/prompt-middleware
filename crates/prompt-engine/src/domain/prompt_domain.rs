#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromptDomain {
    General,
    Code,
    Business,
    Redaction,
    Learning,
    Hr,
    Cybersecurity,
    FinanceTrading,
    TechnicalProject,
}

impl PromptDomain {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::General => "general",
            Self::Code => "code",
            Self::Business => "business",
            Self::Redaction => "redaction",
            Self::Learning => "apprentissage",
            Self::Hr => "rh",
            Self::Cybersecurity => "cybersecurite",
            Self::FinanceTrading => "finance_trading",
            Self::TechnicalProject => "projet_technique",
        }
    }
}
