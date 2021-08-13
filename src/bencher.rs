mod alg_math;

mod benching;
mod dl_lite;
mod helper;
mod interface;
mod kb;

use crate::dl_lite::ontology::OntologyDllite;

use crate::helper::rank_abox;

use crate::kb::aggr_functions::*;
use crate::kb::knowledge_base::{ABox, TBox, TBoxItem};
use crate::kb::types::FileType;

use crate::alg_math::bounds::find_bound_complex_wrapper;

use rand::Rng;
use std::time::Instant;

/*
use benching::generate_tests::{
    bench_find_bound_complex_wrapper_simple, generate_random_aggr_matrix_simple,
    generate_random_prevalues_simple,
};

 */

use std::fs::{File, OpenOptions};
use std::io::prelude::*;

use crate::dl_lite::abox::AbqDllite;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;
use std::{fs, io};

use crate::alg_math::bounds::Adjusters;

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
const all_file_type: FileType = FileType::Native;
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

    let filename = "benchmark_files/bound_times_large.csv";

    let lower_size: usize = 500;
    let upper_size: usize = 600;

    // let iterations: usize = 5;
    let iterations: usize = 1;

    let lower_value: f64 = 0.5;
    let upper_value: f64 = 1.5;

    // let densities: Vec<f64> = vec![0.2, 0.4, 0.6, 0.8, 1_f64];
    let densities: Vec<f64> = vec![0.6];

    /*
    bench_bound_finding(
        lower_size,
        upper_size,
        iterations,
        lower_value,
        upper_value,
        &densities,
        filename,
    );

    */
    compute_all_benches();
}

pub fn bench_bound_finding(
    lower_size: usize,
    upper_size: usize,
    iterations: usize,
    lower_value: f64,
    upper_value: f64,
    densities: &Vec<f64>,
    filename: &str,
) {
    // prepare file:
    // first remove file
    let _o = fs::remove_file(filename);

    println!(
        "INFO: size between [{}, {}], iterations: {}, values in [{}, {}], file name: {}",
        lower_size, upper_size, iterations, lower_value, upper_value, filename
    );

    println!("writting header line to {}", filename);
    {
        let mut file = File::create(filename).unwrap();
        write!(file, "id,size,density,iteration,time,zeroes,clean\n");
    }

    let mut rng = rand::thread_rng();

    // we keep: (size, density, iteration): (time, ratio of zeroes, ratio of clean values)
    let _stats: Vec<((usize, f64, usize), (f64, f64, f64))> = Vec::new();

    let mut counter: usize = 0;

    for ref_size in [10, 20, 50, 100, 200, 400, 800].iter() {
        let size = *ref_size;
        println!("size is: {}", size);

    // for size in lower_size..(upper_size + 1) {
        for density in densities {
            let sqrt_density = (1_f64 - *density).sqrt();

            for iteration in 0..iterations {
                // update counter
                counter += 1;

                // create a matrix here
                let dim = size * size;
                let mut v = vec![0_f64; dim];

                let mut non_zeroes: usize = 0;

                for i in 0..size {
                    if rng.gen_range(0_f64..1_f64) > sqrt_density {
                        for j in 0..size {
                            if rng.gen_range(0_f64..1_f64) > sqrt_density {
                                let value = rng.gen_range(lower_value..upper_value);
                                v[size * i + j] = value;
                                non_zeroes += 1;
                            }
                        }
                    }
                }

                let ratio_zeroes = ((dim - non_zeroes) as f64) / (dim as f64);

                let mut clean_values: usize = 0;
                for i in 0..size {
                    let mut is_clean = true;

                    for j in 0..size {
                        is_clean = is_clean && v[size * j + i] == 0_f64 && v[size * i + j] == 0_f64;
                    }

                    if is_clean {
                        clean_values += 1;
                    }
                }

                let ratio_clean_values = ((size - clean_values) as f64) / (size as f64);

                let now = Instant::now();

                let b = find_bound_complex_wrapper(v, TOLERANCE, M_SCALER, B_TRANSLATE);

                let elapsed = now.elapsed().as_secs_f64();

                println!("     --  bound is {}", b.unwrap());

                println!(" --- size: {}, density: {}, iter: {}, time: {}, zeroes ratio: {}, clean values ration: {}", size, *density, iteration, elapsed, ratio_zeroes, ratio_clean_values);

                /*
                stats.push((
                    (size, *density, iteration),
                    (elapsed, ratio_zeroes, ratio_clean_values),
                ));

                 */

                // write to file scope
                {
                    let mut file = OpenOptions::new()
                        .write(true)
                        .append(true)
                        .open(filename)
                        .unwrap();

                    let line = format!(
                        "{},{},{},{},{},{},{}\n",
                        counter,
                        size,
                        *density,
                        iteration,
                        elapsed,
                        ratio_zeroes,
                        ratio_clean_values,
                    );

                    let a = write!(file, "{}", line);
                    match a {
                        Ok(_) => println!("line written succeffuly"),
                        Err(e) => println!("could't write line: {}", &e),
                    }
                }
            }
        }
    }
}

