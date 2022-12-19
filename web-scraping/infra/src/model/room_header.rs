use domain::model::{Jst, RoomHeader};
use sqlx::{types::chrono::NaiveDateTime, FromRow};

#[derive(FromRow, Debug)]
pub struct RoomHeaderRecord {
    pub url: String,
    pub residence_title: String,
    pub residence_address: String,
    pub residence_nearest_station: String,
    pub residence_age: String,
    pub residence_floors: String,
    pub residence_transfer: String,
    pub residence_area: String,
    pub residence_station: String,
    pub room_floor: String,
    pub room_rent_price: String,
    pub room_condo_fee: String,
    pub room_deposit: String,
    pub room_key_money: String,
    pub room_layout: String,
    pub room_exclusive_area: String,
    pub created_at: NaiveDateTime,
}

impl From<RoomHeader> for RoomHeaderRecord {
    fn from(header: RoomHeader) -> Self {
        Self {
            url: header.url().to_string(),
            residence_title: header.residence_title().to_string(),
            residence_address: header.residence_address().to_string(),
            residence_nearest_station: header.residence_nearest_station().to_string(),
            residence_age: header.residence_age().to_string(),
            residence_floors: header.residence_floors().to_string(),
            residence_transfer: header.residence_transfer().to_string(),
            residence_area: header.residence_area().to_string(),
            residence_station: header.residence_station().to_string(),
            room_floor: header.room_floor().to_string(),
            room_rent_price: header.room_rent_price().to_string(),
            room_condo_fee: header.room_condo_fee().to_string(),
            room_deposit: header.room_deposit().to_string(),
            room_key_money: header.room_key_money().to_string(),
            room_layout: header.room_layout().to_string(),
            room_exclusive_area: header.room_exclusive_area().to_string(),
            created_at: header.created_at().naive_utc(),
        }
    }
}

impl TryFrom<RoomHeaderRecord> for RoomHeader {
    type Error = anyhow::Error;
    fn try_from(header: RoomHeaderRecord) -> Result<Self, Self::Error> {
        Ok(Self::new(
            header.url,
            header.residence_title,
            header.residence_address,
            header.residence_nearest_station,
            header.residence_age,
            header.residence_floors,
            header.residence_transfer,
            header.residence_area.try_into()?,
            header.residence_station,
            header.room_floor,
            header.room_rent_price,
            header.room_condo_fee,
            header.room_deposit,
            header.room_key_money,
            header.room_layout,
            header.room_exclusive_area,
            Jst::from_utc_datetime(&header.created_at),
        ))
    }
}
