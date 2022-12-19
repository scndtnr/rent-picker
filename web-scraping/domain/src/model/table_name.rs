use anyhow::bail;

#[derive(Debug, Clone)]
pub enum TableName {
    Room,
    RoomHeader,
}

impl TryFrom<String> for TableName {
    type Error = anyhow::Error;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_uppercase() {
            table if table == "ROOM" => Ok(TableName::Room),
            table if table == "ROOMHEADER" => Ok(TableName::RoomHeader),
            table => bail!("Unknown table: {}", table),
        }
    }
}

impl std::fmt::Display for TableName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TableName::Room => write!(f, "Room"),
            TableName::RoomHeader => write!(f, "RoomHeader"),
        }
    }
}
