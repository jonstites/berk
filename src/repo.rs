use super::workspace::{AbsolutePath, Workspace};

use std::collections::HashSet;
use failure::ResultExt;
use exitfailure::ExitFailure;
use std::path::{Path, PathBuf};
use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use super::models::{Blob, NewBlob, StageBlob, NewStageBlob};
use diesel_migrations::embed_migrations;

embed_migrations!();

pub const database_name: &str = ".berk.db";

pub struct Repo {
    database_directory: PathBuf,
    workspace: Workspace,
    pub database: SqliteConnection,
}

impl Repo {

    pub fn initialize_database(database_directory: PathBuf) -> Result<(), ExitFailure> {
	let connection = Repo::establish_connection(&database_directory)?;
	embedded_migrations::run(&connection)
	    .with_context(|_| format!("could not initialize repository: {:?}", database_directory))?;
	Ok(())
    }


    pub fn load(working_directory: PathBuf) -> Result<Repo, ExitFailure> {
	let database_directory = Repo::find_database_directory(&working_directory)?;
	let workspace = Workspace::new(working_directory)?;

	let database = Repo::establish_connection(&database_directory)?;
	Ok(Repo {database_directory, workspace, database})
    }

    pub fn add_files(&mut self, files: Vec<PathBuf>) -> Result<(), ExitFailure> {
	use super::schema::{blob, stage};
	
	let files: HashSet<AbsolutePath> = self.workspace.walk(files)?;

	self.database.transaction::<(), ExitFailure, _>(|| {
	    for file in &files {
		
		let blob_data = self.workspace.read_file(file)?;
		let blob = NewBlob::new(blob_data);
		diesel::replace_into(blob::table)
		    .values(&blob)
		    .execute(&self.database)?;


		let path = self.workspace.sanitize_path(file, Path::new(&self.database_directory))?;

		// one failure mode		
		let mut remove_prefix = path.0.clone();
		remove_prefix.push(std::path::MAIN_SEPARATOR);
		remove_prefix.push('%');
		diesel::delete(stage::table.filter(super::schema::stage::dsl::path.like(remove_prefix)))
		    .execute(&self.database)?;

		// need second failure mode too
		let mut path2 = Path::new(&path.0);
		while let Some(parent) = path2.parent() {
		    let parent_str = parent.to_str()
			.ok_or(failure::err_msg(format!("could not convert to UTF-8: {:?}", parent)))?;

		    diesel::delete(stage::table.filter(super::schema::stage::dsl::path.eq(parent_str)))
			.execute(&self.database)?;
		    path2 = parent;
		}

		// And insert into index
		let staged_blob = NewStageBlob::new(path, blob.blob_oid.to_vec());
		diesel::replace_into(stage::table)
		    .values(&staged_blob)
		    .execute(&self.database)?;
	    }
	    Ok(())
	})?;

	Ok(())
    }

    pub fn find_database_directory(mut directory: &Path) -> Result<PathBuf, ExitFailure> {
	loop {
	    let database = directory.join(database_name);
	    if Path::new(&database).is_file() {
		return Ok(directory.to_path_buf());
	    } else {
		let parent = directory.parent()
		    .ok_or(failure::err_msg("not a berk repo"))?;
		directory = parent;
	    }
	}
    }

    pub fn establish_connection(directory: &Path) -> Result<SqliteConnection, ExitFailure> {
	let url = directory.join(database_name);
	let url_str = url.to_str()
	    .ok_or(failure::err_msg(format!("Could not convert to UTF-8 String: {:?}", url)))?;
	let connection = SqliteConnection::establish(url_str)
	    .with_context(|_| format!("count not connect to database: {:?}", url))?;
	Ok(connection)
    }
}
