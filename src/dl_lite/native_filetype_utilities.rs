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

use crate::dl_lite::abox::AbqDllite;
use crate::dl_lite::string_formatter::{
    abiq_to_string, string_to_abiq, string_to_symbol, string_to_tbi, tbi_to_string, PS,
};
use crate::dl_lite::tbox::TBDllite;
use crate::interface::utilities::parse_name_from_filename;
use crate::kb::types::DLType;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, ErrorKind};

use crate::kb::knowledge_base::{ABox, SymbolDict, TBox, TBoxItem};

/*
   in my opinion this functions are too differents to try and do only one,
   what can I do is try to accelerate them
*/

pub fn parse_symbols_native(filename: &str, verbose: bool) -> io::Result<SymbolDict> {
    let file_result = File::open(filename);

    match file_result {
        Err(e) => {
            if verbose {
                println!(
                    " -- native_utilities::parse_symbols_native: couldn't read the file: {}",
                    e
                );
            }

            Result::Err(e)
        }
        Ok(file) => {
            let mut reader = BufReader::new(file);

            let mut begin_symbol_encountered = false;
            let mut end_symbol_encountered = false;

            let mut symbols: SymbolDict = HashMap::new();
            let mut unsorted_symbols: Vec<PS> = Vec::new();

            let mut buffer = String::new();

            loop {
                match reader.read_line(&mut buffer) {
                    Ok(bytes_read) => {
                        // if end of file break!
                        if bytes_read == 0 {
                            break;
                        } else {
                            let line = &buffer;
                            // all the code goes here
                            if verbose {
                                println!(
                                    " -- native_utilities::parse_symbols_native: trying to parse: {}",
                                    &line
                                );
                            }

                            // let line_trimmed = line.trim();
                            let line_trimmed = buffer.trim();

                            if line_trimmed == "BEGINSYMBOL" {
                                begin_symbol_encountered = true;

                                if verbose {
                                    println!(" -- native_utilities::parse_symbols_native: 'BEGINSYMBOL' found, begin parsing");
                                }

                                // clean the buffer here
                                buffer.clear();
                                continue;
                            }

                            if line_trimmed == "ENDSYMBOL" {
                                end_symbol_encountered = true;

                                if verbose {
                                    println!(" -- native_utilities::parse_symbols_native: 'ENDSYMBOL' found, ending parsing");
                                }

                                // clean the buffer
                                buffer.clear();
                                // end the parsing
                                break;
                                // continue;
                            }

                            if begin_symbol_encountered && !end_symbol_encountered {
                                let vec: Vec<&str> = line_trimmed.split("//").collect();

                                let not_ignored = vec[0].trim();

                                if not_ignored == "BEGINSYMBOL" {
                                    begin_symbol_encountered = true;

                                    if verbose {
                                        println!(" -- native_utilities::parse_symbols_native: 'BEGINSYMBOL' found, begin parsing");
                                    }

                                    buffer.clear();
                                    continue;
                                }

                                if not_ignored == "ENDSYMBOL" {
                                    end_symbol_encountered = true;

                                    if verbose {
                                        println!(" -- native_utilities::parse_symbols_native: 'ENDSYMBOL' found, ending parsing");
                                    }

                                    buffer.clear();
                                    // we should end parse here
                                    // continue;
                                    break;
                                }

                                if verbose {
                                    let ignored: String = String::from(vec[1..].join("//").trim());

                                    if !ignored.is_empty() {
                                        println!(" -- native_utilities::parse_symbols_native: this comment will be ignored: {}", &ignored);
                                    }
                                }

                                let parsed: io::Result<(&str, DLType)> =
                                    string_to_symbol(not_ignored);

                                match parsed {
                                    Ok((name, t)) => {
                                        let new_ps = PS::new(String::from(name), t);
                                        unsorted_symbols.push(new_ps);

                                        if verbose {
                                            println!(
                                                " -- native_utilities::parse_symbols_native: result of parsing:   name: {}, type: {}",
                                                name, t
                                            );
                                        }
                                    }
                                    Err(e) => {
                                        if verbose {
                                            println!(" -- native_utilities::parse_symbols_native: couldn't parse: {}", e);
                                        }
                                    }
                                }
                            } else if verbose {
                                println!(" -- native_utilities::parse_symbols_native: line won't be parsed, not in between 'BEGINSYMBOL' and 'ENDSYMBOL' bounds");
                            }
                        }

                        // clean the buffer !!!!
                        buffer.clear();
                    }
                    Err(e) => {
                        if verbose {
                            buffer.clear();
                            println!("passing because of error: {}", e);
                        }
                    }
                }

                buffer.clear();
            }

            if !end_symbol_encountered {
                if verbose {
                    println!(" -- native_utilities::parse_symbols_native: 'ENDSYMBOL' not encountered, returning nothing");
                }

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
                for (i, item) in unsorted_symbols.iter().enumerate().take(unsorted_size) {
                    // for i in 0..unsorted_size {
                    // let s = &unsorted_symbols[i];

                    // let name = s.name();
                    // let t = s.t();

                    let name = item.name();
                    let t = item.t();

                    symbols.insert(String::from(name), (i + 2, t));
                }
                Result::Ok(symbols)
            }
        }
    }
}

