use derive_new::new;

use super::{ResidenceHeader, Rooms, TargetArea};

#[derive(new, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Residence {
    header: ResidenceHeader,
    rooms: Rooms,
}

impl Residence {
    pub fn name(&self) -> &str {
        self.header.name()
    }
    pub fn transfer(&self) -> &str {
        self.header.transfer()
    }
    pub fn area(&self) -> &TargetArea {
        self.header.area()
    }
    pub fn station(&self) -> &str {
        self.header.station()
    }
    pub fn rooms(&self) -> &Rooms {
        &self.rooms
    }
}
