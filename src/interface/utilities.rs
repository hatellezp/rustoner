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

use crate::kb::types::FileType;
use std::fs::File;
use std::io::Write;


/// All these functions are utilities for IO tasks as parsing names and
/// detecting file extensions.

// TODO: write a real test, some file do not have the '.json' extension
pub fn is_json_file(filename: &str) -> bool {
    // check if the file is of json type

    filename.ends_with(".json")
}

pub fn get_filetype(filename: &str) -> FileType {
    // class ontology files in one of the known types to parse
    match is_json_file(filename) {
        true => FileType::JSON,
        false => FileType::NATIVE,
    }
}

/// Find the actual name of an ontology file from a path string
/// (e.g. 'path/to/file/myfile.extension' -> 'myfile').
pub fn parse_name_from_filename(filename: &str) -> &str {

    let path_separator = std::path::MAIN_SEPARATOR; // smart :)
    let v: Vec<&str> = filename.split(path_separator).collect();

    let name = v.last().unwrap().trim();  // this vector can never be empty, asking
                                               // for last is ok
    let v: Vec<&str> = name.split('.').collect();
    v[0].trim()
}

pub fn write_str_to_file(s: &str, filename: &str) -> bool {
    // a simple write to file function
    // take a raw string 's' and dump it to file 'filename'

    let file_res = File::create(filename);

    match file_res {
        Result::Err(e) => {
            println!("something went wrong: {}", e);
            false
        }
        Result::Ok(mut file) => {
            let result = file.write(s.as_bytes());

            match result {
                Result::Err(e) => {
                    println!("something went wrong while writing to the file: {}", e);
                    false
                }
                Result::Ok(_) => true,
            }
        }
    }
}
