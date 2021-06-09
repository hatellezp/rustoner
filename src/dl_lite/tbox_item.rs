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

use std::fmt;

use crate::dl_lite::node::ItemDllite;
use crate::kb::knowledge_base::{Implier, TbRule};
use crate::kb::knowledge_base::{Item, TBoxItem};
use crate::kb::types::{DLType, CR};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use crate::dl_lite::utilities::ordering_cmp_helper;

/// TBox items are composed basically of two parts:
/// - a left side
/// - a right side
/// e.g. 'a Human IS a Mortal' left: Human, right: Mortal.
/// Left and right sides are Items.
/// There are two more fields, added for deduction.
///
/// A TBox item added from a file as level 0, now suppose item 'it' was added using
/// a deduction rule 'r' and items in the list 'its', then its level
/// is max(level(i) | i in its).
///
/// The impliers field help to keep track of which items produced a new one using a
/// certain deduction rule.
/// An implier of item 'it' is an array of tuples ('r', 'its') where 'its' produced
/// item 'it' by deduction rule 'r'.
#[derive(Debug, Clone)]
pub struct TbiDllite {
    lside: ItemDllite,
    rside: ItemDllite,
    level: usize,
    impliers: Vec<(CR, Vec<TbiDllite>)>,
}

impl PartialEq for TbiDllite {
    fn eq(&self, other: &Self) -> bool {
        self.lside == other.lside && self.rside == other.rside
    }
}

/*
    I must implement Hash myself because PartialEq is implemented and can cause
    unforeseen errors: thanks Clippy.
    Long explanation short 'a = b' must imply that 'hash(a) = hash(b)', thus because I
    implemented PartialEq I must assure that Hash behaves the same way.
 */
impl Hash for TbiDllite {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.lside.hash(state);
        self.rside.hash(state);
    }
}

// The 'Eq' trait falls back to PartialEq if not implemented.
impl Eq for TbiDllite {}

impl fmt::Display for TbiDllite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} < {}", self.lside, self.rside)
    }
}

// TBox items are ordered by lexicographic order.
impl PartialOrd for TbiDllite {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.lside() == other.lside() && self.rside() == other.rside() {
            Some(Ordering::Equal)
        } else if self.lside().cmp(other.lside()) == Ordering::Less {
            Some(Ordering::Less)
        } else if self.lside().cmp(other.lside()) == Ordering::Greater {
            Some(Ordering::Greater)
        } else {
            self.rside().partial_cmp(other.rside())
        }
    }
}

impl Ord for TbiDllite {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Implier for TbiDllite {
    type Imp = (CR, Vec<TbiDllite>);

    fn implied_by(&self) -> &Vec<(CR, Vec<TbiDllite>)> {
        &(self.impliers)
    }

    /// Compares two impliers to search for minimality, ideally an implier
    /// should be minimal with respect to subset comparison.
    /// Impliers are compared with the following rule:
    /// if imp1 is included in imp2 then imp1 is smaller than imp2, if
    /// imp2 is included in imp1 then imp1 is greater than imp1,
    /// if not comparison can be made then None is returned
    fn cmp_imp(imp1: &(CR, Vec<TbiDllite>), imp2: &(CR, Vec<TbiDllite>)) -> Option<Ordering> {

        let len1 = (&imp1.1).len();
        let len2 = (&imp2.1).len();
        let mut all_good = true; // accumulates equality of element in both arrays
        let mut tbi1: &TbiDllite;
        let mut tbi2: &TbiDllite;
        let (length, ordering) = ordering_cmp_helper(len1, len2);

        for i in 0..length {
            tbi1 = (&imp1.1).get(i).unwrap();
            tbi2 = (&imp2.1).get(i).unwrap();

            all_good = all_good && (tbi1 == tbi2);

            if ! all_good {  // early stopping condition
                break
            }
        }

        match all_good {
            true => Some(ordering),
            false => Option::None,
        }
    }

