use crate::dl_lite::types::DLType;
use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use crate::dl_lite::string_formatter::{string_to_symbol, PS};

pub fn read_file_and_print_line_by_line(filename: &str) -> io::Result<()> {
    let file_result = File::open(filename);

    match file_result {
        Err(e) => {
            println!("couldn't read the file: {}", e);
            Result::Err(e)
        }
        Ok(file) => {
            let reader = BufReader::new(file);

            for line_result in reader.lines() {
                match line_result {
                    Err(_) => (),
                    Ok(line) => {
                        println!("---- original line: {}", &line);
                        let mut vec: Vec<&str> = line.split("//").collect();

                        let not_ignored = vec[0].clone();

                        /*
                        here I'm doing this to verify that after '//' everything is well ignored
                        but in a real file we forget this
                         */
                        let ignored: String = vec[1..].join("//");

                        println!("this part is not ignored: {}", not_ignored);
                        println!("this part is ignored: {}", ignored);
                    }
                }
            }

            Result::Ok(())
        }
    }
}

pub fn parse_symbols(
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

                            let not_ignored = vec[0].clone();

                            if verbose {
                                let ignored: String = String::from(vec[1..].join("//").trim());

                                if &ignored != "" {
                                    println!("this comment will be ignored: {}", &ignored);
                                }
                            }

                            let parsed: io::Result<(&str, DLType)> =
                                string_to_symbol(&not_ignored);

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

                let new_error = Error::new(
                    ErrorKind::UnexpectedEof,
                    "'ENDSYMBOL' not found before file ended",
                );

                Result::Err(new_error)
            } else {
                // here I need to sort unsorted_symbols
                unsorted_symbols.sort();

                // symbols sorted, so now I put everyting in symbols
                symbols.insert(String::from("BOTTOM"), (0, DLType::Bottom));
                symbols.insert(String::from("TOP"), (1, DLType::Top));

                let unsorted_size = unsorted_symbols.len();

                for i in 0..unsorted_size {
                    let s = &unsorted_symbols[i];

                    let name = s.name();
                    let t = s.t();

                    symbols.insert(String::from(name), (i + 2, t));
                }

                Result::Ok((symbols))
            }
        }
    }
}
