use derive_new::new;

use super::{ResidenceHeader, Rooms};

#[derive(new, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Residence {
    header: ResidenceHeader,
    rooms: Rooms,
}

impl Residence {
    pub fn header(&self) -> &ResidenceHeader {
        &self.header
    }
    pub fn rooms(&self) -> &Rooms {
        &self.rooms
    }
}
