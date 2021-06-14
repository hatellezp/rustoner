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
