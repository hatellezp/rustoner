use crate::kb::types::FileType;

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

pub fn parse_name_from_filename<'a>(filename: &'a str) -> &'a str {
    let path_separator = std::path::MAIN_SEPARATOR;
    let v: Vec<&str> = filename.split(path_separator).collect();

    let name = v.last().unwrap().trim();
    let v: Vec<&str> = name.split(".").collect();

    v[0].trim()
}
