#[derive(Debug, Clone, clap::ValueEnum, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum TableType {
    Main,
    Load,
    Temp,
}

impl std::fmt::Display for TableType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TableType::Main => write!(f, "Main"),
            TableType::Load => write!(f, "Load"),
            TableType::Temp => write!(f, "Temp"),
        }
    }
}
