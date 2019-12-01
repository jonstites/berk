use super::database::{Blob, Database};
use super::workspace::{AbsolutePath, Workspace};

use std::collections::HashSet;
use failure::ResultExt;
use exitfailure::ExitFailure;
use std::path::{Path, PathBuf};


pub const database_name: &str = ".berk.db";

pub struct Repo {
    database_directory: PathBuf,
    workspace: Workspace,
    database: Database,    
}

impl Repo {

    pub fn load(working_directory: PathBuf) -> Result<Repo, ExitFailure> {
	let database_directory = Repo::find_database_directory(&working_directory)?;
	let workspace = Workspace::new(working_directory)?;

	let database_url = Repo::make_database_url(&database_directory)?;
	let database = Database::new(&database_url)?;
	
	Ok(Repo {database_directory, workspace, database})
    }

    pub fn initialize_database(database_directory: PathBuf) -> Result<(), ExitFailure> {
	let database_url = Repo::make_database_url(&database_directory)?;
	let database = Database::new(&database_url)?;
	database.initialize()?;
	Ok(())
    }

    pub fn add_files(&mut self, files: Vec<PathBuf>) -> Result<(), ExitFailure> {
	let files: HashSet<AbsolutePath> = self.workspace.walk(files)?;

	let mut transaction = self.database.transaction()?;
	for file in &files {
	    
	    let blob_data = self.workspace.read_file(file)?;
	    let blob = Blob::new(blob_data);
	    transaction = self.database.add_blob(&blob, transaction)?;
	    
	    let path = self.workspace.sanitize_path(file, Path::new(&self.database_directory))?;
	    transaction = self.database.stage(path, blob.blob_oid.to_vec(), transaction)?;
	}
	self.database.commit(transaction)?;
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
}
