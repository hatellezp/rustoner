use crate::kb::types::FileType;
use crate::dl_lite::types::CR::Fifth;

pub fn is_json_file(filename: &str) -> bool {
    filename.ends_with(".json")
}

pub fn get_filetype(filename: &str) -> FileType {
    let res = match is_json_file(filename) {
        true => FileType::JSON,
        false => FileType::NATIVE,
    };

    res
}