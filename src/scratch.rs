mod alg_math;

use crate::alg_math::matrix_building::Filter;

/*
mod benching;

mod dl_lite;
mod helper;
mod interface;
mod kb;

use dl_lite::ontology::OntologyDllite;
use dl_lite::tbox::TBDllite;
use dl_lite::tbox_item::TbiDllite;
use helper::rank_abox;
use interface::format_constants::*;
use kb::aggr_functions::*;
use kb::knowledge_base::{ABox, ABoxItem, Implier, Item, TBox, TBoxItem};
use kb::types::FileType;

use alg_math::bounds::find_bound_complex_wrapper;

use rand::Rng;
use std::time::{Duration, Instant};

use crate::benching::generate_tests::{
    bench_find_bound_complex_wrapper_simple, generate_random_aggr_matrix_simple,
    generate_random_prevalues_simple,
};
use crate::benching::utilities::pretty_print_matrix;
use nalgebra::{max, min};

use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::BufWriter;
use std::ptr::write_bytes;

use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;
use std::{fs, io};

/*
What I need to do:
    bench tbox unraveling
    bench abox unraveling
    bench dot generation
    bench conflict matrix generation
    bench bound finding

    bench ranking (sum of two precedents)
 */

const onto_path: &str = "benchmark_files/onto/";
const symbols_path: &str = "benchmark_files/onto/symbols.txt";
const all_file_type: FileType = FileType::NATIVE;
const verbose: bool = false;
const bench_time_file_path: &str = "benchmark_files/bench_time.csv";

// constants for the bound computing
const TOLERANCE: f64 = 0.0000000000000001;
const M_SCALER: f64 = 1.1;
const B_TRANSLATE: f64 = 1.;
const DOT_COMMAND_LINUX: &str = "dot";
const DOT_COMMAND_WINDOWS: &str = "dot.exe";
const COMMAND_SHELL_LINUX: &str = "sh";
const COMMAND_SHELL_WINDOWS: &str = "cmd";

const COMPLEXITY_LIMIT: usize = 50000;


pub fn main() {
    println!("Hello World!");

    let tbox_filename = "examples/university_tbox";
    let abox_filename = "examples/university_abox2";
    let ft = FileType::NATIVE;
    // let verbose = false;
    let deduction_tree = true;

    let mut onto = OntologyDllite::new(String::from("test"));

    onto.add_symbols_from_file(tbox_filename, ft, verbose);
    onto.add_tbis_from_file(tbox_filename, ft, verbose);
    onto.new_abox_from_file_quantum(abox_filename, ft, verbose);

    let (a, b, c) = onto.conflict_matrix(onto.abox().unwrap(), deduction_tree, verbose);


    println!("{:?}\n{:?}\n{:?}", &a, b, c);

    pretty_print_matrix2(&a);

}

pub fn pretty_print_matrix2(v: &Vec<i8>) {
    let size = (v.len() as f64).sqrt() as usize;

    for i in 0..size {
        for j in 0..size {
            print!("{}, ", v[size*i + j]);
        }
        println!("");
    }
}

 */

pub fn main() {
    println!("Hello there!");

    for n in 5..11 {
        let mut f = Filter::new(n as usize);

        while !f.is_done() {
            // println!("      {}", &f);
            f.next();
        }

        let case_c = f.cc();
        let case_sum = case_c.0 + case_c.1 + case_c.2 + case_c.3;

        let avg_comp: f64 = 1_f64 * ((case_c.0 + case_c.2) as f64) + (n as f64) * ((case_c.1 + case_c.3) as f64);

        println!("size {}: {:?}, sum: {}, total avg: {}, real avg: {}", n, case_c, case_sum, avg_comp, avg_comp / (case_sum as f64));
    }

}
