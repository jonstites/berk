#![deny(unsafe_code)]

pub mod args;
pub mod models;
pub mod repo;
pub mod schema;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use exitfailure::ExitFailure;
use failure::ResultExt;


pub fn eat(_data: &[u8]) {}

use self::models::{BlobObject, NewBlobObject};

pub fn insert_blob(connection: &SqliteConnection, data: Vec<u8>) -> Result<(), ExitFailure> {
    use schema::blob_objects;

    let new_blob = NewBlobObject {
	blob_oid: data.clone(),
	blob_data: data.to_vec(),
    };

    diesel::insert_into(blob_objects::table)
	.values(&new_blob)
	.execute(connection)?;

    Ok(())
}
