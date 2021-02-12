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

pub fn parse_symbols_native(
    filename: &str,
    verbose: bool,
) -> io::Result<HashMap<String, (usize, DLType)>> {
    let file_result = File::open(filename);

    match file_result {
        Err(e) => {
            println!("couldn't read the file: {}", e);
            Result::Err(e)
        }
        Ok(file) => {
            let reader = BufReader::new(file);

            let mut begin_symbol_encountered = false;
            let mut end_symbol_encountered = false;

            let mut symbols: HashMap<String, (usize, DLType)> = HashMap::new();
            let mut unsorted_symbols: Vec<PS> = Vec::new();

            for line_result in reader.lines() {
                match line_result {
                    Err(e) => {
                        if verbose {
                            println!("passing this line: {}", e);
                        }
                    }
                    Ok(line) => {
                        if verbose {
                            println!("trying to parse: {}", &line);
                        }

                        let line_trimmed = line.trim();

                        if line_trimmed == "BEGINSYMBOL" {
                            begin_symbol_encountered = true;

                            if verbose {
                                println!("'BEGINSYMBOL' found, begin parsing");
                            }

                            continue;
                        }

                        if line_trimmed == "ENDSYMBOL" {
                            end_symbol_encountered = true;

                            if verbose {
                                println!("'ENDSYMBOL' found, ending parsing");
                            }
                            continue;
                        }

                        if begin_symbol_encountered && !end_symbol_encountered {
                            let mut vec: Vec<&str> = line_trimmed.split("//").collect();

                            let not_ignored = vec[0].clone().trim();

                            if not_ignored == "BEGINSYMBOL" {
                                begin_symbol_encountered = true;

                                if verbose {
                                    println!("'BEGINSYMBOL' found, begin parsing");
                                }

                                continue;
                            }

                            if not_ignored == "ENDSYMBOL" {
                                end_symbol_encountered = true;

                                if verbose {
                                    println!("'ENDSYMBOL' found, ending parsing");
                                }
                                continue;
                            }

                            if verbose {
                                let ignored: String = String::from(vec[1..].join("//").trim());

                                if &ignored != "" {
                                    println!("this comment will be ignored: {}", &ignored);
                                }
                            }

                            let parsed: io::Result<(&str, DLType)> = string_to_symbol(&not_ignored);

                            match parsed {
                                Ok((name, t)) => {
                                    let new_ps = PS::new(String::from(name), t);
                                    unsorted_symbols.push(new_ps);

                                    if verbose {
                                        println!(
                                            "result of parsing:   name: {}, type: {}",
                                            name, t
                                        );
                                    }
                                }
                                Err(e) => {
                                    if verbose {
                                        println!("couldn't parse: {}", e);
                                    }
                                }
                            }
                        } else {
                            if verbose {
                                println!("line won't be parsed, not in between 'BEGINSYMBOL' and 'ENDSYMBOL' bounds");
                            }
                        }
                    }
                }
            }

            if !end_symbol_encountered {
                if verbose {
                    println!("'ENDSYMBOL' not encountered, returning nothing");
                }
                println!("putaindljfldfldjf-----------------");
                let new_error = Error::new(
                    ErrorKind::UnexpectedEof,
                    "'ENDSYMBOL' not found before file ended",
                );

                Result::Err(new_error)
            } else {
                // here I need to sort unsorted_symbols
                unsorted_symbols.sort();

                // symbols sorted, so now I put everyting in symbols
                symbols.insert(String::from("Bottom"), (0, DLType::Bottom));
                symbols.insert(String::from("Top"), (1, DLType::Top));

                let unsorted_size = unsorted_symbols.len();

                for i in 0..unsorted_size {
                    let s = &unsorted_symbols[i];

                    let name = s.name();
                    let t = s.t();

                    symbols.insert(String::from(name), (i + 2, t));
                }
                Result::Ok(symbols)
            }
        }
    }
}

