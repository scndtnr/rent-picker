use super::{as_vec::AsVec, RawRoom};
use derive_new::new;

#[derive(new, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RawRooms(Vec<RawRoom>);

impl From<Vec<RawRoom>> for RawRooms {
    fn from(rooms: Vec<RawRoom>) -> Self {
        Self::new(rooms)
    }
}

impl AsVec for RawRooms {
    type Item = RawRoom;

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
