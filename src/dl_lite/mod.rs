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

//! Implementation of dl_lite_r reasoner. ABox and TBox items are defined here,
//! ABox and TBox as well. Reasoning tasks to detect for inconsistency, to complete
//! and others are implemented here.
//! Some utilities to parse ontology files are also defined here.
pub mod abox;
pub mod abox_item;
pub mod abox_item_quantum;
pub mod helpers_and_utilities;
pub mod json_filetype_utilities;
pub mod native_filetype_utilities;
pub mod node;
pub mod ontology;
pub mod rule;
pub mod string_formatter;
pub mod tbox;
pub mod tbox_item;
mod utilities; // only for me
