use crate::dl_lite::abox_item::ABI;
use crate::dl_lite::abox_item_quantum::ABIQ;
use crate::dl_lite::json_filetype_utilities::{invalid_data_result, result_from_error};
use crate::dl_lite::node::{Mod, Node};
use crate::dl_lite::tbox_item::TBI;
use crate::dl_lite::types::DLType;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;
use std::io;
use std::io::{Error, ErrorKind};

use crate::dl_lite::string_formatter::{string_to_abi, node_to_string, abi_to_string};

//--------------------------------------------------------------------------------------------------




// this approach is a dynamic one, concepts must be present in symbols,
// but nominals are added dynamically
pub fn string_to_abiq(
    s: &str,
    symbols: &mut HashMap<String, (usize, DLType)>,
    mut current_id: usize,
    for_completion: bool,
) -> (io::Result<(ABIQ, Vec<(String, (usize, DLType))>)>, usize) {
    let splitted = s.trim();

    // a normal abi is splitted by ':', here we split by ';' first
    let splitted: Vec<&str> = splitted.split(",").collect();
    let splitted: Vec<&str> = splitted.iter().map(|x| x.trim()).collect();
    let lenght = splitted.len();

    let mut new_s = splitted[0].to_string();

    let (for_abi, pvalue, value) = match lenght {
        1 => (s.to_string(), 1.0, Option::None),
        2 => {
            let possible_pv = splitted[1].parse::<f64>();

            match possible_pv {
                Err(_) => {
                    new_s.push_str(",");
                    new_s.push_str(splitted[1]);

                    (new_s, 1.0, Option::None)
                },
                Ok(n) => {
                    (new_s, n, Option::None)
                },
            }
        },
        3 => {
            let possible_pv = splitted[1].parse::<f64>();

            match possible_pv {
                Err(_) => { // the second term is not a number
                    new_s.push_str(",");
                    new_s.push_str(splitted[1]);

                    // now parse the rest
                    let pv_res = splitted[2].parse::<f64>();
                    let v = Option::None;

                    match pv_res {
                        Err(_) => (new_s, 1.0, v),
                        Ok(n) => (new_s, n, v),
                    }
                },
                Ok(n) => {
                    // it is indeed a number
                    let v_res = splitted[2].parse::<f64>();

                    match v_res {
                        Err(_) => (new_s, n, Option::None),
                        Ok(nvalue) => (new_s, n, Some(nvalue)),
                    }
                },
            }
        },
        _ => {
            new_s.push_str(",");
            new_s.push_str(splitted[1]);

            let pv_res = splitted[2].parse::<f64>();
            let v_res = splitted[3].parse::<f64>();

            match (pv_res, v_res) {
                (Err(_), Err(_)) => (new_s, 1.0, Option::None),
                (Err(_), Ok(nvalue)) => (new_s, 1.0, Some(nvalue)),
                (Ok(npvalue), Err(_)) => (new_s, npvalue, Option::None),
                (Ok(npvalue), Ok(nvalue)) => (new_s, npvalue, Some(nvalue)),
            }
        },
    };

    // println!("---- len: {}\n     abi: {}\n     pv: {}\n     v: {:?}", splitted.len(), for_abi, pvalue, value);

    let (abi_res, cid) = string_to_abi(&for_abi, symbols, current_id, for_completion);

    match abi_res {
        Err(e) => {
            (Err(e), cid)
        },
        Ok((abi, v)) => {
            let abiq = ABIQ::new(abi, Some(pvalue), value);

            (Ok((abiq, v)), cid)
        },
    }
}

pub fn abiq_to_string(abiq: &ABIQ, symbols: &HashMap<String, (usize, DLType)>) -> Option<String> {
    let abi_to_string = abi_to_string(abiq.abi(), symbols);

    let pv_to_string = format!("{}", abiq.prevalue());
    let v_to_string = match abiq.value() {
        Option::None => "NC".to_string(),
        Some(v) => format!("{}", v),
    };

    match abi_to_string {
        Option::None => Option::None,
        Some(s) => {
            let res = format!("{},{},{}", s, pv_to_string, v_to_string);

            Some(res)
        }
    }
}

