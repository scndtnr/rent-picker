#[derive(Debug, Clone, clap::ValueEnum, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum DataAction {
    Summary,
    Top,
    Export,
}
