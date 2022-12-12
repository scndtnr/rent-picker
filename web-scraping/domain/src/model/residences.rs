use super::{as_vec::AsVec, Residence};
use derive_new::new;

#[derive(new, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Residences(Vec<Residence>);

impl From<Vec<Residence>> for Residences {
    fn from(residences: Vec<Residence>) -> Self {
        Self::new(residences)
    }
}

impl AsVec for Residences {
    type Item = Residence;

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
