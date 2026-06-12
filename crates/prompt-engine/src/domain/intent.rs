#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Intent {
    Expliquer,
    Rediger,
    Corriger,
    Coder,
    Analyser,
    Comparer,
    Resumer,
    Planifier,
    Auditer,
    Transformer,
}

impl Intent {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Expliquer => "expliquer",
            Self::Rediger => "rediger",
            Self::Corriger => "corriger",
            Self::Coder => "coder",
            Self::Analyser => "analyser",
            Self::Comparer => "comparer",
            Self::Resumer => "resumer",
            Self::Planifier => "planifier",
            Self::Auditer => "auditer",
            Self::Transformer => "transformer",
        }
    }
}
