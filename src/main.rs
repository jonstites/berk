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

use walkdir::WalkDir;

fn main() -> Result<(), ExitFailure> {
    let opts = args::Opt::from_args();
    
    match opts.subcmd {
	args::SubCommand::Init{dir} => {	    
	    let mut repo_path = dir;

	    let mut repo_absolute = std::fs::canonicalize(repo_path.clone())
		.with_context(|_| format!("Could not resolve path: {:?}", repo_path))?;
			      
	    repo_absolute.push(".berk.db");
			      
	    let repo = repo_absolute.to_str()
		.ok_or(failure::err_msg(format!("Not valid UTF-8: {:?}", repo_path)))?;
	    
	    berk::repo::initialize(repo)?

	},
	args::SubCommand::Add{files} => {

	    for file in &files {
		for entry in WalkDir::new(file) {
		    let entry = entry.unwrap();
		    let entry_type = entry.file_type();

		    if entry_type.is_file() {
			println!("{}", entry.path().display());
		    }
		}
	    }
	},
	args::SubCommand::Print{} => {

	},
    }
    Ok(())
}

