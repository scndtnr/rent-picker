use derive_new::new;

use super::TargetArea;

#[derive(new, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResidenceHeader {
    name: String,
    transfer: String,
    area: TargetArea,
    station: String,
}

impl ResidenceHeader {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn transfer(&self) -> &str {
        &self.transfer
    }
    pub fn area(&self) -> &TargetArea {
        &self.area
    }
    pub fn station(&self) -> &str {
        &self.station
    }
}
