use std::collections::HashSet;
use std::path::{Path, PathBuf};
use exitfailure::ExitFailure;
use failure::ResultExt;
use walkdir::WalkDir;
use std::fs::File;
use std::io::Read;
use std::path::StripPrefixError;

pub struct Workspace {
    working_directory: PathBuf,
}

impl Workspace {
    
    pub fn new(working_directory: PathBuf) -> std::io::Result<Workspace> {
	let working_directory = std::fs::canonicalize(working_directory)?;
	Ok(Workspace {
	    working_directory
	})
    }

    pub fn walk(&self, files: Vec<PathBuf>) -> std::io::Result<HashSet<AbsolutePath>> {
	let mut all_files = HashSet::new();
	for file in &files {
            for entry in WalkDir::new(file) {
		let entry = entry?;
		let entry_type = entry.file_type();

		if entry_type.is_file() {
		    let path: PathBuf = entry.path().to_path_buf();
		    let path = std::fs::canonicalize(path)?;
		    all_files.insert(AbsolutePath(path));
		}
	    }
	}
	
	Ok(all_files)
    }

    pub fn read_file(&self, file: &AbsolutePath) -> std::io::Result<Vec<u8>> {
        let mut file = File::open(Path::new(&file.0))?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;
	Ok(contents)
    }

    pub fn sanitize_path<P: AsRef<Path>>(&self, file: &AbsolutePath, relative_dir: P) -> Result<SanitizedPath, ExitFailure> {
	let path = Path::new(&file.0);
	let path = path.strip_prefix(relative_dir)?;
	let path = path.to_str()
	    .ok_or(failure::err_msg(format!("could not read as UTF-8: {:?}", path)))?;
	Ok(SanitizedPath(path.to_string()))
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct AbsolutePath(PathBuf);
pub struct SanitizedPath(pub String);
