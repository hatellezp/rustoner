mod dl_lite;
mod interface;
mod kb;
// for cli interface
use std::path::PathBuf;
use structopt::StructOpt;

// use crate::shiq::xml_parser::{read_from_filename};

#[derive(Debug, StructOpt)]
#[structopt(name = "example")]
struct Opt {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn main() {
    let opt = Opt::from_args();

    let _p: PathBuf = opt.input;

    // read_from_filename(&p);
}
