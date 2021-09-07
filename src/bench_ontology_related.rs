mod alg_math;

mod benching;
mod dl_lite;
mod helper;
mod interface;
mod kb;

use rayon::prelude::*;

use regex::Regex;
use crate::dl_lite::abox::AbqDllite;
use crate::dl_lite::abox_item_quantum::AbiqDllite;
use crate::dl_lite::tbox::TBDllite;
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::str::FromStr;
use std::{fs, io};

use crate::dl_lite::ontology::OntologyDllite;

use crate::helper::rank_abox;

use crate::kb::aggr_functions::*;
use crate::kb::knowledge_base::{ABox, TBox, TBoxItem};
use crate::kb::types::FileType;

use crate::alg_math::bounds::{find_bound_complex_wrapper, Adjusters};

use std::io::Write;
use std::time::Instant;
use pad::PadStr;
use std::cmp::Ordering;

const onto_path: &str = "onto/";
const symbols_path: &str = "onto/symbols.txt";
const all_file_type: FileType = FileType::Native;
const verbose: bool = false;

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
    println!("hello there!!!");

    bench_ontology_related();
}

pub fn bench_ontology_related() {

    println!("===============================================");
    println!("benching\n - benching of ontology related tasks");
    println!("===============================================");

    let filename_without_extension = "benchmark_ontology_related";
    let line_to_write = "tb_size, tb_depth, tb_chain, abox_size, gencontb, vertb, verab, genconab, cleanab, matrix_building, rankab";

    let filename = create_csv_file(filename_without_extension, line_to_write);


    let header = "| tbox size       | tbox depth      | tbox chain      | abox size       | gencon tbox     | ver tbox        | ver abox        | gencon abox     | clean abox      | matrix building | rank abox       |";
    let header_length = header.len();
    let line_divisor = vec!["-"; header_length].join("");

    let mut entries = fs::read_dir(onto_path)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();

    entries.par_sort_by(|x, y| {
        let x_str = x.to_str().unwrap();
        let y_str = y.to_str().unwrap();

        if x_str.contains("symbols") {
            return Ordering::Greater
        }

        if y_str.contains("symbols") {
            return Ordering::Less
        }

        let x_name = extract_onto_from_path(x_str);
        let y_name = extract_onto_from_path(y_str);

        let (_, _, tx, _) = extract_from_onto_name(&x_name);
        let (_, _, ty, _) = extract_from_onto_name(&y_name);

        tx.cmp(&ty)
    });

    println!("done sorting: {:?}", &entries[0..2]);


    println!("-------------------------");
    println!("| ontology related tasks |");

    println!("{}", line_divisor);
    println!("{}", header);
    println!("{}", line_divisor);

    for p in entries.iter() {
        // extract information from path
        let p_str = p.to_str().unwrap();

        if p_str.contains("symbols") {
            continue;
        }

        let onto_name = extract_onto_from_path(p_str);
        let (chain, depth, tbis, _) = extract_from_onto_name(&onto_name);

        let mut tbox_name = onto_name.clone();
        tbox_name.push_str(".txt");

        let mut path_to_tbox = p.clone();
        path_to_tbox.push(tbox_name.clone());

        // extract information for aboxes
        let mut path_to_aboxes = p.clone();
        path_to_aboxes.push("aboxes");

        let mut inner_entries = fs::read_dir(path_to_aboxes)
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()
            .unwrap();

        // tbox related measures are to be done before
        // first declare all the values that you need
        let mut vertb_time = 0_f64;
        let mut gencontb_time = 0_f64;
        let mut verab_time = 0_f64;
        let mut genconab_time = 0_f64;
        let mut cleanab_time = 0_f64;
        let mut matrix_building_time = 0_f64;
        let mut rankab_time = 0_f64;

        /*
           verify tbox time
        */

        // create the ontology struct and add symbols
        let mut onto = OntologyDllite::new(tbox_name.clone());
        onto.add_symbols_from_file(symbols_path, all_file_type, verbose);

        // read the tbox
        let path_to_tbox = path_to_tbox.to_str().unwrap();
        onto.add_tbis_from_file(&path_to_tbox, all_file_type, verbose);

        // measure verify tbox
        // initialize measure for the time
        let mut now = Instant::now();

        onto.generate_cln(false, false, 1_i8);
        let full_closure = onto.cln(true);

        let mut tbox_contradiction_counter: usize = 0;
        for tbi in full_closure.items() {
            if tbi.is_contradiction() && !tbi.is_trivial() {
                tbox_contradiction_counter += 1;
                break;
            }
        }

        // just after we get an answer
        vertb_time = now.elapsed().as_secs_f64();

        /*
           end of measure verify tbox time
        */

        // measure unravel tbox time

        let now = Instant::now();

        onto.generate_cln(true, false, 1_i8);
        let _full_closure = onto.cln(true);

        gencontb_time = now.elapsed().as_secs_f64();

        /*
           end of gencontb measure time
        */

        // here we create the tbox that we need for the other tasks
        // a sole tbox with both positive and negative closures

        let mut onto = OntologyDllite::new(tbox_name.clone());
        onto.add_symbols_from_file(symbols_path, all_file_type, verbose);
        onto.add_tbis_from_file(&path_to_tbox, all_file_type, verbose);

        onto.generate_cln(false, false, 0_i8);
        let neg_closure = onto.cln(false).clone();
        let full_closure = onto.cln(true).clone();

        /*
           here we begin the measure of abox tasks
        */

        inner_entries.par_sort_by(|x, y| {
            let x_str = x.to_str().unwrap();
            let y_str = y.to_str().unwrap();


            let (_, ax) = extract_abox_from_path(x_str);
            let (_, ay) = extract_abox_from_path(y_str);

            ax.cmp(&ay)
        });

        for inner_p in inner_entries {
            let inner_p_str = inner_p.to_str().unwrap();
            let (_, assertion_number) = extract_abox_from_path(inner_p_str);

            if assertion_number > 2000 {
                continue;
            }

            let path_to_abox = String::from(inner_p_str);
            let clone_tbox_name = tbox_name.clone();

            let tbox_abox_complexity = assertion_number * tbis;
            let tbox_abox_complexity = (tbox_abox_complexity as f64) / (COMPLEXITY_LIMIT as f64);

            if tbox_abox_complexity <= 1_f64 {
                /*
                   verify abox measure time
                */

                onto.new_abox_from_file_quantum(&path_to_abox, all_file_type, verbose);
                let abox = onto.abox().unwrap();

                let now = Instant::now();

                let (abox_is_inconsistent, _) =
                    AbqDllite::is_inconsistent_refs_only(abox.items_by_ref(), &neg_closure, false);

                verab_time = now.elapsed().as_secs_f64();

                /*
                   end of verab measure time
                */

                /*

                    // I'm not doing this, argument that it is not necessary and do not
                    // talks about the quality of the algorithm

                /*
                   genconab measure time
                */

                onto.new_abox_from_file_quantum(&path_to_abox, all_file_type, verbose);
                let abox = onto.abox().unwrap();

                let now = Instant::now();

                let abox_complete = abox.complete(&full_closure, true, false);

                genconab_time = now.elapsed().as_secs_f64();

                /*
                   end of genconab measure time
                */

                 */

                /*
                   begin matrix building measure time
                */

                onto.new_abox_from_file_quantum(&path_to_abox, all_file_type, verbose);
                let _abox = onto.abox().unwrap();

                let now = Instant::now();

                let (_before_matrix, _real_to_virtual, _virtual_to_real) =
                    onto.conflict_matrix_refs_only(onto.abox().unwrap(), false);

                matrix_building_time = now.elapsed().as_secs_f64();

                /*
                   end of matrix building measure time
                */

                /*
                   begin of rankab measure time
                */
                onto.new_abox_from_file_quantum(&path_to_abox, all_file_type, verbose);
                let mut abox = onto.abox().unwrap().clone();

                let aggr = AGGR_SUM;

                let now = Instant::now();

                // to remove the matrix computation from the rank computation
                let (_before_matrix, _real_to_virtual, _virtual_to_real) =
                    onto.conflict_matrix_refs_only(onto.abox().unwrap(), false);

                let adjusters: Adjusters = (TOLERANCE, M_SCALER, B_TRANSLATE);
                let (_before_matrix, _virtual_to_real, _conflict_type) =
                    rank_abox(&onto, &mut abox, false, aggr, adjusters, false, true);

                rankab_time = now.elapsed().as_secs_f64();

                /*
                   end of rankab measure time
                */

                let line_to_write = format!(
                    "{},{},{},{},{},{},{},{},{},{},{}",
                    tbis,
                    depth,
                    chain,
                    assertion_number,
                    gencontb_time,
                    vertb_time,
                    verab_time,
                    genconab_time,
                    cleanab_time,
                    matrix_building_time,
                    rankab_time
                );

                // here we append to file
                let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(&filename)
                    .unwrap();
                writeln!(file, "{}", line_to_write);

                let width: usize = 15;
                let tbis_str = format!("{}", tbis).pad_to_width(width);
                let depth_str = format!("{}", depth).pad_to_width(width);
                let chain_str = format!("{}", chain).pad_to_width(width);
                let assertion_number_str = format!("{}", assertion_number).pad_to_width(width);

                let gencontb_time_str = format!("{:.9}", gencontb_time).pad_to_width(width);
                let vertb_time_str = format!("{:.9}", vertb_time).pad_to_width(width);
                let verab_time_str = format!("{:.9}", verab_time).pad_to_width(width);
                let genconab_time_str = format!("{:.9}", genconab_time).pad_to_width(width);
                let cleanab_time_str = format!("{:.9}", cleanab_time).pad_to_width(width);
                let matrix_building_time_str = format!("{:.9}", matrix_building_time).pad_to_width(width);
                let rankab_time_str = format!("{:.9}", rankab_time).pad_to_width(width);


                println!(
                    "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |",
                    tbis_str,
                    depth_str,
                    chain_str,
                    assertion_number_str,
                    gencontb_time_str,
                    vertb_time_str,
                    verab_time_str,
                    genconab_time_str,
                    cleanab_time_str,
                    matrix_building_time_str,
                    rankab_time_str
                );


            }
        }

        println!("{}", line_divisor);
    }
}

