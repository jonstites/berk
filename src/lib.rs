use sha1::{Digest, Sha1};
use std::fmt;
use std::fs::{File, create_dir_all};
use std::io::prelude::*;
use std::path::Path;
use std::result;

pub type Result<T> = result::Result<T, BerkError>;

#[derive(Debug)]
pub enum BerkError {
    NotAGitRepo,
    IOError(std::io::Error),
}

impl From<std::io::Error> for BerkError {
    fn from(err: std::io::Error) -> BerkError {
        BerkError::IOError(err)
    }
}

pub enum ObjectType {
    Commit,
    Tree,
    Blob,
    Tag,
}

pub struct Object {
    object_type: ObjectType,
    contents: Vec<u8>,
}

impl Object {
    pub fn with_header(&self) -> Vec<u8> {
        let mut v = Vec::new();
        v.extend_from_slice(
            format!("{} {}\0", self.object_type, self.contents.len().to_string()).as_bytes(),
        );
        v.extend_from_slice(&self.contents);
        v
    }
}

impl fmt::Display for ObjectType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ObjectType::Commit => write!(f, "commit"),
            ObjectType::Tree => write!(f, "tree"),
            ObjectType::Blob => write!(f, "blob"),
            ObjectType::Tag => write!(f, "tag"),
        }
    }
}

pub fn object_from_file(filename: &str) -> std::io::Result<Object> {
    let mut file = File::open(filename)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    let object_type = ObjectType::Blob;
    Ok(Object {
        object_type,
        contents,
    })
}

pub fn hash_object(object: &Object) -> Vec<u8> {
    Sha1::new().chain(object.with_header()).result().to_vec()
}

pub fn is_git_src(path: &Path) -> bool {
    // TODO: there is supposed to be a check that HEAD is valid
    path.join(".berk/HEAD").is_file()
        && path.join(".git/objects").is_dir()
        && path.join(".git/refs").is_dir()
}

pub fn find_git_src(path: &Path) -> Result<&Path> {
    let ancestors = path.ancestors();
    for ancestor in ancestors {
        if is_git_src(ancestor) {
            return Ok(ancestor);
        }
    }
    Err(BerkError::NotAGitRepo)
}

pub fn initialize_repo(path: &Path) -> Result<()> {
    let repo_dir = path.join(".berk");
    create_dir_all(&repo_dir)?;

    let branches_dir = repo_dir.join("branches");
    create_dir_all(branches_dir)?;
    let objects_dir = repo_dir.join("objects");
    create_dir_all(objects_dir)?;
    let refs_dir = repo_dir.join("refs");
    create_dir_all(refs_dir)?;

    let head_file = repo_dir.join("HEAD");
    std::fs::write(head_file, "ref: refs/heads/master\n")?;
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_hash_object() {
        let contents = "what is up, doc?".as_bytes().to_vec();
        let object_type = ObjectType::Blob;
        let object = Object {
            contents,
            object_type,
        };
        let hash = hash_object(&object);
        let hex_hash: String = hash.iter().map(|&byte| format!("{:02x}", byte)).collect();
        assert_eq!(hex_hash, "bd9dbf5aae1a3862dd1526723246b20206e5fc37");
    }

}
