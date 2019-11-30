use diesel_migrations::embed_migrations;
use failure::ResultExt;
use exitfailure::ExitFailure;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

embed_migrations!();

// This will create the repo and its tables.
pub fn initialize(repo: &str) -> Result<(), ExitFailure> {
    let connection = establish_connection(repo)?;
    embedded_migrations::run(&connection)
	.with_context(|_| format!("could not initialize repo"))?;
    Ok(())
}

pub fn establish_connection(url: &str) -> Result<SqliteConnection, ExitFailure> {
    let connection = SqliteConnection::establish(url)
        .with_context(|_| format!("Error connecting to {}", url))?;
    Ok(connection)
}
