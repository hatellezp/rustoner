use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::io;

use crate::dl_lite::abox_item::ABI_DLlite;
use crate::dl_lite::tbox_item::TBI_DLlite;
use crate::kb::types::DLType;

pub trait Item: PartialOrd + Ord + PartialEq + Eq + Debug + Hash + Display + Sized {
    fn t(&self) -> DLType;
    fn base(node: &Self) -> &Self;
    fn child(node: Option<&Self>, depth: usize) -> Option<Vec<&Self>>;
    fn is_negated(&self) -> bool;
}

pub trait ABoxItem: PartialOrd + Ord + PartialEq + Eq + Debug + Hash + Display + Sized {
    type NodeItem: Item;

    fn item(&self) -> &Self::NodeItem;
    fn negate(&self) -> Self;
    fn t(&self) -> DLType;
}

pub trait ABox: PartialEq + Debug + Display {
    type AbiItem: ABoxItem;

    fn name(&self) -> String;
    fn len(&self) -> usize;
    fn add(&mut self, abi: Self::AbiItem) -> bool;
    fn items(&self) -> &Vec<Self::AbiItem>;
    fn get(&self, index: usize) -> Option<&Self::AbiItem>;
    fn sort(&mut self);
    fn is_empty(&self) -> bool;
    fn contains(&self, abi: &Self::AbiItem) -> bool;
}

pub trait TBoxItem: PartialEq + Eq + PartialOrd + Ord + Debug + Hash + Display + Sized{
    type NodeItem: Item;

    fn lside(&self) -> &Self::NodeItem;
    fn rside(&self) -> &Self::NodeItem;
    fn is_trivial(&self) -> bool;
    fn is_negative_inclusion(&self) -> bool;
    fn implied_by(&self) -> &Vec<Vec<Self>>; // where Self: Sized;
    fn add_to_implied_by(&mut self, impliers: Vec<Self>); // where Self: Sized;

    // test if i can implement here some behaviour
    fn is_positive_inclusion(&self) -> bool {
        !self.is_negative_inclusion()
    }

    fn is_redundant(&self) -> bool {
        self.lside() == self.rside()
    }
}

pub trait TBox: PartialEq + Debug + Display {
    type TbiItem: TBoxItem;

    fn len(&self) -> usize;
    fn add(&mut self, abi: Self::TbiItem) -> bool;
    fn items(&self) -> &Vec<Self::TbiItem>;
    fn get(&self, index: usize) -> Option<&Self::TbiItem>;
    fn sort(&mut self);
    fn is_empty(&self) -> bool;
    fn contains(&self, tbi: &Self::TbiItem) -> bool;
}


// TESTING: does this work ?
pub type Symbol= HashMap<String, (usize, DLType)>;

pub type TbRule<T: TBoxItem> = fn(Vec<&T>, bool) -> Option<Vec<T>>;
// pub type TbRule = fn(Vec<&TBI_DLlite>, bool) -> Option<Vec<TBI_DLlite>>;

pub type AbRule = fn(Vec<&ABI_DLlite>, Vec<&TBI_DLlite>) -> Option<Vec<ABI_DLlite>>;
