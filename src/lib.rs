#![deny(unsafe_code)]

use rusqlite::types::ToSql;
use rusqlite::{Connection, Result, NO_PARAMS};
use crypto::sha3::Sha3;
use crypto::digest::Digest;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct GitBlob {
    oid: String,
    data: Vec<u8>,
}

impl GitBlob {

    pub fn new(data: Vec<u8>) -> GitBlob {
        // Calculate the sha-256 of the bytes of data,
        // then store the result as a String in hex format
        let mut hasher = Sha3::sha3_256();
        hasher.input(&data);

        let oid = hasher.result_str();
        
        GitBlob {
            oid,
            data
        }
    }

    pub fn from_file(path: &str) -> std::io::Result<GitBlob> {
        let mut file = File::open(path)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;
        Ok(GitBlob::new(contents))        
    }
}

pub struct Database {
    connection: Connection,
}

impl Database {

    pub fn new(path: &str) -> Result<Database> {
        let connection = Connection::open(path)?;
        Ok(Database { connection })
    }

    pub fn init(&self) -> Result<()> {
        self.init_blob_db()?;
        Ok(())
    }

    fn init_blob_db(&self) -> Result<()> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS git_blob (
                  oid              TEXT PRIMARY KEY,
                  data             BLOB
                  )",
            NO_PARAMS,
        )?;

        Ok(())
    }

    pub fn commit_blob(&self, git_blob: GitBlob) -> Result<()> {
        self.connection.execute(
            "INSERT OR IGNORE INTO git_blob (oid, data)
                  VALUES (?1, ?2)",
            &[&git_blob.oid, &git_blob.data as &ToSql],
        )?;

        Ok(())
    }
}
