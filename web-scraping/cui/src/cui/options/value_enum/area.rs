#[derive(Debug, Clone, clap::ValueEnum, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Area {
    Tokyo,
    Kanagawa,
    Saitama,
    Chiba,
}

impl std::fmt::Display for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Area::Tokyo => write!(f, "Tokyo"),
            Area::Kanagawa => write!(f, "Kanagawa"),
            Area::Saitama => write!(f, "Saitama"),
            Area::Chiba => write!(f, "Chiba"),
        }
    }
}
