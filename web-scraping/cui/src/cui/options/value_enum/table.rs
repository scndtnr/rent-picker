#[derive(Debug, Clone, clap::ValueEnum, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Table {
    Room,
    RoomHeader,
}

impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Table::Room => write!(f, "Room"),
            Table::RoomHeader => write!(f, "RoomHeader"),
        }
    }
}
