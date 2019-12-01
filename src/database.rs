use rusqlite::{params, Connection, Result, Transaction, NO_PARAMS};
use sha3::{Digest, Sha3_256};
use super::workspace::SanitizedPath;


pub fn initialize(connection: &mut Connection) -> Result<()> {
    connection.execute_batch(
	"BEGIN;
        CREATE TABLE IF NOT EXISTS blob(blob_oid BLOB NOT NULL PRIMARY KEY, blob_data BLOB NOT NULL);
        CREATE TABLE IF NOT EXISTS tree(tree_oid BLOB NOT NULL PRIMARY KEY);
        CREATE TABLE IF NOT EXISTS stage(path TEXT NOT NULL PRIMARY KEY, blob_oid BLOB NOT NULL);
        END;"
    )
}

pub fn add_blob(connection: &Connection, blob: &Blob) -> Result<()> { 
    connection.execute(
	"INSERT OR REPLACE INTO blob (blob_oid, blob_data) VALUES (?1, ?2)",
	params![blob.blob_oid, blob.blob_data]
    )?;
    Ok(())
}

pub fn stage(connection: &Connection, blob: StagedBlob) -> Result<()> {
    connection.execute(
	"INSERT OR REPLACE INTO stage (path, blob_oid) VALUES (?1, ?2)",
	params![blob.path, blob.blob_oid]
    )?;
    Ok(())
}

pub fn read_blobs(connection: &Connection) -> Result<Vec<Blob>> {
    let mut statement = connection.prepare("SELECT blob_oid, blob_data FROM blob")?;
    let rows = statement.query_map(NO_PARAMS, |row| Ok(Blob {
	blob_oid: row.get(0)?,
	blob_data: row.get(1)?,
    }))?;

    let mut blobs = Vec::new();
    for blob in rows {
	blobs.push(blob?);
    }
    Ok(blobs)
}

pub fn read_stage(connection: &Connection) -> Result<Vec<StagedBlob>> {
    let mut statement = connection.prepare("SELECT path, blob_oid FROM stage")?;
    let rows = statement.query_map(NO_PARAMS, |row| Ok(StagedBlob {
	path: row.get(0)?,
	blob_oid: row.get(1)?,	
    }))?;

    let mut blobs = Vec::new();
    for blob in rows {
	blobs.push(blob?);
    }
    Ok(blobs)
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct StagedBlob {
    pub path: String,
    pub blob_oid: Vec<u8>,
}

impl StagedBlob {

    pub fn new(path: SanitizedPath, blob_oid: Vec<u8>) -> StagedBlob {
	let path = path.0.to_string();
	StagedBlob {
	    path,
	    blob_oid
	}
    }
}
