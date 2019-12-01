#![deny(unsafe_code)]

extern crate berk;
extern crate diesel;
extern crate structopt;

use exitfailure::ExitFailure;
use failure::ResultExt;
use std::path::PathBuf;
use walkdir::WalkDir;
use std::fs::File;
use std::io::Read;


use self::berk::*;
use self::diesel::prelude::*;
use self::models::*;
use self::structopt::StructOpt;

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
    let mut repo_path = std::fs::canonicalize(dir.clone())
        .with_context(|_| format!("Could not resolve path: {:?}", dir))?;

    repo_path.push(".berk.db");

    let repo = repo_path.to_str()
	.ok_or(failure::err_msg(format!("Not valid UTF-8: {:?}", repo_path)))?;

    berk::repo::initialize(repo)
}

fn add(files: Vec<PathBuf>) -> Result<(), ExitFailure> {
    
    let connection = berk::repo::establish_connection(".berk.db")?;
    
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
    }
    Ok(())
}

fn print() -> Result<(), ExitFailure> {
    use berk::schema::blob_objects::dsl::*;
    
    let connection = berk::repo::establish_connection(".berk.db")?;

    let blobs = blob_objects
	.load::<BlobObject>(&connection)?;

    for blob in &blobs {
	println!("{:?}", blob);
    }
    Ok(())
}
