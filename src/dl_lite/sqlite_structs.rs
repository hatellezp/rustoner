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

#[derive(Debug, Clone, PartialEq)]
pub struct RelationDb {
    pub(crate) type_db: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DltypeDb {
    pub(crate) id_db: i64,
    pub(crate) type_db: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SymbolDb {
    pub(crate) id_db: i64,
    pub(crate) name_db: String,
    pub(crate) type_db: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NodeDb {
    pub(crate) id_db: i64,
    pub(crate) name_db: String,
    pub(crate) type_db: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TboxItemDb {
    pub(crate) id_db: i64,
    pub(crate) lside_name_db: String,
    pub(crate) rside_name_db: String,
    pub(crate) lside_db: i64,
    pub(crate) rside_db: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AboxItemCDb {
    pub(crate) id_db: i64,
    pub(crate) constant_name_db: String,
    pub(crate) concept_name_db: String,
    pub(crate) constant_db: i64,
    pub(crate) concept_db: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AboxItemRDb {
    pub(crate) id_db: i64,
    pub(crate) constant1_name_db: String,
    pub(crate) constant2_name_db: String,
    pub(crate) role_name_db: String,
    pub(crate) constant1_db: i64,
    pub(crate) constant2_db: i64,
    pub(crate) role_db: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableDb {
    pub(crate) name_db: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AboxQItemCDb {
    pub(crate) id_db: i64,
    pub(crate) constant_name_db: String,
    pub(crate) concept_name_db: String,
    pub(crate) constant_db: i64,
    pub(crate) concept_db: i64,
    pub(crate) prevalue: f64,
    pub(crate) value: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AboxQItemRDb {
    pub(crate) id_db: i64,
    pub(crate) constant1_name_db: String,
    pub(crate) constant2_name_db: String,
    pub(crate) role_name_db: String,
    pub(crate) constant1_db: i64,
    pub(crate) constant2_db: i64,
    pub(crate) role_db: i64,
    pub(crate) prevalue: f64,
    pub(crate) value: Option<f64>,
}
