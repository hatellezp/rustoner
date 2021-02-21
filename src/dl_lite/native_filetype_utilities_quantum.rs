use crate::dl_lite::types::DLType;
// use std::cmp::Ordering;
// use std::collections::{HashMap, VecDeque};
use crate::dl_lite::abox::AB;
use crate::dl_lite::string_formatter::{
    abi_to_string, string_to_abi, string_to_symbol, string_to_tbi, tbi_to_string, PS,
};
use crate::dl_lite::tbox::TB;
use crate::interface::utilities::parse_name_from_filename;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use crate::dl_lite::native_filetype_utilities::find_bound_of_symbols;
use crate::dl_lite::string_formatter_quantum::{string_to_abiq, abiq_to_string};
use crate::dl_lite::abox_quantum::ABQ;

pub fn parse_abox_native_quantum(
    filename: &str,
    symbols: &mut HashMap<String, (usize, DLType)>,
    verbose: bool,
) -> io::Result<ABQ> {
    /*
    this function might add nominal symbols dynamically, so we need to actuallize symbols :/
    dangerous manipulation here...
     */
    let file_result = File::open(filename);

    match file_result {
        Err(e) => {
            if verbose {
                println!("couldn't read the file: {}", e);
            }
            Result::Err(e)
        }
        Ok(file) => {
            let reader = BufReader::new(file);

            let ab_name = parse_name_from_filename(filename);

            let mut begin_abox_encountered = false;
            let mut end_abox_encountered = false;

            let (_, id_bound) = find_bound_of_symbols(symbols);
            let mut current_id = id_bound + 1;

            let mut ab = ABQ::new(ab_name);

            for line_result in reader.lines() {
                if verbose {
                    println!("now parsing: {:?}", line_result);
                }

                match line_result {
                    Err(e) => {
                        if verbose {
                            println!("passing this line because of: {}", &e);
                        }
                    }
                    Ok(line) => {
                        let line_trimmed = line.trim();

                        if line_trimmed == "BEGINABOX" {
                            begin_abox_encountered = true;

                            if verbose {
                                println!("'BEGINABOX' found, begin parsing");
                            }

                            continue;
                        }

                        if line_trimmed == "ENDABOX" {
                            end_abox_encountered = true;

                            if verbose {
                                println!("'ENDABOX' found, ending parsing");
                            }
                            continue;
                        }

                        if begin_abox_encountered && !end_abox_encountered {
                            let vec: Vec<&str> = line_trimmed.split("//").collect();

                            let not_ignored = vec[0].clone();

                            if verbose {
                                let ignored: String = String::from(vec[1..].join("//").trim());

                                if &ignored != "" {
                                    println!("this comment will be ignored: {}", &ignored);
                                }
                            }

                            let (parsed_result, current_id_result) =
                                string_to_abiq(&not_ignored, symbols, current_id, false); // parsing from file should be a new abox
                            current_id = current_id_result;

                            match parsed_result {
                                Ok((abi, mut to_be_added)) => {
                                    ab.add(abi);

                                    if !to_be_added.is_empty() {
                                        while !(&to_be_added).is_empty() {
                                            let (s, (id, dltype)) = to_be_added.pop().unwrap();

                                            symbols.insert(s, (id, dltype));
                                        }
                                    }
                                }
                                Err(e) => {
                                    if verbose {
                                        println!("couldn't parse: {}", &e);
                                    }
                                }
                            }
                        } else {
                            if verbose {
                                println!("line won't be parsed, not in between 'BEGINTBOX' and 'ENDTBOX' bounds");
                            }
                        }
                    }
                }
            }

            if !end_abox_encountered {
                if verbose {
                    println!("'ENDTBOX' not encountered, returning nothing");
                }

                let new_error = Error::new(
                    ErrorKind::UnexpectedEof,
                    "'ENDABOX' not found before file ended",
                );

                Result::Err(new_error)
            } else {
                Result::Ok(ab)
            }
        }
    }
}

pub fn abox_to_native_string_quantum(
    abox: &ABQ,
    symbols: &HashMap<String, (usize, DLType)>,
    dont_write_trivial: bool,
) -> Option<String> {
    let mut res = String::new();

    // I should define a header for this files
    let header = "";
    res.push_str(header);

    res.push_str("BEGINABOX\n");

    for abi in abox.items() {
        if !(abi.is_trivial() && dont_write_trivial) {
            let abi_str_op = abiq_to_string(abi, symbols);

            match abi_str_op {
                Some(abi_str) => {
                    res.push_str(abi_str.as_str());
                    res.push_str("\n");
                }
                _ => (),
            }
        }
    }

    res.push_str("ENDABOX\n");
    Some(res)
}
