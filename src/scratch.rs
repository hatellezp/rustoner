use nalgebra::{Complex, DMatrix, DVector};

mod alg_math;

use alg_math::utilities::*;

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
    let zero: Complex<f64> = Complex { re: 0., im: 0. };
    let n: usize = 2;

    let mut m = DMatrix::from_vec(n, n, vec![zero; n * n]);
    let mut v = DVector::from(vec![zero; n]);

    println!("before modification: {}", matrix_is_zero_complex(&m));

    m[n] = Complex { re: 1., im: 0. };
    println!("after modification: {}", matrix_is_zero_complex(&m));

    println!("m: {:?}", &m);
    println!("v: {:?}", &v);

    v[1] = Complex { re: 1., im: 0. };
    v[0] = Complex { re: 1., im: 2. };
    println!("v: {:?}", &v);

    let scalar: Complex<f64> = Complex { re: 2.0, im: 1.0 };
    multiply_vector_complex(&mut v, scalar);

    println!("v: {:?}", &v);

    println!("m: {:?}", &m);
    m[n + 1] = Complex { re: 1., im: 2. };
    multiply_matrix_complex(&mut m, scalar);
    println!("m: {:?}", &m);
}
