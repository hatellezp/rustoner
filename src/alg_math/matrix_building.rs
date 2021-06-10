/*
UMONS 2021
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

use std::cmp::min;
use std::collections::HashMap;

/// This file comprehends two main functions, computing the Indicator function,
/// which is a hashmap, and building a matrix for computing the abox ranking.

/// The indicator struct models the function Indicator.
/// It has
/// - an indicator field to store the result of the function,
/// - a conflict limit size (that will default to length size if none is provided) to
///   limit the search of conflict to a certain size
/// - the actual length of the knowledge base in question
/// - an id_generator, this function will generate a filter for subsets of the knowledge base
///   e.g. suppose length = 3, then the result of id_generator is the following
///   - id_generator(0) -> [false, false, false]
///   - id_generator(1) -> [true, false, false]
///   - id_generator(2) -> [false, true, false]
///   - id_generator(3) -> [false, false, true]
///   - ...
///   - id_generator(7) -> [true, true, true]
/// The id_generator function generates filter for the subsets of the knowledge base following
/// first an order of size and for a fixed size a lexicographic order.
pub struct Indicator {
    // I(id of B, T or F, index of alpha, index of beta) -> value of I
    indicator: HashMap<(usize, bool, usize, usize), bool>,
    conflict_limit: usize, // limit size of conflicts to search for
    length: usize,         // size of the actual knowledge base
    subset_filter: Vec<bool>,
}

impl Indicator {
    pub fn new(length: usize, conflict_limit: Option<usize>) -> Indicator {
        let conflict_limit = if matches!(conflict_limit, Some(_)) {
            min(length, conflict_limit.unwrap())
        } else {
            length
        };

        let indicator: HashMap<(usize, bool, usize, usize), bool> = HashMap::new();

        let subset_filter = vec![false; length];

        Indicator {
            indicator,
            conflict_limit,
            length,
            subset_filter,
        }
    }

    pub fn next_filter(&mut self) {}
}
