use crate::kb::types::FileType;
use std::fs::File;
use std::io::Write;

pub fn is_json_file(filename: &str) -> bool {
    filename.ends_with(".json")
}

pub fn get_filetype(filename: &str) -> FileType {
    match is_json_file(filename) {
        true => FileType::JSON,
        false => FileType::NATIVE,
    }
}

pub fn parse_name_from_filename(filename: &str) -> &str {
    let path_separator = std::path::MAIN_SEPARATOR; // smart :)
    let v: Vec<&str> = filename.split(path_separator).collect();

    let name = v.last().unwrap().trim();
    let v: Vec<&str> = name.split(".").collect();

    v[0].trim()
}

pub fn write_str_to_file(s: &str, filename: &str) -> bool {
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
