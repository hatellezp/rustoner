use crate::dl_lite::types::DLType;
use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use crate::dl_lite::string_formatter::{string_to_symbol, PS, string_to_tbi};
use crate::dl_lite::tbox::TB;

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
                symbols.insert(String::from("Bottom"), (0, DLType::Bottom));
                symbols.insert(String::from("Top"), (1, DLType::Top));

                let unsorted_size = unsorted_symbols.len();

                for i in 0..unsorted_size {
                    let s = &unsorted_symbols[i];

                    let name = s.name();
                    let t = s.t();

                    symbols.insert(String::from(name), (i + 2, t));
                }

                Result::Ok((symbols))
            }
        },
    }
}

pub fn parse_tbox_native(filename: &str, symbols: &HashMap<String, (usize, DLType)>, verbose: bool) -> io::Result<TB> {
    let file_result = File::open(filename);

    match file_result {
        Err(e) => {
            println!("couldn't read the file: {}", e);
            Result::Err(e)
        },
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

                            let not_ignored = vec[0].clone();

                            if verbose {
                                let ignored: String = String::from(vec[1..].join("//").trim());

                                if &ignored != "" {
                                    println!("this comment will be ignored: {}", &ignored);
                                }
                            }

                            let parsed = string_to_tbi(&not_ignored, symbols);

                            match parsed {
                                Ok(tbi) => {
                                    tb.add(tbi);
                                },
                                Err(e) => {
                                    if verbose {
                                        println!("couldn't parse: {}", &e);
                                    }
                                },
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
                Result::Ok((tb))
            }
        },
    }
}
