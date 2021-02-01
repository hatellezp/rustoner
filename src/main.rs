mod dl_lite;
mod kb;

use crate::kb::types::FileType;

use crate::dl_lite::node::Node;
use crate::dl_lite::ontology::Ontology;
use crate::dl_lite::tbox::TB;
use crate::dl_lite::tbox_item::TBI;
use crate::dl_lite::types::DLType;
use std::iter::Filter;
use crate::dl_lite::native_filetype_utilities::parse_abox_native;
use std::collections::HashMap;

use structopt::StructOpt;
use std::str::FromStr;
use std::string::ParseError;
use std::convert::Infallible;

// more to be added after
#[derive(Debug)]
enum Task {
    CTB, // complete tbox
    // CAB, // complete abox
    UNDEFINED,
}


impl FromStr for Task {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s2 = s.trim();

        match s2 {
            "ctb" | "CTB" => Ok(Task::CTB),
            _ => Ok(Task::UNDEFINED),
        }
    }
}

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(short="t", long="task")]
    task: Task,

    #[structopt(parse(from_os_str), long="path_tbox")]
    path_tbox: std::path::PathBuf,

    #[structopt(parse(from_os_str), long="path_output")]
    path_output: std::path::PathBuf,
}


fn main() {
    let args = Cli::from_args();

    match args.task {
        Task::CTB => {
            let path_tbox = args.path_tbox.to_str().unwrap();
            let path_output = args.path_output.to_str().unwrap();

            let mut onto = Ontology::new(String::from("test"));

            onto.add_symbols(path_tbox, FileType::NATIVE);
            onto.add_tbis(path_tbox, FileType::NATIVE, false);

            onto.auto_complete(false);

            onto.tbox_to_file(path_output, FileType::NATIVE, true);

        },
        Task::UNDEFINED => println!("unrecognized task!"),
    }
}

