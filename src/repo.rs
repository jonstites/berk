use super::database::{Blob, StagedBlob};
use super::database;
use super::workspace::{AbsolutePath, Workspace};

use std::collections::HashSet;
use failure::ResultExt;
use exitfailure::ExitFailure;
use std::path::{Path, PathBuf};
use rusqlite::Connection;

pub const database_name: &str = ".berk.db";

pub struct Repo {
    database_directory: PathBuf,
    workspace: Workspace,
    database: Connection,    
}

impl Repo {

    pub fn load(working_directory: PathBuf) -> Result<Repo, ExitFailure> {
	let database_directory = Repo::find_database_directory(&working_directory)?;
	let workspace = Workspace::new(working_directory)?;

	let database_url = Repo::make_database_url(&database_directory)?;
	let database = Connection::open(database_url)?;
	
	Ok(Repo {database_directory, workspace, database})
    }

    pub fn initialize_database(database_directory: PathBuf) -> Result<(), ExitFailure> {
	let database_url = Repo::make_database_url(&database_directory)?;
	let mut database = Connection::open(database_url)?;	
	database::initialize(&mut database)?;
	Ok(())
    }

    pub fn add_files(&mut self, files: Vec<PathBuf>) -> Result<(), ExitFailure> {
	let files: HashSet<AbsolutePath> = self.workspace.walk(files)?;

	let mut transaction = self.database.transaction()?;
	for file in &files {
	    
	    let blob_data = self.workspace.read_file(file)?;
	    let blob = Blob::new(blob_data);
	    database::add_blob(&transaction, &blob)?;

	    let path = self.workspace.sanitize_path(file, Path::new(&self.database_directory))?;
	    let staged_blob = StagedBlob::new(path,blob.blob_oid.to_vec());
	    database::stage(&transaction, staged_blob)?;
	}
	transaction.commit()?;
	Ok(())
    }

    pub fn find_database_directory(mut directory: &Path) -> Result<PathBuf, ExitFailure> {
	loop {
	    let database = Repo::make_database_url(&directory)?;
	    if Path::new(&database).is_file() {
		return Ok(directory.to_path_buf());
	    } else {
		let parent = directory.parent()
		    .ok_or(failure::err_msg("not a berk repo"))?;
		directory = parent;
	    }
	}
    }

    pub fn make_database_url(directory: &Path) -> Result<String, ExitFailure> {
	let url = directory.join(database_name);
	let database_url = url.to_str()
	    .ok_or(failure::err_msg(format!("could not convert to UTF-8 string: {:?}", url)))?;
	Ok(database_url.to_string())
    }

    pub fn read_blobs(&self) -> Result<Vec<Blob>, ExitFailure> {
	let blobs = database::read_blobs(&self.database)?;
	Ok(blobs)
    }

    pub fn read_stage(&self) -> Result<Vec<StagedBlob>, ExitFailure> {
	let blobs = database::read_stage(&self.database)?;
	Ok(blobs)
    }
}
