use derive_new::new;

use super::Rooms;

#[derive(new, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Residence {
    rooms: Rooms,
}
