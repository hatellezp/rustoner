mod dl_lite;
mod kb;
mod interface;

// for cli interface
use structopt::StructOpt;

use crate::kb::types::FileType;

use crate::dl_lite::ontology::Ontology;
use crate::interface::cli::{Cli, Task};
use crate::interface::utilities::get_filetype;


fn main() {
    let args = Cli::from_args();

    match args.task {
        Task::CTB => {
            let path_tbox = args.path_tbox.to_str().unwrap();
            let tbox_ft = get_filetype(path_tbox);

            let path_output_op = args.path_output;
            let path_symbols_op = args.path_symbols;

            let mut onto = Ontology::new(String::from("test"));

            match path_symbols_op {
                Some(path_symbols) => {
                    let path_symbols = path_symbols.to_str().unwrap();
                    let symbols_ft = get_filetype(path_symbols);

                    onto.add_symbols(path_symbols, symbols_ft, args.verbose);
                },
                Option::None => {
                    println!("here: {:?} and {:?}", &path_tbox, &tbox_ft);
                    onto.add_symbols(path_tbox, tbox_ft, args.verbose);
                },
            }

            onto.add_tbis(path_tbox, tbox_ft, args.verbose);
            onto.auto_complete(false);

            match path_output_op {
                Some(path_output) => {
                    onto.tbox_to_file(path_output.to_str().unwrap(), FileType::NATIVE, true);
                },
                Option::None => {
                    let mut s = String::new();
                    let formatted = format!("----<TBox>\n{}\n", &onto.tbox_to_string(&onto.tbox()));
                    s.push_str(formatted.as_str());

                    println!("{}", s);
                },
            }
        },
        Task::CAB | Task::RNK => println!("not implemented"),
        Task::UNDEFINED => println!("unrecognized task!"),
    }
}

