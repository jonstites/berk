use diesel_migrations::embed_migrations;
use exitfailure::ExitFailure;
use failure::ResultExt;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

embed_migrations!();

// This will create the repo and its tables.
pub fn initialize(repo: &str) -> Result<(), ExitFailure> {
    let connection = establish_connection(repo)?;
    embedded_migrations::run(&connection)
        .with_context(|_| format!("could not initialize repo: {}", repo))?;
    Ok(())
}

pub fn establish_connection(url: &str) -> Result<SqliteConnection, ExitFailure> {
    let connection = SqliteConnection::establish(url)
        .with_context(|_| format!("error connecting to database: {}", url))?;
    Ok(connection)
}

