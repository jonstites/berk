#![deny(unsafe_code)]

extern crate berk;
extern crate structopt;

use structopt::StructOpt;
use exitfailure::ExitFailure;
use failure::ResultExt;
use self::berk::{args, repo};
use std::path::PathBuf;

fn main() -> Result<(), ExitFailure> {
    let opts = args::Opt::from_args();

    match opts.subcmd {
        args::SubCommand::Init { dir } => init(dir)?,
        args::SubCommand::Add { files } => add(files)?,
        args::SubCommand::Print {} => print()?,
    }
    Ok(())
}

fn init(dir: PathBuf) -> Result<(), ExitFailure> {
    let working_directory = std::env::current_dir()?;
    repo::Repo::initialize_database(dir)?;
    Ok(())
}

fn add(files: Vec<PathBuf>) -> Result<(), ExitFailure> {
    let working_directory = std::env::current_dir()?;    
    let mut repo = repo::Repo::load(working_directory)?;
    repo.add_files(files)?;
    Ok(())
}

fn print() -> Result<(), ExitFailure> {
    let working_directory = std::env::current_dir()?;    
    let repo = repo::Repo::load(working_directory)?;

    let blobs = repo.read_blobs()?;

    for blob in &blobs {
        println!("{:?}", blob.blob_oid);
    }

    let blobs = repo.read_stage()?;
    for blob in &blobs {
	println!("{:?}", blob);
    }
    Ok(())
}
