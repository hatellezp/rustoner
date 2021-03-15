use crate::dl_lite::abox::AbqDllite;
use crate::dl_lite::abox_item_quantum::AbiqDllite;
use crate::dl_lite::tbox::TB_DLlite;
use crate::dl_lite::tbox_item::TBI_DLlite;
use std::io;

use crate::kb::knowledge_base::ABox;

/*------------------------------------------------------------------------------------------------*/
// tbox tasks

pub fn verify_tbox(_tb: &TB_DLlite) -> bool {
    false
}

pub fn generate_consequence_tree_tbox(_tb: &TB_DLlite) -> Vec<(Vec<&TBI_DLlite>, &TBI_DLlite)> {
    let v: Vec<(Vec<&TBI_DLlite>, &TBI_DLlite)> = Vec::new();

    v
}

pub fn complete_tbox(_tb: &TB_DLlite) -> TB_DLlite {
    let new_tb = TB_DLlite::new();

    new_tb
}

pub fn save_to_file_tbox(_tb: &TB_DLlite) -> io::Result<String> {
    Ok(String::from("all is well"))
}

/*------------------------------------------------------------------------------------------------*/
// abox tasks, follow the same name convention

pub fn verify_abox(_ab: &AbqDllite, _tb: &TB_DLlite) -> bool {
    false
}

pub fn generate_consequence_tree_abox<'a>(
    _ab: &'a AbqDllite,
    _tb: &'a TB_DLlite,
) -> Vec<((Vec<&'a TBI_DLlite>, Vec<&'a AbiqDllite>), &'a AbiqDllite)> {
    let new_v: Vec<((Vec<&TBI_DLlite>, Vec<&AbiqDllite>), &AbiqDllite)> = Vec::new();

    new_v
}

pub fn complete_abox(ab: &AbqDllite, _tb: &TB_DLlite) -> AbqDllite {
    let mut new_name = ab.name().to_string();
    new_name.push_str("_complete");

    let new_ab = AbqDllite::new(&new_name);

    new_ab
}

pub fn save_to_file_abox(_ab: &AbqDllite) -> io::Result<String> {
    Ok(String::from("all is well"))
}
