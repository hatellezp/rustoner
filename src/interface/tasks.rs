use crate::dl_lite::abox::ABQ_DLlite;
use crate::dl_lite::abox_item_quantum::ABIQ_DLlite;
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

pub fn verify_abox(_ab: &ABQ_DLlite, _tb: &TB_DLlite) -> bool {
    false
}

pub fn generate_consequence_tree_abox<'a>(
    _ab: &'a ABQ_DLlite,
    _tb: &'a TB_DLlite,
) -> Vec<((Vec<&'a TBI_DLlite>, Vec<&'a ABIQ_DLlite>), &'a ABIQ_DLlite)> {
    let new_v: Vec<((Vec<&TBI_DLlite>, Vec<&ABIQ_DLlite>), &ABIQ_DLlite)> = Vec::new();

    new_v
}

pub fn complete_abox(ab: &ABQ_DLlite, _tb: &TB_DLlite) -> ABQ_DLlite {
    let mut new_name = ab.name().to_string();
    new_name.push_str("_complete");

    let new_ab = ABQ_DLlite::new(&new_name);

    new_ab
}

pub fn save_to_file_abox(_ab: &ABQ_DLlite) -> io::Result<String> {
    Ok(String::from("all is well"))
}
