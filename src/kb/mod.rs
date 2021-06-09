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

//! The kb module is sort of an interface or abstract class module for
//! what knowledge bases are.
//! Rust do not have interfaces not abstract classes, everything is
//! done by traits.
//! Items, TBoxes, ABoxes, every one is implemented by a trait.

pub mod aggr_functions;
pub mod knowledge_base;
pub mod types;
