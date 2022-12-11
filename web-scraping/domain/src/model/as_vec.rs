pub trait AsVec {
    type Item;
    fn into_inner(self) -> Vec<Self::Item>;
    fn as_vec(&self) -> &[Self::Item];
    fn as_mut_vec(&mut self) -> &mut Vec<Self::Item>;
    fn sorted_asc(&self) -> Self;
    fn sorted_desc(&self) -> Self;
    fn len(&self) -> usize {
        self.as_vec().len()
    }
    fn is_empty(&self) -> bool {
        self.as_vec().is_empty()
    }
}
