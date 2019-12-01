use rusqlite::{params, Connection, Result, Transaction};
use sha3::{Digest, Sha3_256};
use super::workspace::SanitizedPath;

pub struct Database{
    connection: Connection,
}

impl Database {

    pub fn new(database_path: &str) -> Result<Database> {
	let connection = Connection::open(database_path)?;
	Ok(Database {
	    connection
	})
    }

    pub fn initialize(&self) -> Result<()> {
	self.connection.execute_batch(
	    "BEGIN;
            CREATE TABLE IF NOT EXISTS blob(blob_oid BLOB NOT NULL PRIMARY KEY, blob_data BLOB NOT NULL);
            CREATE TABLE IF NOT EXISTS tree(tree_oid BLOB NOT NULL PRIMARY KEY);
            CREATE TABLE IF NOT EXISTS stage(path TEXT NOT NULL PRIMARY KEY, blob_oid BLOB NOT NULL);
            END;"
	)?;
	Ok(())
    }

    pub fn transaction(&mut self) -> Result<Database> {
	
    }

    pub fn add_blob(&self, blob: &Blob) -> Result<Statement> { 
	tx.execute(
	    "INSERT INTO blob (blob_oid, blob_data) VALUES (?1, ?2)",
	    params![blob.blob_oid, blob.blob_data]
	)?;
	Ok(tx)
    }

    pub fn stage(&'a self, path: SanitizedPath, blob_oid: Vec<u8>, tx: Transaction<'b>) -> Result<Transaction<'b>> {
	tx.execute(
	    "INSERT INTO stage (path, blob_oid) VALUES (?1, ?2)",
	    params![path.0.to_string(), blob_oid]
	)?;
	Ok(tx)
    }
    
}

pub struct Blob {
    pub blob_oid: Vec<u8>,
    pub blob_data: Vec<u8>,
}

impl Blob {

    pub fn new(blob_data: Vec<u8>) -> Blob {
	let mut hasher = Sha3_256::new();
	hasher.input(&blob_data);	
	let blob_oid = hasher.result().to_vec();
	Blob {
	    blob_data,
	    blob_oid
	}
    }
}
