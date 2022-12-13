use super::{as_vec::AsVec, RoomHeader};
use derive_new::new;

#[derive(new, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RoomHeaders(Vec<RoomHeader>);

impl From<Vec<RoomHeader>> for RoomHeaders {
    fn from(rooms: Vec<RoomHeader>) -> Self {
        Self::new(rooms)
    }
}

impl AsVec for RoomHeaders {
    type Item = RoomHeader;

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
