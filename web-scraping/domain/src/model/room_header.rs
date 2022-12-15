use chrono::{DateTime, FixedOffset};
use derive_new::new;

use super::TargetArea;

#[allow(clippy::too_many_arguments)]
#[derive(new, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RoomHeader {
    url: String,
    residence_title: String,
    residence_address: String,
    residence_nearest_station: String,
    residence_age: String,
    residence_floors: String,
    residence_transfer: String,
    residence_area: TargetArea,
    residence_station: String,
    room_floor: String,
    room_rent_price: String,
    room_condo_fee: String,
    room_deposit: String,
    room_key_money: String,
    room_layout: String,
    room_exclusive_area: String,
    created_at: DateTime<FixedOffset>,
}

impl RoomHeader {
    pub fn url(&self) -> &str {
        &self.url
    }
    pub fn residence_title(&self) -> &str {
        &self.residence_title
    }
    pub fn residence_address(&self) -> &str {
        &self.residence_address
    }
    pub fn residence_nearest_station(&self) -> &str {
        &self.residence_nearest_station
    }
    pub fn residence_age(&self) -> &str {
        &self.residence_age
    }
    pub fn residence_floors(&self) -> &str {
        &self.residence_floors
    }
    pub fn residence_transfer(&self) -> &str {
        &self.residence_transfer
    }
    pub fn residence_area(&self) -> &TargetArea {
        &self.residence_area
    }
    pub fn residence_station(&self) -> &str {
        &self.residence_station
    }
    pub fn room_floor(&self) -> &str {
        &self.room_floor
    }
    pub fn room_rent_price(&self) -> &str {
        &self.room_rent_price
    }
    pub fn room_condo_fee(&self) -> &str {
        &self.room_condo_fee
    }
    pub fn room_deposit(&self) -> &str {
        &self.room_deposit
    }
    pub fn room_key_money(&self) -> &str {
        &self.room_key_money
    }
    pub fn room_layout(&self) -> &str {
        &self.room_layout
    }
    pub fn room_exclusive_area(&self) -> &str {
        &self.room_exclusive_area
    }
    pub fn created_at(&self) -> DateTime<FixedOffset> {
        self.created_at
    }
}