pub fn create_csv_file(filename: &str, line_to_write: &str) -> String {
    let filename_without_extension = filename;
    let mut extension: Option<usize> = None;
    let mut possible_filename = String::new();
    let mut found_next = false;

    {
        while !found_next {
            possible_filename = if extension.is_none() {
                format!("{}.csv", filename_without_extension)
            } else {
                format!("{}{}.csv", filename_without_extension, extension.unwrap())
            };

            if Path::new(&possible_filename).exists() {
                if extension.is_none() {
                    extension = Some(1);
                } else {
                    extension = Some(extension.unwrap() + 1);
                }

                continue;
            } else {
                found_next = true;
                break;
            }
        }

        {
            let mut file = File::create(Path::new(&possible_filename)).unwrap();

            let _ = writeln!(file, "{}", line_to_write);
        }
    }

    String::from(&possible_filename)
}

pub fn extract_abox_from_path(s: &str) -> (String, usize) {
    let reg = Regex::new(r"\w*/\w*/\w*/(\w*_a(\d*).txt)").unwrap();

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
    let reg = Regex::new(r"\w*/(\w*)").unwrap();

    let mut v: Vec<String> = Vec::new();

    for cap in reg.captures_iter(s) {
        v.push(String::from(&cap[1]));
        break;
    }

    v[0].clone()
}