pub fn compute_all_benches() {
    // some test
    // some_test();

    // prepare file:
    // first remove file
    let _o = fs::remove_file(bench_time_file_path);

    println!("writting header line to {}", &bench_time_file_path);
    {
        let mut file = File::create(bench_time_file_path).unwrap();
        write!(file, "id,chain,depth,tbis,iteration,abis,verify_tbox,unravel_tbox,verify_abox,unravel_abox,conflict_matrix,rank_abox\n");
    }

    let entries = fs::read_dir(onto_path)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();

    let length = entries.len();

    let mut counter: usize = 0;

    // this vector is as: (id, chain, depth, tbis, iteration, abis, tbox verification time, tbox unraveling time,
    //                          abox unraveling time, conflict matrix generation time, bound computing time)
    let mut time_hashmap: HashMap<
        (usize, usize, usize, usize, usize, usize),
        (f64, f64, f64, f64, f64, f64),
    > = HashMap::new();

    for p in entries {
        let p_str = p.to_str().unwrap();
        let onto_name = extract_onto_from_path(p_str);
        let (c, d, tbi, i) = extract_from_onto_name(&onto_name);

        // println!("path: {:?}, onto: {}, c: {}, d: {}, tbi: {}, i: {}", &p, onto_name, c, d, tbi, i);

        let mut tbox_name = onto_name.clone();
        tbox_name.push_str(".txt");

        let mut path_to_aboxes = p.clone();
        path_to_aboxes.push("aboxes");

        let mut path_to_tbox = p.clone();
        path_to_tbox.push(tbox_name.clone());

        // println!("path to aboxes: {:?}", &path_to_aboxes);
        // println!("path to tbox: {:?}", &path_to_tbox);

        let inner_entries = fs::read_dir(path_to_aboxes)
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()
            .unwrap();

        println!("===============================================================================");
        println!("taking measure for tbox: {}", &tbox_name);

        // tbox related measures are to be done before

        // create the ontology struct and add symbols
        let mut onto = OntologyDllite::new(tbox_name.clone());
        onto.add_symbols_from_file(symbols_path, all_file_type, verbose);

        // read the tbox
        let path_to_tbox = path_to_tbox.to_str().unwrap();
        onto.add_tbis_from_file(&path_to_tbox, all_file_type, verbose);

        // measure verify tbox
        let deduction_tree = false;
        let negative_only = 1_i8;
        let which_closure = true;

        let now = Instant::now();

        onto.generate_cln(deduction_tree, verbose, negative_only);
        let full_closure = onto.cln(which_closure);

        let mut tbox_contradiction_counter: usize = 0;
        for tbi in full_closure.items() {
            if tbi.is_contradiction() && !tbi.is_trivial() {
                tbox_contradiction_counter += 1;
                break;
            }
        }

        // just after we get an answer
        let verify_tbox_time = now.elapsed().as_secs_f64();

        if tbox_contradiction_counter > 0 {
            println!("  --  INFO: contradiction where found");
        }

        println!("  --  verify tbox time:   {}", verify_tbox_time);
        // end of verify tbox

        // measure unravel tbox time
        let deduction_tree = true;
        let _dont_write_trivial = true;

        let now = Instant::now();

        onto.generate_cln(deduction_tree, verbose, negative_only);
        let _full_closure = onto.cln(which_closure);

        let unravel_tbox_time = now.elapsed().as_secs_f64();

        println!("  --  unravel tbox time:  {}", unravel_tbox_time);
        // end of unravel tbox

        // end of tbox related tasks

        // here we create the tbox that we need for the other tasks
        let mut onto = OntologyDllite::new(tbox_name.clone());
        onto.add_symbols_from_file(symbols_path, all_file_type, verbose);
        onto.add_tbis_from_file(&path_to_tbox, all_file_type, verbose);

        onto.generate_cln(false, false, 0_i8);
        let neg_closure = onto.cln(false).clone();
        let _full_closure = onto.cln(true).clone();

        // now this tbox is complete for the task

        for inner_p in inner_entries {
            let inner_p_str = inner_p.to_str().unwrap();
            let (abox_name, a) = extract_abox_from_path(inner_p_str);

            if a >= 2000 {
                continue;
            }

            let path_to_abox = String::from(inner_p_str);

            // println!("    aboxes: {:?}, abox name: {}, a: {}", &inner_p, &abox_name, a);
            let clone_tbox_name = tbox_name.clone();

            let _value = (
                counter,
                String::from(p.to_str().unwrap()),
                String::from(path_to_tbox),
                clone_tbox_name,
                String::from(inner_p.to_str().unwrap()),
                String::from(abox_name.clone()),
                c,
                d,
                tbi,
                i,
                a,
            );

            let tbox_abox_complexity = a * tbi;
            let tbox_abox_complexity = (tbox_abox_complexity as f64) / (COMPLEXITY_LIMIT as f64);

            if tbox_abox_complexity <= 1_f64 {
                println!(
                    "  ---------------------------------------------------------------------------"
                );
                println!(
                    "  ----  analysing abox: {} with {} assertions\ncomplexity of {} in range [0, 1] (LIMIT IS: {})",
                    &abox_name, a, tbox_abox_complexity, COMPLEXITY_LIMIT
                );

                counter += 1;

                // println!("{:?}", onto.symbols());

                // measure verify abox
                onto.new_abox_from_file_quantum(&path_to_abox, all_file_type, verbose);

                let abox = onto.abox().unwrap();
                let _deduction_tree = false;

                let now = Instant::now();

                let (abox_is_inconsistent, _) =
                    AbqDllite::is_inconsistent_refs_only(abox.items_by_ref(), &neg_closure, false);

                let verify_abox_time = now.elapsed().as_secs_f64();

                if abox_is_inconsistent {
                    println!("    --  INFO: abox is inconsistent");
                }

                println!("    --  verify abox time: {}", verify_abox_time);
                // end verify abox

                // measure unravel abox
                onto.new_abox_from_file_quantum(&path_to_abox, all_file_type, verbose);
                let _abox = onto.abox().unwrap();

                let _deduction_tree = true;

                let now = Instant::now();

                /*
                for the moment I'm forgetting this one, is really expensive and only exploratory
                let abox_complete = abox.complete(&full_closure, true, false);
                 */

                let unravel_abox_time = now.elapsed().as_secs_f64();

                println!("    --  unravel abox time: {}", unravel_abox_time);
                // end unravel abox

                // measure conflict matrix
                onto.new_abox_from_file_quantum(&path_to_abox, all_file_type, verbose);
                let _abox = onto.abox().unwrap();

                let _deduction_tree = false;

                let now = Instant::now();

                let (_before_matrix, _real_to_virtual, _virtual_to_real) =
                    onto.conflict_matrix_refs_only(onto.abox().unwrap(), false);

                let conflict_matrix_time = now.elapsed().as_secs_f64();

                println!("    --  conflict matrix time: {}", conflict_matrix_time);
                // end conflict matrix

                // measure rank abox
                onto.new_abox_from_file_quantum(&path_to_abox, all_file_type, verbose);
                let mut abox = onto.abox().unwrap().clone();

                let deduction_tree = false;
                let aggr = AGGR_SUM;

                let now = Instant::now();

                // to remove the matrix computation from the rank computation
                let (_before_matrix, _real_to_virtual, _virtual_to_real) =
                    onto.conflict_matrix_refs_only(onto.abox().unwrap(), false);

                let conflict_matrix_time_prov = now.elapsed().as_secs_f64();

                let adjusters: Adjusters = (TOLERANCE, M_SCALER, B_TRANSLATE);
                let (_before_matrix, _virtual_to_real, _conflict_type) =
                    rank_abox(&onto, &mut abox, deduction_tree, aggr, adjusters, verbose);

                let rank_abox_time = now.elapsed().as_secs_f64() - conflict_matrix_time_prov;

                println!("    --  rank abox time: {}", rank_abox_time);
                // end rank abox

                // here I add the values to the hash
                time_hashmap.insert(
                    (counter, c, d, tbi, i, a),
                    (
                        verify_tbox_time,
                        unravel_tbox_time,
                        verify_abox_time,
                        unravel_abox_time,
                        conflict_matrix_time,
                        rank_abox_time,
                    ),
                );
                println!(
                    "added the following to hash: {:?} : {:?}",
                    (counter, c, d, tbi, i, a),
                    (
                        verify_tbox_time,
                        unravel_tbox_time,
                        verify_abox_time,
                        unravel_abox_time,
                        conflict_matrix_time,
                        rank_abox_time
                    )
                );

                // write to file scope
                {
                    let mut file = OpenOptions::new()
                        .write(true)
                        .append(true)
                        .open(bench_time_file_path)
                        .unwrap();

                    let line = format!(
                        "{},{},{},{},{},{},{},{},{},{},{},{}\n",
                        counter,
                        c,
                        d,
                        tbi,
                        i,
                        a,
                        verify_tbox_time,
                        unravel_tbox_time,
                        verify_abox_time,
                        unravel_abox_time,
                        conflict_matrix_time,
                        rank_abox_time
                    );

                    let a = write!(file, "{}", line);
                    match a {
                        Ok(_) => println!("line written succeffuly"),
                        Err(e) => println!("could't write line: {}", &e),
                    }
                }
            }
        }
    }

    println!("there are {} ontologies", length);
}

