use sha1::{Digest, Sha1};
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

pub enum ObjectType {
    Commit,
    Tree,
    Blob,
    Tag,
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

pub fn object_from_file(filename: &str) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(filename)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    Ok(contents)
}

pub fn hash_object(object_type: &ObjectType, object: &[u8]) -> String {
    let hash = Sha1::new()
        .chain(format!("{}", object_type))
        .chain(b" ")
        .chain(object.len().to_string())
        .chain(b"\0")
        .chain(object)
        .result();

    format!("{:x}", hash)
}
