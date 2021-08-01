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

// =================================================================================================
// MODULE DECLARATION

mod alg_math; // this module is for computing the rank, matrix manipulation and interpolation are
              // defined here
mod dl_lite; // dl_lite reasoner is in this module
mod graph_maker; // a module that is only a file, creates a graph from ontologies and alike
mod helper; // helper functions to parse files, update list and other
mod interface; // module with the cli interface
mod kb;
mod tasks;

// END OF MODULE DECLARATION
// =================================================================================================

// =================================================================================================
// STRUCTS AND FUNCTION IMPORTS

// for cli interface, need to import it so the interface module works
use structopt::StructOpt;

// from the interface module
use crate::interface::cli::{AggrName, Cli, Task};

// (for dot -args blabla, create a pdf image)
use crate::tasks::{task_abox_related, task_tbox_related};
use std::path::PathBuf;

// END OF IMPORTS
// =================================================================================================

// =================================================================================================
// TYPES RELATED TO EACH TASK
type TBoxRelatedPaths<'a> = (
    &'a Option<PathBuf>,
    &'a Option<PathBuf>,
    &'a Option<PathBuf>,
);
type ABoxRelatedPaths<'a> = (
    &'a Option<PathBuf>,
    &'a Option<PathBuf>,
    &'a Option<PathBuf>,
    &'a Option<PathBuf>,
);

// type for the numeric adjusters
pub type Adjusters = (f64, f64, f64);

// END OF TYPE DECLARATION
// =================================================================================================

// =================================================================================================
// SOME CONSTANTS

// constants for the bound computing
// this values are not random, DO NOT TWEAK THEM  if you don't know what you're doing
const TOLERANCE: f64 = 0.0000000000000001; // below this value we can consider values are equal
                                           // computing sinus introduces rounding errors
const M_SCALE: f64 = 1.1; // avoid singular matrices
const B_TRANSLATE: f64 = 1.; // I found (not on the paper) that this value is superfluous in
                             // the particular case of FFT interpolation, thus is
                             // set to the multiplicative identity: 1

// commands for executing dot command
const DOT_COMMAND_LINUX: &str = "dot";
const DOT_COMMAND_WINDOWS: &str = "dot.exe";
const COMMAND_SHELL_LINUX: &str = "sh";
const COMMAND_SHELL_WINDOWS: &str = "cmd";

// END OF CONSTANTS DECLARATION
// =================================================================================================

// the main function
pub fn main() {
    let args = Cli::from_args();

    // get all arguments regardless of the task
    let task: Task = args.task;
    let path_tbox_op: Option<std::path::PathBuf> = args.path_tbox;
    let path_abox_op: Option<std::path::PathBuf> = args.path_abox;
    let path_symbols_op: Option<std::path::PathBuf> = args.path_symbols;
    let path_output_op: Option<std::path::PathBuf> = args.path_output;
    let verbose: bool = args.verbose;
    let silent: bool = args.silent;
    let aggr_name_op: Option<AggrName> = args.aggr;

    // now do what you are ask
    match task {
        Task::VerTB | Task::GenConTB => {
            let tbox_paths: TBoxRelatedPaths = (&path_tbox_op, &path_symbols_op, &path_output_op);

            task_tbox_related(tbox_paths, task, verbose, silent);
        }
        Task::VerAB | Task::CleanAB | Task::GenConAB | Task::RankAB => {
            let abox_paths: ABoxRelatedPaths = (
                &path_abox_op,
                &path_tbox_op,
                &path_symbols_op,
                &path_output_op,
            );

            task_abox_related(abox_paths, &aggr_name_op, task, verbose, silent);
        }
        _ => println!("NOT IMPLEMENTED !!!"),
    }
}
