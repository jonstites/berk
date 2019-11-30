#![deny(unsafe_code)]

extern crate berk;
extern crate diesel;


extern crate structopt;
use structopt::StructOpt;

use failure::ResultExt;
use exitfailure::ExitFailure;

use self::berk::*;
use self::models::*;
use self::diesel::prelude::*;

#[macro_use]
extern crate diesel_migrations;
use diesel_migrations::embed_migrations;

use std::path::PathBuf;

embed_migrations!();


fn main() -> Result<(), ExitFailure> {
    use berk::schema::blob_objects::dsl::*;

    let opts = Opt::from_args();
    match opts.subcmd {
	SubCommand::Init{dir} => initialize_repo(dir)?,
    }
    Ok(())
}

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(StructOpt, Debug)]
#[structopt(version = "0.1")]
struct Opt {
    #[structopt(subcommand)]
    subcmd: SubCommand,
}

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(StructOpt, Debug)]
enum SubCommand {
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
    }
}


fn initialize_repo(repo_path: PathBuf) -> Result<(), ExitFailure> {

    let connection = open_repo(repo_path)?;
    
    // This will run the necessary migrations.
    embedded_migrations::run(&connection)
	.with_context(|_| format!("could not initialize repo"))?;

    Ok(())
}

fn open_repo(mut repo_path: PathBuf) -> Result<SqliteConnection, ExitFailure> {
    repo_path.push(".berk.db");
    let repo_url = repo_path.to_str()
	.ok_or(failure::err_msg(format!("could not read repo as a String: {:?}", repo_path)))?;
    
    let connection = berk::establish_connection(repo_url)?;

    Ok(connection)
}
