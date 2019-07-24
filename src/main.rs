#![deny(unsafe_code)]

extern crate structopt;
use berk::{Database, GitBlob};
use structopt::StructOpt;

#[macro_use]
extern crate error_chain;
mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{}
}

use errors::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "berk")]
/// the stupid content tracker
enum Opt {
    #[structopt(name = "init")]
    /// initialize the berk repo
    Init {},
    #[structopt(name = "add")]
    /// add files to the staging area
    Add {
        files: Vec<String>,
    },
}

fn main() -> Result<()> {
    let matches = Opt::from_args();

    match matches {
        Opt::Init{} => init_repo("./.berk/berk.db3"),
        Opt::Add{files} => add_files("./.berk/berk.db3", files),
    }
}

fn init_repo(db_path: &str) -> Result<()> {
    let database = Database::new(db_path)
        .chain_err(|| "unable to open database")?;
    
    database.init()
        .chain_err(|| "unable to create database tables")?;

    Ok(())
}

fn add_files(db_path: &str, files: Vec<String>) -> Result<()> {
    let database = Database::new(db_path)
        .chain_err(|| "unable to open database")?;

    for file in files.iter() {
        let git_blob = GitBlob::from_file(file)
            .chain_err(|| "unable to read file")?;

        database.commit_blob(git_blob)
            .chain_err(|| "unable to commit git blob")?;
    }

    Ok(())

}