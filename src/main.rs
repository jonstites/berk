use berk::{Database, GitBlob};

#[macro_use]
extern crate error_chain;
mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{}
}

use errors::*;


fn main() -> Result<()> {

    let database = Database::new("./.berk/berk.db3")
        .chain_err(|| "unable to open database")?;

    database.init()
        .chain_err(|| "unable to create database tables")?;

    let git_blob = GitBlob::from_file("hello.txt")
        .chain_err(|| "unable to read file")?;

    database.commit_blob(git_blob)
        .chain_err(|| "unable to commit git blob")?;

    Ok(())
}
