use crate::dl_lite::abox::ABQ;
use crate::dl_lite::abox_item_quantum::ABIQ;
use crate::dl_lite::tbox::TB;
use crate::dl_lite::tbox_item::TBI;
use std::io;

/*------------------------------------------------------------------------------------------------*/
// tbox tasks

pub fn verify_tbox(_tb: &TB) -> bool {
    false
}

pub fn generate_consequence_tree_tbox(_tb: &TB) -> Vec<(Vec<&TBI>, &TBI)> {
    let v: Vec<(Vec<&TBI>, &TBI)> = Vec::new();

    v
}

pub fn complete_tbox(_tb: &TB) -> TB {
    let new_tb = TB::new();

    new_tb
}

pub fn save_to_file_tbox(_tb: &TB) -> io::Result<String> {
    Ok(String::from("all is well"))
}

/*------------------------------------------------------------------------------------------------*/
// abox tasks, follow the same name convention

pub fn verify_abox(_ab: &ABQ, _tb: &TB) -> bool {
    false
}

pub fn generate_consequence_tree_abox<'a>(
    _ab: &'a ABQ,
    _tb: &'a TB,
) -> Vec<((Vec<&'a TBI>, Vec<&'a ABIQ>), &'a ABIQ)> {
    let new_v: Vec<((Vec<&TBI>, Vec<&ABIQ>), &ABIQ)> = Vec::new();

    new_v
}

pub fn complete_abox(ab: &ABQ, _tb: &TB) -> ABQ {
    let mut new_name = ab.name().to_string();
    new_name.push_str("_complete");

    let new_ab = ABQ::new(&new_name);

    new_ab
}

pub fn save_to_file_abox(_ab: &ABQ) -> io::Result<String> {
    Ok(String::from("all is well"))
}
