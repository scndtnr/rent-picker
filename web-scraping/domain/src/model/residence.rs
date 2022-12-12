use derive_new::new;

use super::Rooms;

#[derive(new, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Residence {
    name: String,
    transfer: String,
    rooms: Rooms,
}

impl Residence {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn transfer(&self) -> &str {
        &self.transfer
    }
    pub fn rooms(&self) -> &Rooms {
        &self.rooms
    }
}