    /// add a new implier to the array of impliers of self
    /// a new implier is added only if
    /// - it is not equivalent to self
    /// - it is not present in the impliers of self
    /// - there is no implier smaller in the impliers of self
    fn add_to_implied_by(&mut self, mut implier: (CR, Vec<TbiDllite>)) {

        if !(&implier.1).contains(&self) { // verify it is not present in self.impliers
            implier.1.sort();

            // compares the new implier with the present ones
            let contains = self.contains_implier(&implier);

            match contains {
                Option::Some(Ordering::Less) => {  // if it is smaller then a substitution is done
                    let mut cmpd: Option<Ordering>;
                    let mut inner_implier: &(CR, Vec<TbiDllite>);
                    let length: usize = self.impliers.len();

                    for index in 0..length {
                        inner_implier = &self.impliers.get(index).unwrap();
                        cmpd = Self::cmp_imp(&implier, inner_implier);

                        if let Option::Some(Ordering::Less) = cmpd {
                            self.impliers[index] = implier;
                            break; // rust is very smart, told me that value was being used in
                                   // future iteration, so I put a break right here
                        }
                    }
                },
                // all these cases amount to nothing to do
                Option::None => self.impliers.push(implier),
                Option::Some(Ordering::Equal) | Option::Some(Ordering::Greater) => (),
            }
        }
    }
}

impl TBoxItem for TbiDllite {
    type NodeItem = ItemDllite;

    fn lside(&self) -> &ItemDllite {
        &(self.lside)
    }

    fn rside(&self) -> &ItemDllite {
        &(self.rside)
    }

    fn is_trivial(&self) -> bool {
        self.lside.t() == DLType::Bottom || self.rside.t() == DLType::Top
    }

    fn is_negative_inclusion(&self) -> bool {
        self.rside.is_negated()
    }
}

impl TbiDllite {
    pub fn new(lside: ItemDllite, rside: ItemDllite, level: usize) -> Option<TbiDllite> {
        if (lside.t() == DLType::Nominal || rside.t() == DLType::Nominal)
            || (lside.is_negated() || !DLType::same_type(lside.t(), rside.t()))
        {
            Option::None
        } else {
            let impliers: Vec<(CR, Vec<TbiDllite>)> = Vec::new();

            Some(TbiDllite {
                lside,
                rside,
                level,
                impliers,
            })
        }
    }

    pub fn level(&self) -> usize {
        self.level
    }

    pub fn is_contradiction(&self) -> bool {
        self.lside.is_negation(&self.rside)
    }

    pub fn reverse_negation(&self, add_level: bool) -> Option<TbiDllite> {
        /*
        this method creates a new item
         */
        if self.rside.is_negated() {
            let lside = self.lside.clone();
            let rside = self.rside.clone();

            let level: usize = if !add_level {
                self.level
            } else {
                self.level + 1
            };

            TbiDllite::new(rside.negate(), lside.negate(), level)
        } else {
            Option::None
        }
    }

    pub fn apply_rule(
        tbis: Vec<&TbiDllite>,
        rule: &TbRule<TbiDllite>,
        deduction_tree: bool,
    ) -> Option<Vec<TbiDllite>> {
        /*
        put a switch here to add consequences when needed
        every vector in the answer get the vectors that created it in the implied_by field
         */

        let prov_vec = match tbis.len() {
            1 => rule(tbis, deduction_tree),
            2 => rule(tbis, deduction_tree),
            _ => Option::None,
        };

        match prov_vec {
            Option::None => Option::None,
            Some(prov_vec_unwrapped) => {
                let mut final_vec: Vec<TbiDllite> = Vec::new();

                for item in &prov_vec_unwrapped {
                    if !item.is_redundant() {
                        final_vec.push(item.clone());
                    }
                }

                Some(final_vec)
            }
        }
    }

    // function utility for levels
    pub fn get_extrema_level(v: Vec<&TbiDllite>, max_index: usize, get_max: bool) -> usize {
        // for max or min
        let mut extrema_level: usize = if get_max { 0 } else { usize::max_value() };

        // this part is independent of max and min
        let v_len = v.len();
        let max_index = if (v_len - 1) >= max_index {
            max_index
        } else {
            v_len
        };

        for i in 0..max_index {
            if get_max {
                extrema_level = extrema_level.max(v.get(i).unwrap().level);
            } else {
                extrema_level = extrema_level.min(v.get(i).unwrap().level);
            }
        }

        extrema_level
    }
}
