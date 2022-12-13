use domain::model::{Jst, RoomHeader};
use sqlx::{types::chrono::NaiveDateTime, FromRow};

#[derive(FromRow, Debug)]
pub struct RoomHeaderTable {
    pub url: String,
    pub residence_title: String,
    pub residence_transfer: String,
    pub residence_area: String,
    pub residence_station: String,
    pub created_at: NaiveDateTime,
}

impl From<RoomHeader> for RoomHeaderTable {
    fn from(header: RoomHeader) -> Self {
        Self {
            url: header.url().to_string(),
            residence_title: header.residence_title().to_string(),
            residence_transfer: header.residence_transfer().to_string(),
            residence_area: header.residence_area().to_string(),
            residence_station: header.residence_station().to_string(),
            created_at: header.created_at().naive_utc(),
        }
    }
}

impl TryFrom<RoomHeaderTable> for RoomHeader {
    type Error = anyhow::Error;
    fn try_from(header: RoomHeaderTable) -> Result<Self, Self::Error> {
        Ok(Self::new(
            header.url,
            header.residence_title,
            header.residence_transfer,
            header.residence_area.try_into()?,
            header.residence_station,
            Jst::from_native_datetime(&header.created_at),
        ))
    }
}
