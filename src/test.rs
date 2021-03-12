mod dl_lite;
mod interface;
mod kb;
// for cli interface

/*
use std::path::PathBuf;
use structopt::StructOpt;

// use crate::shiq::xml_parser::{read_from_filename};

#[derive(Debug, StructOpt)]
#[structopt(name = "example")]
struct Opt {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

 */

use crate::dl_lite::abox_item_quantum::ABIQ_DLlite;
use crate::dl_lite::ontology::Ontology_DLlite;
use crate::dl_lite::tbox_item::TBI_DLlite;
use crate::kb::knowledge_base::{ABox, ABoxItem, Implier, TBox};
use crate::kb::types::FileType;

fn main() {
    let abox_filename = "a_man";
    let tbox_filename = "are_men_mortals";

    let mut onto = Ontology_DLlite::new(String::from("onto1"));

    onto.add_symbols_from_file(tbox_filename, FileType::NATIVE, false);
    onto.add_tbis_from_file(tbox_filename, FileType::NATIVE, false);

    onto.new_abox_from_file_quantum(abox_filename, FileType::NATIVE, false);

    let symbols = onto.symbols();
    let abox = onto.abox().unwrap();
    let tbox = onto.tbox();

    let abiq = abox.items().get(0).unwrap();
    let tbi = tbox.items().get(0).unwrap();

    let number: usize = 4;
    let op_float: Option<f64> = Some(22.2);
    let float: f64 = 22.2;

    // this are dynamic sizes
    println!(
        " -- size of onto {:?}: {} bytes\n",
        &onto,
        std::mem::size_of_val(&onto)
    );
    println!(
        " -- size of symbols {:?}: {} bytes\n",
        symbols,
        std::mem::size_of_val(symbols)
    );
    println!(
        " -- size of tbox {:?}: {}bytes\n",
        tbox,
        std::mem::size_of_val(tbox)
    );
    println!(
        " -- size of abox {:?}: {} bytes\n",
        abox,
        std::mem::size_of_val(abox)
    );
    println!(
        " -- size of tbi {:?}: {} bytes\n",
        tbi,
        std::mem::size_of_val(tbi)
    );
    println!(
        " -- size of abiq {:?}: {} bytes\n",
        abiq,
        std::mem::size_of_val(abiq)
    );
    println!(
        " -- size of {}: {} bytes\n",
        number,
        std::mem::size_of_val(&number)
    );
    println!(
        " -- size of {:?}: {} bytes\n",
        op_float,
        std::mem::size_of_val(&op_float)
    );
    println!(
        " -- size of {}: {} bytes\n",
        float,
        std::mem::size_of_val(&float)
    );

    println!(" -- what's the problem with abiq?");
    let symbol = abiq.item();
    let nom = abiq.nominal(0).unwrap();
    let pv = abiq.prevalue();
    let v = abiq.value();
    let impliers = abiq.implied_by();

    println!(
        " -- size of symbol {:?}: {} bytes\n",
        symbol,
        std::mem::size_of_val(symbol)
    );
    println!(
        " -- size of nom {:?}: {} bytes\n",
        nom,
        std::mem::size_of_val(nom)
    );
    println!(
        " -- size of pv {:?}: {} bytes \n",
        pv,
        std::mem::size_of_val(&pv)
    );
    println!(
        " -- size of v {:?}: {} bytes \n",
        v,
        std::mem::size_of_val(&v)
    );
    println!(
        " -- size of impliers {:?}: {} bytes\n",
        impliers,
        std::mem::size_of_val(impliers)
    );

    println!(" -- size of vectors");
    let vusize: Vec<usize> = Vec::new();
    let vabiq: Vec<ABIQ_DLlite> = Vec::new();
    let vtbi: Vec<TBI_DLlite> = Vec::new();

    println!(
        " -- size of vusize {:?}: {} bytes\n",
        vusize,
        std::mem::size_of_val(&vusize)
    );
    println!(
        " -- size of vabiq {:?}: {} bytes\n",
        vabiq,
        std::mem::size_of_val(&vabiq)
    );
    println!(
        " -- size of vtbi {:?}: {} bytes\n",
        vtbi,
        std::mem::size_of_val(&vtbi)
    );

    onto.complete_tbox(true, false);
    let ab = onto.complete_abox(true, false).unwrap();
    onto.add_abis_from_abox(&ab);

    let symbols = onto.symbols();
    let abox = onto.abox().unwrap();
    let tbox = onto.tbox();

    let abiq = abox.items().last().unwrap();
    let tbi = tbox.items().last().unwrap();

    let number: usize = 4;
    let op_float: Option<f64> = Some(22.2);
    let float: f64 = 22.2;

    // this are dynamic sizes
    println!(
        " -- size of onto {:?}: {} bytes\n",
        &onto,
        std::mem::size_of_val(&onto)
    );
    println!(
        " -- size of symbols {:?}: {} bytes\n",
        symbols,
        std::mem::size_of_val(symbols)
    );
    println!(
        " -- size of tbox {:?}: {} bytes\n",
        tbox,
        std::mem::size_of_val(tbox)
    );
    println!(
        " -- size of abox {:?}: {} bytes\n",
        abox,
        std::mem::size_of_val(abox)
    );
    println!(
        " -- size of tbi {:?}: {} bytes\n",
        tbi,
        std::mem::size_of_val(tbi)
    );
    println!(
        " -- size of abiq {:?}: {} bytes\n",
        abiq,
        std::mem::size_of_val(abiq)
    );
    println!(
        " -- size of {}: {} bytes\n",
        number,
        std::mem::size_of_val(&number)
    );
    println!(
        " -- size of {:?}: {} bytes\n",
        op_float,
        std::mem::size_of_val(&op_float)
    );
    println!(
        " -- size of {}: {} bytes\n",
        float,
        std::mem::size_of_val(&float)
    );
}
