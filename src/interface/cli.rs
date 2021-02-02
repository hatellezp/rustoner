use std::string::ParseError;
use std::str::FromStr;
use structopt::StructOpt;


// more to be added after
#[derive(Debug)]
pub enum Task {
    CTB, // complete tbox
    CAB, // complete abox
    RNK, // rank assertions
    UNDEFINED,
}


impl FromStr for Task {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s2 = s.trim();

        match s2 {
            "ctb" | "CTB" => Ok(Task::CTB),
            "cab" | "CAB" => Ok(Task::CAB),
            "rank" | "RANK" | "Rank" => Ok(Task::RNK),
            _ => Ok(Task::UNDEFINED),
        }
    }
}

#[derive(StructOpt, Debug)]
pub struct Cli {
    #[structopt(short="t", long="task")]
    pub task: Task,

    #[structopt(long="verbose")]
    pub verbose: bool,

    #[structopt(parse(from_os_str), long="tbox")]
    pub path_tbox: std::path::PathBuf,

    #[structopt(parse(from_os_str), long="output")]
    pub path_output: Option<std::path::PathBuf>,

    #[structopt(parse(from_os_str), long="symbol")]
    pub path_symbols: Option<std::path::PathBuf>,

}