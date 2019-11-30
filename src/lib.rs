#![deny(unsafe_code)]

pub mod schema;
pub mod models;


#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use failure::ResultExt;
use exitfailure::ExitFailure;

pub fn eat(_data: &[u8]) {

}

pub fn establish_connection(database_url: &str) -> Result<SqliteConnection, ExitFailure> {
    let connection = SqliteConnection::establish(database_url)
        .with_context(|_| format!("Error connecting to {}", database_url))?;
    Ok(connection)
}
