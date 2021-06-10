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

/// An Item is the basic construct in DL types
/// - a role
/// - a concept
/// - a nominal name
/// more complex constructs as TBox items are built over this
/// an item can
/// - be basic (no tranformation)
/// - have a child (like a tree, 'a inter b' has two childs : 'a' and 'b')
/// - be negated or not
/// each Item has a type defined by the DLType defined in the 'types.rs' file of this module.
pub trait Item: PartialOrd + Ord + PartialEq + Eq + Debug + Hash + Display + Sized {
    fn t(&self) -> DLType;
    fn base(node: &Self) -> &Self;
    fn child(node: Option<&Self>, depth: usize) -> Option<Vec<&Self>>;
    fn is_negated(&self) -> bool;
}

/// Specialization of an Item. Leveled items have
/// a level field to keep track on their position in the
/// deduction tree.
pub trait LeveledItem {
    fn level(&self) -> usize;
}

/// Whenever a deduction rule is applied to TBox items or ABox items two things happen
/// - a certain rule is applied
/// - some items triggered the rule
/// the Implier trait captures those items that trigger some rules, regardless of if they
/// are TBox items or ABox items.
/// Impliers can be compared sometimes by a subset criteria, this allows for minimal
/// impliers and non redundant information.
pub trait Implier {
    type Imp: Clone + PartialOrd + Ord + PartialEq + Eq;

    fn implied_by(&self) -> &Vec<Self::Imp>;
    fn cmp_imp(imp1: &Self::Imp, imp2: &Self::Imp) -> Option<Ordering>;
    fn add_to_implied_by(&mut self, implier: Self::Imp);

    /// search if an comparable implier is present
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

/// An ABoxItem is an assertion in an ABox, it contains two units
/// - the constants
/// - the TBox item
/// for example in the case 'horacio: Student', 'horacio' is the constant and
/// 'Student' is the TBox item.
/// We can
/// - access the constants in an ABoxItem
/// - negate it (for example 'horacio: NOT Student')
/// - access its type (the type of the TBox item)
pub trait ABoxItem:
    PartialOrd + Ord + PartialEq + Eq + Debug + Hash + Display + Sized + Implier
{
    type NodeItem: Item;
    type TBI: TBoxItem;

    fn item(&self) -> &Self::NodeItem;
    fn negate(&self) -> Self;
    fn t(&self) -> DLType;
}

/// An ABox is exactly a materialization of an ABox in an ontology.
/// Virtually a list of ABox assertions, there are nevertheless some functionalities
/// - it has a name
/// - we can access its size
/// - we can add new assertions
/// - we can get a list of its items
/// - we can get an specific item (a reference to it)
/// - sort the items (almost every struct and enum in this work implements PartialOrd or Ord)
/// - we can check for emptiness
/// - check for presence of an item
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

/// A TBoxItem is composed of two parts, take 'a Human IS a Mortal' for example,
/// then the left side of the TBox item would be 'Human', 'Mortal' being the right side.
/// Informally, any DL has two trivial items: 'Bottom' and 'Top', in OWL they are
/// implemented as 'Nothing' and 'Thing' respectively, here they are named
/// 'Bottom' and 'Top', pretty easy.
/// We can be done with TBox items:
/// - access its left and right side
/// - check for triviality (e.g. 'Nothing is a Human' or 'a Human is a Thing')
/// - check for negative inclusion (e.g. 'a Human is NOT a Dog')
/// - check for redundancy (e.g. 'a Human is a Human').
pub trait TBoxItem:
    PartialEq + Eq + PartialOrd + Ord + Debug + Hash + Display + Sized + Implier
{
    type NodeItem: Item;

    fn lside(&self) -> &Self::NodeItem;
    fn rside(&self) -> &Self::NodeItem;
    fn is_trivial(&self) -> bool;
    fn is_negative_inclusion(&self) -> bool;

    // test if i can implement here some behaviour
    fn is_positive_inclusion(&self) -> bool {
        !self.is_negative_inclusion()
    }

    fn is_redundant(&self) -> bool {
        self.lside() == self.rside()
    }
}

/// Materialization of a TBox of an ontology. What we can do:
/// - access its size
/// - add new TBox inclusions
/// - get a vector of the items present
/// - get a reference to an specific item
/// - sort the TBox
/// - check for emptiness
/// - check if a certain item is present
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

/// A real ontology is made of words (e.g. 'a Human IS a Mortal', 'horacio IS a Human').
/// Comparisons between items are unavoidable and some words are longer than others, some
/// really long.
/// Our solution to this is to abstract every word present in the ontology to a number of
/// constant size. From a theoretical point of view we can really talk about complexity,
/// as each atomic instruction can be done in constant time, and from a practical point of
/// view the engine that do the reasoning works thus on numbers, which results in easiness
/// of read, easiness of debug, and it is notably faster.
/// SymbolDict contains this information, for each individual name:
/// role name, basic concept name or constant name, a tuple is associated: (number id, type)
/// (e.g. 'a Human IS a Mortal', 'horacio IS a Human' would be transformed in
///     'Bottom' : (0, BaseConcept)
///     'Top'    : (1, BaseConcept)
///     'Human'  : (2, BaseConcept)
///     'Mortal' : (3, BaseConcept)
///     'horacio': (4, Nominal)
/// ).
/// Note how 'Bottom' and 'Top' are automatically added.
pub type SymbolDict = HashMap<String, (usize, DLType)>;

/// Deduction rules are defined as functions that take the corresponding
/// arguments.
/// - TBox deduction rules take a variable number of TBox items as argument and
///   produce a variable number of new TBox items.
///  - ABox deduction rules take a variable number of TBox items and a variable
///    number of ABox items and produce a variable number of new ABox items.
/// Both types of rule have a swtich (deduction tree activated or not) to keep track
/// of which items produced each new item.
pub type TbRule<T> = fn(Vec<&T>, bool) -> Option<Vec<T>>;
pub type AbRule<T, A> = fn(Vec<&A>, Vec<&T>, bool) -> Option<Vec<A>>;

/// Aggregate operators produce new credibility values for subsets of an ABox
/// following each aggregation
/// (e.g. Sum([1,2,3,1]) = 1 + 2 + 3 + 1 = 7).
pub type AggrFn = fn(Vec<f64>) -> f64;
