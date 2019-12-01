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

/// todo. need to store filenames
/// actually, saving to index requires more.. including filename
/// to store the filename, we'll need a clean way to go from absolute paths
/// to path relative to database.
fn add(files: Vec<PathBuf>) -> Result<(), ExitFailure> {
    let working_directory = std::env::current_dir()?;    
    let repo = repo::Repo::load(working_directory)?;
    repo.add_files(files)?;
    
    /*let connection = berk::repo::establish_connection(".berk.db")?;

    for file in &files {
        for entry in WalkDir::new(file) {
            let entry = entry.unwrap();
            let entry_type = entry.file_type();

            if entry_type.is_file() {
                println!("{}", entry.path().display());
                let mut file = File::open(entry.path())?;
                let mut contents = Vec::new();
                file.read_to_end(&mut contents)?;
                berk::insert_blob(&connection, contents)?;
            }
        }
    }*/
    Ok(())
}

fn print() -> Result<(), ExitFailure> {
    /*use berk::schema::blob_objects::dsl::*;

    let connection = berk::repo::establish_connection(".berk.db")?;

    let blobs = blob_objects.load::<BlobObject>(&connection)?;

    for blob in &blobs {
        println!("{:?}", blob);
    }*/
    Ok(())
}
