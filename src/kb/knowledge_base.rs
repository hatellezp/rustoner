/*
 © - 2021 – UMONS
Horacio Alejandro Tellez Perez

LICENSE GPLV3+:
This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see https://www.gnu.org/licenses/.
*/

use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;

use crate::kb::types::DLType;
use std::cmp::Ordering;

pub trait Item: PartialOrd + Ord + PartialEq + Eq + Debug + Hash + Display + Sized {
    fn t(&self) -> DLType;
    fn base(node: &Self) -> &Self;
    fn child(node: Option<&Self>, depth: usize) -> Option<Vec<&Self>>;
    fn is_negated(&self) -> bool;
}

// trait to allow for impliers to be for everyone
pub trait Implier {
    type Imp: Clone + PartialOrd + Ord + PartialEq + Eq;

    fn implied_by(&self) -> &Vec<Self::Imp>;
    fn cmp_imp(imp1: &Self::Imp, imp2: &Self::Imp) -> Option<Ordering>;
    fn add_to_implied_by(&mut self, implier: Self::Imp);

    fn contains_implier(&self, implier: &Self::Imp) -> Option<Ordering> {
        for inner_implier in self.implied_by() {
            let cpmd = Self::cmp_imp(implier, inner_implier);

            if cpmd.is_some() {
                return cpmd;
            }
        }

        Option::None
    }
}

pub trait ABoxItem:
    PartialOrd + Ord + PartialEq + Eq + Debug + Hash + Display + Sized + Implier
{
    type NodeItem: Item;
    type TBI: TBoxItem;

    fn item(&self) -> &Self::NodeItem;
    fn negate(&self) -> Self;
    fn t(&self) -> DLType;
    // fn implied_by(&self) -> &Vec<(Vec<Self::TBI>, Vec<Self>)>;
    // fn add_to_implied_by(&mut self, impliers: (Vec<Self::TBI>, Vec<Self>)); // where Self: Sized;
}

pub trait ABox: PartialEq + Debug + Display {
    type AbiItem: ABoxItem;

    fn name(&self) -> String;
    fn len(&self) -> usize;
    fn add(&mut self, abi: Self::AbiItem) -> bool;
    fn items(&self) -> &Vec<Self::AbiItem>;
    fn get(&self, index: usize) -> Option<&Self::AbiItem>;
    fn sort(&mut self);
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn contains(&self, abi: &Self::AbiItem) -> bool;
}

pub trait TBoxItem:
    PartialEq + Eq + PartialOrd + Ord + Debug + Hash + Display + Sized + Implier
{
    type NodeItem: Item;

    fn lside(&self) -> &Self::NodeItem;
    fn rside(&self) -> &Self::NodeItem;
    fn is_trivial(&self) -> bool;
    fn is_negative_inclusion(&self) -> bool;
    // fn implied_by(&self) -> &Vec<Vec<Self>>; // where Self: Sized;
    // fn add_to_implied_by(&mut self, impliers: Vec<Self>); // where Self: Sized;

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
pub type SymbolDict = HashMap<String, (usize, DLType)>;

pub type TbRule<T> = fn(Vec<&T>, bool) -> Option<Vec<T>>;

pub type AbRule<T, A> = fn(Vec<&A>, Vec<&T>, bool) -> Option<Vec<A>>;

pub type AggrFn = fn(Vec<f64>) -> f64;
