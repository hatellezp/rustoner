/*
© - 2021 – UMON
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

use std::str::FromStr;
use std::string::ParseError;
use structopt::StructOpt;

/// Every task the the binary dl_lite_r can do, each is explained as a comment.
/// more to be added after
#[derive(Debug)]
pub enum Task {
    VerTB,    // verify tbox
    GenConTB, // generate consequence tree tbox
    CTB,      // complete tbox
    VerAB,    // verify abox
    CleanAB,  // clean from self conflicts
    GenConAB, // generate consequence tree abox
    CAB,      // complete abox
    RankAB,   // rank assertions on abox
    Undefined,
}

/// Implementing 'FromStr' allows to cast an string value as 'Task' enum
impl FromStr for Task {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "vertb" => Ok(Task::VerTB),
            "gencontb" => Ok(Task::GenConTB),
            "ctb" => Ok(Task::CTB),
            "verab" => Ok(Task::VerAB),
            "cleanab" => Ok(Task::CleanAB),
            "genconab" => Ok(Task::GenConAB),
            "cab" => Ok(Task::CAB),
            "rankab" => Ok(Task::RankAB),
            _ => Ok(Task::Undefined),
        }
    }
}

/// name of the aggregate operators for credibility of subsets
#[derive(Debug)]
pub enum AggrName {
    Max,
    Min,
    Sum,
    Mean,
    Count,
    Undefined,
}

/// to cast to enum from string
impl FromStr for AggrName {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s2 = s.trim();

        match s2 {
            "sum" => Ok(AggrName::Sum),
            "max" => Ok(AggrName::Max),
            "min" => Ok(AggrName::Min),
            "count" => Ok(AggrName::Count),
            "mean" => Ok(AggrName::Mean),
            _ => Ok(AggrName::Undefined),
        }
    }
}

/// The binary dl_lite_r works through an interface of arguments of the
/// form '--argname'.
/// They should be self-explanatory.
#[derive(StructOpt, Debug)]
pub struct Cli {
    #[structopt(
        short = "t",
        long = "task",
        help = "describes the wanted task, (vertb|gencontb|ctb|verab|genconab|cleanab|cab|rankab)"
    )]
    pub task: Task,

    #[structopt(long = "verbose")]
    pub verbose: bool,

    #[structopt(long = "silent", help = "almost no prompt discussion")]
    pub silent: bool,

    #[structopt(parse(from_os_str), long = "tbox", help = "path to the tbox file")]
    pub path_tbox: Option<std::path::PathBuf>,

    #[structopt(parse(from_os_str), long = "abox", help = "path to the abox file")]
    pub path_abox: Option<std::path::PathBuf>,

    #[structopt(
        parse(from_os_str),
        long = "output",
        help = "optional, if present will output the result of the task to this file"
    )]
    pub path_output: Option<std::path::PathBuf>,

    #[structopt(
        parse(from_os_str),
        long = "symbols",
        help = "optional, if present will parse symbols from this file instead on symbols in the tbox file"
    )]
    pub path_symbols: Option<std::path::PathBuf>,

    #[structopt(
        long = "aggr",
        help = "choose a function to aggregate during conflict graph computing: (sum|min|max|count|mean)"
    )]
    pub aggr: Option<AggrName>,
}
