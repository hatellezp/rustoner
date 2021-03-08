mod interface;
mod kb;
mod shiq;
mod dl_lite;
// for cli interface
use std::path::PathBuf;
use structopt::StructOpt;

use crate::kb::types::FileType;

use crate::shiq::helpers_and_utilities::print_matrix;
use crate::shiq::ontology::Ontology_SHIQ;

use crate::shiq::xml_parser::{read_from_filename};

#[derive(Debug, StructOpt)]
#[structopt(name="example")]
struct Opt {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn main() {
    let opt = Opt::from_args();

    let p: PathBuf = opt.input;

    read_from_filename(&p);

}