pub fn extract_abox_from_path(s: &str) -> (String, usize) {
    let reg = Regex::new(r"\w*/\w*/\w*/\w*/(\w*_a(\d*).txt)").unwrap();

    let mut v: Vec<(String, usize)> = Vec::new();

    for cap in reg.captures_iter(s) {
        // println!("{:?}", &cap);

        v.push((String::from(&cap[1]), usize::from_str(&cap[2]).unwrap()));
        break;
    }

    v[0].clone()
}

// if there is a match then is the first one
pub fn extract_from_onto_name(s: &str) -> (usize, usize, usize, usize) {
    let reg = Regex::new(r"_c(\d*)_d(\d*)_tbi(\d*)_i(\d*)").unwrap();

    let mut v: Vec<(usize, usize, usize, usize)> = Vec::new();

    for cap in reg.captures_iter(s) {
        // only one needed
        let value = (
            usize::from_str(&cap[1]).unwrap(),
            usize::from_str(&cap[2]).unwrap(),
            usize::from_str(&cap[3]).unwrap(),
            usize::from_str(&cap[4]).unwrap(),
        );
        v.push(value);
        break;
    }

    v[0]
}

pub fn extract_onto_from_path(s: &str) -> String {
    let reg = Regex::new(r"\w*/\w*/(\w*)").unwrap();

    let mut v: Vec<String> = Vec::new();

    for cap in reg.captures_iter(s) {
        v.push(String::from(&cap[1]));
        break;
    }

    v[0].clone()
}
