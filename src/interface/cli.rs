use std::str::FromStr;
use std::string::ParseError;
use structopt::StructOpt;

// more to be added after
#[derive(Debug)]
pub enum Task {
    VERTB,    // verify tbox
    GENCONTB, // generate consequence tree tbox
    CTB,      // complete tbox
    VERAB,    // verify abox
    CLEANAB,  // clean from self conflicts
    GENCONAB, // generate consequence tree abox
    CAB,      // complete abox
    RNKAB,    // rank assertions on abox
    UNDEFINED,
    // INIT, // initiate database
}

impl FromStr for Task {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s2 = s.trim();

        match s2 {
            "vertb" => Ok(Task::VERTB),
            "gencontb" => Ok(Task::GENCONTB),
            "ctb" => Ok(Task::CTB),
            "verab" => Ok(Task::VERAB),
            "cleanab" => Ok(Task::CLEANAB),
            "genconab" => Ok(Task::GENCONAB),
            "cab" => Ok(Task::CAB),
            "rankab" => Ok(Task::RNKAB),
            // "init" => Ok(Task::INIT),
            _ => Ok(Task::UNDEFINED),
        }
    }
}

#[derive(Debug)]
pub enum AggrName {
    MAX,
    MIN,
    SUM,
    MEAN,
    COUNT,
    UNDEFINED,
}

impl FromStr for AggrName {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s2 = s.trim();

        match s2 {
            "sum" => Ok(AggrName::SUM),
            "max" => Ok(AggrName::MAX),
            "min" => Ok(AggrName::MIN),
            "count" => Ok(AggrName::COUNT),
            "mean" => Ok(AggrName::MEAN),
            _ => Ok(AggrName::UNDEFINED),
        }
    }
}

#[derive(StructOpt, Debug)]
pub struct Cli {
    #[structopt(
        short = "t",
        long = "task",
        help = "describes the wanted task, (init|ctb|cab|rank), 'init' to initialize your ontology on a sqlite database, 'ctb' for complete tbox, 'cab' for complete abox and 'rank' for ranking of abox assertions"
    )]
    pub task: Task,

    #[structopt(long = "verbose")]
    pub verbose: bool,

    #[structopt(long = "silent")]
    pub silent: bool,

    #[structopt(
        long = "ephemere",
        help = "if specified will work everything in memory and no database trace will be kept"
    )]
    pub ephemere: bool,

    #[structopt(parse(from_os_str), long = "db", help = "path to the database file")]
    pub path_db: Option<std::path::PathBuf>,

    #[structopt(parse(from_os_str), long = "tbox", help = "path to the tbox file")]
    pub path_tbox: Option<std::path::PathBuf>,

    #[structopt(parse(from_os_str), long = "abox", help = "path to the abox file")]
    pub path_abox: Option<std::path::PathBuf>,

    #[structopt(
        parse(from_os_str),
        long = "output",
        help = "optional, if present will output the result of the task to this file"
    )]
    pub path_output: Option<std::path::PathBuf>,

    #[structopt(
        parse(from_os_str),
        long = "symbols",
        help = "optional, if present will parse symbols from this file instead od symbols in the tbox file"
    )]
    pub path_symbols: Option<std::path::PathBuf>,

    #[structopt(
        long = "aggr",
        help = "choose a function to aggregate during conflict graph computing: (sum|min|max|count)"
    )]
    pub aggr: Option<AggrName>,
}