pub fn parse_abox_native(
    filename: &str,
    symbols: &mut HashMap<String, (usize, DLType)>,
    verbose: bool,
) -> io::Result<AB> {
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

            let mut ab = AB::new(ab_name);

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
                            let mut vec: Vec<&str> = line_trimmed.split("//").collect();

                            let not_ignored = vec[0].clone();

                            if verbose {
                                let ignored: String = String::from(vec[1..].join("//").trim());

                                if &ignored != "" {
                                    println!("this comment will be ignored: {}", &ignored);
                                }
                            }

                            let (parsed_result, current_id_result) =
                                string_to_abi(&not_ignored, symbols, current_id, false); // parsing from file should be a new abox
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

pub fn parse_tbox_native(
    filename: &str,
    symbols: &HashMap<String, (usize, DLType)>,
    verbose: bool,
) -> io::Result<TB> {
    let file_result = File::open(filename);

    match file_result {
        Err(e) => {
            println!("couldn't read the file: {}", e);
            Result::Err(e)
        }
        Ok(file) => {
            let reader = BufReader::new(file);

            let mut begin_tbox_encountered = false;
            let mut end_tbox_encountered = false;

            let mut tb = TB::new();

            for line_result in reader.lines() {
                match line_result {
                    Err(e) => {
                        if verbose {
                            println!("passing this line: {}", e);
                        }
                    }
                    Ok(line) => {
                        if verbose {
                            println!("trying to parse: {}", &line);
                        }

                        let line_trimmed = line.trim();

                        if line_trimmed == "BEGINTBOX" {
                            begin_tbox_encountered = true;

                            if verbose {
                                println!("'BEGINTBOX' found, begin parsing");
                            }

                            continue;
                        }

                        if line_trimmed == "ENDTBOX" {
                            end_tbox_encountered = true;

                            if verbose {
                                println!("'ENDTBOX' found, ending parsing");
                            }
                            continue;
                        }

                        if begin_tbox_encountered && !end_tbox_encountered {
                            let mut vec: Vec<&str> = line_trimmed.split("//").collect();

                            let not_ignored = vec[0].clone().trim();

                            if not_ignored == "BEGINTBOX" {
                                begin_tbox_encountered = true;

                                if verbose {
                                    println!("'BEGINTBOX' found, begin parsing");
                                }

                                continue;
                            }

                            if not_ignored == "ENDTBOX" {
                                end_tbox_encountered = true;

                                if verbose {
                                    println!("'ENDTBOX' found, ending parsing");
                                }
                                continue;
                            }

                            if verbose {
                                let ignored: String = String::from(vec[1..].join("//").trim());

                                if &ignored != "" {
                                    println!("this comment will be ignored: {}", &ignored);
                                }
                            }

                            let parsed = string_to_tbi(&not_ignored, symbols);

                            match parsed {
                                Ok(mut tbi_vec) => {
                                    if !(&tbi_vec).is_empty() {
                                        while !(&tbi_vec).is_empty() {
                                            let tbi = tbi_vec.pop().unwrap();
                                            tb.add(tbi);
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

            if !end_tbox_encountered {
                if verbose {
                    println!("'ENDTBOX' not encountered, returning nothing");
                }

                let new_error = Error::new(
                    ErrorKind::UnexpectedEof,
                    "'ENDTBOX' not found before file ended",
                );

                Result::Err(new_error)
            } else {
                Result::Ok(tb)
            }
        }
    }
}

pub fn tbox_to_native_string(
    tbox: &TB,
    symbols: &HashMap<String, (usize, DLType)>,
    dont_write_trivial: bool,
) -> Option<String> {
    let mut res = String::new();

    // I should define a header for this files
    let header = "";
    res.push_str(header);

    res.push_str("BEGINTBOX\n");

    for tbi in tbox.items() {
        if !(tbi.is_trivial() && dont_write_trivial) {
            let tbi_str_op = tbi_to_string(tbi, symbols);

            match tbi_str_op {
                Some(tbi_str) => {
                    res.push_str(tbi_str.as_str());
                    res.push_str("\n");
                }
                _ => (),
            }
        }
    }

    res.push_str("ENDTBOX\n");
    Some(res)
}

pub fn abox_to_native_string(
    abox: &AB,
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
            let abi_str_op = abi_to_string(abi, symbols);

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

pub fn find_bound_of_symbols(symbols: &HashMap<String, (usize, DLType)>) -> (usize, usize) {
    if symbols.is_empty() {
        (0, 0)
    } else {
        let mut lower: Option<usize> = Option::None;
        let mut upper: Option<usize> = Option::None;

        for (_, (id, _)) in symbols {
            lower = match lower {
                Option::None => Some(*id),
                Some(old_id) => {
                    if old_id > *id {
                        Some(*id)
                    } else {
                        Some(old_id)
                    }
                }
            };

            upper = match upper {
                Option::None => Some(*id),
                Some(old_id) => {
                    if old_id < *id {
                        Some(*id)
                    } else {
                        Some(old_id)
                    }
                }
            };
        }

        (lower.unwrap(), upper.unwrap())
    }
}