pub fn parse_tbox_native(
    filename: &str,
    symbols: &SymbolDict,
    verbose: bool,
) -> io::Result<TBDllite> {
    let file_result = File::open(filename);

    match file_result {
        Err(e) => {
            if verbose {
                println!(
                    " -- native_utilities::parse_tbox_native: couldn't read the file: {}",
                    e
                );
            }

            Result::Err(e)
        }
        Ok(file) => {
            let mut reader = BufReader::new(file);

            let mut begin_tbox_encountered = false;
            let mut end_tbox_encountered = false;

            let mut tb = TBDllite::new();

            let mut buffer = String::new();

            loop {
                match reader.read_line(&mut buffer) {
                    Ok(bytes_read) => {
                        // end of line
                        if bytes_read == 0 {
                            break;
                        } else {
                            let line = &buffer;

                            if verbose {
                                println!(
                                    " -- native_utilities::parse_tbox_native: trying to parse: {}",
                                    &line
                                );
                            }

                            let line_trimmed = line.trim();

                            if line_trimmed == "BEGINTBOX" {
                                begin_tbox_encountered = true;

                                if verbose {
                                    println!(" -- native_utilities::parse_tbox_native: 'BEGINTBOX' found, begin parsing");
                                }

                                // clean after you
                                buffer.clear();
                                continue;
                            }

                            if line_trimmed == "ENDTBOX" {
                                end_tbox_encountered = true;

                                if verbose {
                                    println!(" -- native_utilities::parse_tbox_native: 'ENDTBOX' found, ending parsing");
                                }
                                buffer.clear();
                                break;
                                // continue;
                            }

                            if begin_tbox_encountered && !end_tbox_encountered {
                                let vec: Vec<&str> = line_trimmed.split("//").collect();

                                let not_ignored = vec[0].trim();

                                if not_ignored == "BEGINTBOX" {
                                    begin_tbox_encountered = true;

                                    if verbose {
                                        println!(" -- native_utilities::parse_tbox_native: 'BEGINTBOX' found, begin parsing");
                                    }
                                    buffer.clear();
                                    continue;
                                }

                                if not_ignored == "ENDTBOX" {
                                    end_tbox_encountered = true;

                                    if verbose {
                                        println!(" -- native_utilities::parse_tbox_native: 'ENDTBOX' found, ending parsing");
                                    }
                                    buffer.clear();
                                    break;
                                    // continue;
                                }

                                if verbose {
                                    let ignored: String = String::from(vec[1..].join("//").trim());

                                    if !ignored.is_empty() {
                                        println!(" -- native_utilities::parse_tbox_native: this comment will be ignored: {}", &ignored);
                                    }
                                }

                                let parsed = string_to_tbi(not_ignored, symbols);

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
                                            println!(" -- native_utilities::parse_tbox_native: couldn't parse: {}", &e);
                                        }
                                    }
                                }
                            } else if verbose {
                                println!(" -- native_utilities::parse_tbox_native: line won't be parsed, not in between 'BEGINTBOX' and 'ENDTBOX' bounds");
                            }
                        }
                        buffer.clear();
                    }
                    Err(e) => {
                        if verbose {
                            println!(" -- native_utilities::parse_tbox_native: couldn't read the line: {}", e);
                            buffer.clear();
                        }
                    }
                }
                buffer.clear();
            }

            if !end_tbox_encountered {
                if verbose {
                    println!(" -- native_utilities::parse_tbox_native: 'ENDTBOX' not encountered, returning nothing");
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
    tbox: &TBDllite,
    symbols: &SymbolDict,
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

            if let Some(some_tbi_str) = tbi_str_op {
                res.push_str(some_tbi_str.as_str());
                res.push('\n');
            }
        }
    }

    res.push_str("ENDTBOX\n");
    Some(res)
}

