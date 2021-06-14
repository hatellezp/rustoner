use std::fmt::Display;

pub trait DataItem: Clone + Display {
    fn negate(&self) -> Self;
}

pub trait DataHolder: Clone + Display {
    type DI: DataItem;

    fn len(&self) -> usize;
    fn contains(&self, i: &Self::DI) -> bool;
    fn add_item(&self, i: Self::DI) -> bool;
    fn get(&self, index: usize) -> Option<&Self::DI>;

    // This last method build a new DataHolder from an array of
    // indices,
    // the array has always the same length, teh same as the DataHolder,
    // a true value at position i means DataItem at position i is in the
    // new DataHolder
    fn sub_data_holder(&self, indices: &[bool]) -> Self;
}

pub trait Oracle {
    type DH: DataHolder;

    fn is_consistent(&self, dh: &Self::DH) -> bool;
    fn is_inconsistent(&self, dh: &Self::DH) -> bool;
}
