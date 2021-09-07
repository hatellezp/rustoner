use rustoner::dl_lite::abox::AbqDllite;
use rustoner::dl_lite::abox_item_quantum::AbiqDllite;
use rustoner::dl_lite::tbox::TBDllite;
use rustoner::kb::types::FileType;
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::str::FromStr;
use regex::Regex;

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
}

pub fn bench_ontology_related() {
    let filename_without_extension = "benchmark_ontology_related";
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

            let _ = writeln!(
                file,
                "tb_size, tb_depth, abox_size, gencontb, vertb, verab, genconab, cleanab, matrix_building, rankab"
            );
        }
    }



    let tboxes: Vec<TBDllite> = Vec::new();

    for tb in tboxes.iter() {

        // do vertb


        // do gencontb


        let aboxes: Vec<AbqDllite> = Vec::new();

        for abox in aboxes.iter() {

            // do verab

            // do cleanab

            // do genconab

            // do matrix building

            // do rankab
        }
    }
}

pub fn bench_ontology_related_one_line(filename: &str) {
    let tb_size_inner = 0_usize;
    let tb_depth_inner = 0_usize;
    let abox_size_inner = 0_usize;

    let gencontb_time = 0_f64;
    let vertb_time = 0_f64;
    let verab_time = 0_f64;
    let genconab_time = 0_f64;
    let cleanab_time = 0_f64;
    let matrix_building_time = 0_f64;
    let rankab_time = 0_f64;

    // here we append to file
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(filename)
        .unwrap();

    let line = format!(
        "{}, {}, {}, {}, {}, {}, {} , {}, {}, {}",
        tb_size_inner,
        tb_depth_inner,
        abox_size_inner,
        gencontb_time,
        vertb_time,
        verab_time,
        genconab_time,
        cleanab_time,
        matrix_building_time,
        rankab_time
    );

    let _ = writeln!(file, "{}", line);
}

pub fn bench_gencontb(tb: TBDllite) {}

pub fn bench_vertb(tb: TBDllite) {}

pub fn bench_verab(ab: AbqDllite, tb: TBDllite) {}

pub fn bench_genconab(ab: AbqDllite, tb: TBDllite) {}

pub fn bench_cleanab(ab: AbqDllite, tb: TBDllite) {}

pub fn bench_matrix_building(ab: AbqDllite, tb: TBDllite) {}

pub fn bench_rankab(ab: AbqDllite, tb: TBDllite) {}

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


