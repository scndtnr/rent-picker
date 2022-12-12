use super::{as_vec::AsVec, Room};
use derive_new::new;

#[derive(new, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rooms(Vec<Room>);

impl From<Vec<Room>> for Rooms {
    fn from(rooms: Vec<Room>) -> Self {
        Self::new(rooms)
    }
}

impl AsVec for Rooms {
    type Item = Room;

    fn into_inner(self) -> Vec<Self::Item> {
        self.0
    }
    fn as_vec(&self) -> &[Self::Item] {
        &self.0
    }
    fn as_mut_vec(&mut self) -> &mut Vec<Self::Item> {
        &mut self.0
    }
    fn sorted_asc(&self) -> Self {
        todo!()
    }
    fn sorted_desc(&self) -> Self {
        todo!()
    }
}