pub fn find_bound_of_symbols(symbols: &SymbolDict) -> (usize, usize) {
    if symbols.is_empty() {
        (0, 0)
    } else {
        let mut lower: Option<usize> = Option::None;
        let mut upper: Option<usize> = Option::None;

        for (id, _) in symbols.values() {
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

pub fn parse_abox_native_quantum(
    filename: &str,
    symbols: &mut SymbolDict,
    verbose: bool,
) -> io::Result<AbqDllite> {
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
            let mut reader = BufReader::new(file);

            let ab_name = parse_name_from_filename(filename);

            let mut begin_abox_encountered = false;
            let mut end_abox_encountered = false;

            let (_, id_bound) = find_bound_of_symbols(symbols);
            let mut current_id = id_bound + 1;

            let mut ab = AbqDllite::new(ab_name);

            let mut buffer = String::new();

            loop {
                match reader.read_line(&mut buffer) {
                    Err(e) => {
                        println!(
                            " -- native_utilities::parse_abox_native:: passing this line: {:?}",
                            e
                        );
                        buffer.clear();
                        continue;
                    }
                    Ok(bytes_read) => {
                        if bytes_read == 0 {
                            break;
                        } else {
                            let line = &buffer;
                            let line_trimmed = line.trim();

                            if line_trimmed == "BEGINABOX" {
                                begin_abox_encountered = true;

                                if verbose {
                                    println!("'BEGINABOX' found, begin parsing");
                                }

                                buffer.clear();
                                continue;
                            }

                            if line_trimmed == "ENDABOX" {
                                end_abox_encountered = true;

                                if verbose {
                                    println!("'ENDABOX' found, ending parsing");
                                }
                                buffer.clear();
                                break;
                                // continue;
                            }

                            if begin_abox_encountered && !end_abox_encountered {
                                let vec: Vec<&str> = line_trimmed.split("//").collect();

                                let not_ignored = vec[0];

                                if verbose {
                                    let ignored: String = String::from(vec[1..].join("//").trim());

                                    if !ignored.is_empty() {
                                        println!("this comment will be ignored: {}", &ignored);
                                    }
                                }

                                let (parsed_result, current_id_result) =
                                    string_to_abiq(not_ignored, symbols, current_id, false); // parsing from file should be a new abox
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
                            } else if verbose {
                                println!("line won't be parsed, not in between 'BEGINTBOX' and 'ENDTBOX' bounds");
                            }

                            buffer.clear();
                        }
                        buffer.clear();
                    }
                }
                buffer.clear();
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
    abox: &AbqDllite,
    symbols: &SymbolDict,
    dont_write_trivial: bool,
) -> Option<String> {
    let mut res = String::new();

    // I should define a header for this files
    let to_native = true;
    // let header = "";
    // res.push_str(header);

    res.push_str("BEGINABOX\n");

    for abi in abox.items() {
        if !(abi.is_trivial() && dont_write_trivial) {
            let abi_str_op = abiq_to_string(abi, symbols, to_native);

            if let Some(some_abi_str) = abi_str_op {
                res.push_str(some_abi_str.as_str());
                res.push('\n');
            }
        }
    }

    res.push_str("ENDABOX\n");
    Some(res)
}
