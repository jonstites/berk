use std::path::PathBuf;
use structopt::StructOpt;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(StructOpt, Debug)]
#[structopt(version = "0.1")]
pub struct Opt {
    #[structopt(subcommand)]
    pub subcmd: SubCommand,
}

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(StructOpt, Debug)]
pub enum SubCommand {
    /// This doc string acts as a help message when the user runs '--help'
    /// as do all doc strings on fields   
    Init {
        #[structopt(parse(from_os_str))]
        dir: PathBuf,
    },

    /// Add...
    Add {
        #[structopt(parse(from_os_str))]
        files: Vec<PathBuf>,
    },
    Print {},
}
