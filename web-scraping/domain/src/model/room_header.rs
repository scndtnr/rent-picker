use chrono::{DateTime, FixedOffset};
use derive_new::new;

use super::TargetArea;

#[derive(new, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RoomHeader {
    url: String,
    residence_title: String,
    residence_transfer: String,
    residence_area: TargetArea,
    residence_station: String,
    created_at: DateTime<FixedOffset>,
}

impl RoomHeader {
    pub fn url(&self) -> &str {
        &self.url
    }
    pub fn residence_title(&self) -> &str {
        &self.residence_title
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
    pub fn created_at(&self) -> DateTime<FixedOffset> {
        self.created_at
    }
}
