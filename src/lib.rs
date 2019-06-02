use libflate::zlib::Encoder;
use sha1::{Digest, Sha1};
use std::fs::{create_dir_all, rename, File};
use std::path::PathBuf;
use std::result;
pub type Result<T> = result::Result<T, BerkError>;

use rand;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub enum BerkError {
    NotAGitRepo,
    IOError(std::io::Error),
    OsStringError(std::ffi::OsString),
}

impl From<std::io::Error> for BerkError {
    fn from(err: std::io::Error) -> BerkError {
        BerkError::IOError(err)
    }
}

impl From<std::ffi::OsString> for BerkError {
    fn from(err: std::ffi::OsString) -> BerkError {
        BerkError::OsStringError(err)
    }
}

#[derive(Debug)]
pub struct Commit {
    oid: [u8; 20],
    tree_oid: [u8; 20],
    author_name: String,
    author_email: String,
    timestamp: String,
    message: String,
    data: Vec<u8>,
}

impl Commit {
    pub fn new(
        tree_oid: [u8; 20],
        author_name: String,
        author_email: String,
        timestamp: String,
        message: String,
    ) -> Commit {
        let hex_tree_oid: String = tree_oid
            .iter()
            .map(|&byte| format!("{:02x}", byte))
            .collect();

        let raw_data: Vec<u8> = [
            b"tree ",
            hex_tree_oid.as_bytes(),
            format!(
                "\nauthor {} {} {}\ncommiter {} {} {}\n\n{}",
                author_name, author_email, timestamp, author_name, author_email, timestamp, message
            )
            .as_bytes(),
        ]
        .concat();

        let data: Vec<u8> = [
            b"commit ",
            format!("{}\0", raw_data.len()).as_bytes(),
            &raw_data,
        ]
        .concat();

        let hash = Sha1::digest(&data);
        let mut oid = [0_u8; 20];
        oid.copy_from_slice(hash.as_slice());
        Commit {
            oid,
            tree_oid,
            author_name,
            author_email,
            timestamp,
            message,
            data,
        }
    }
}

impl Object for Commit {
    fn get_type(&self) -> &str {
        "commit"
    }

    fn get_data(&self) -> &[u8] {
        &self.data
    }

    fn get_oid(&self) -> [u8; 20] {
        self.oid
    }
}

#[derive(Debug)]
pub struct Tree {
    oid: [u8; 20],
    entries: Vec<(String, [u8; 20])>,
    data: Vec<u8>,
}

impl Tree {
    pub fn new(mut entries: Vec<(String, [u8; 20])>) -> Tree {
        entries.sort_by_key(|(path, _oid)| path.clone());
        let raw_data: Vec<u8> = entries
            .iter()
            .map(|(path, oid)| [b"100644 ", path.as_bytes(), b"\0", oid].concat())
            .flatten()
            .collect();
        let data: Vec<u8> = [
            b"tree ",
            format!("{}\0", raw_data.len()).as_bytes(),
            &raw_data,
        ]
        .concat();

        let hash = Sha1::digest(&data);
        let mut oid = [0_u8; 20];
        oid.copy_from_slice(hash.as_slice());

        Tree { oid, entries, data }
    }
}
impl Object for Tree {
    fn get_type(&self) -> &str {
        "tree"
    }

    fn get_data(&self) -> &[u8] {
        &self.data
    }

    fn get_oid(&self) -> [u8; 20] {
        self.oid
    }
}

pub struct Blob {
    oid: [u8; 20],
    _raw_data: Vec<u8>,
    data: Vec<u8>,
}

impl Blob {
    pub fn new(raw_data: Vec<u8>) -> Blob {
        let data = [
            b"blob ",
            format!("{}\0", raw_data.len()).as_bytes(),
            &raw_data,
        ]
        .concat();

        let hash = Sha1::digest(&data);
        let mut oid = [0_u8; 20];
        oid.copy_from_slice(hash.as_slice());
        Blob {
            oid,
            _raw_data: raw_data,
            data,
        }
    }
}

impl Object for Blob {
    fn get_type(&self) -> &str {
        "blob"
    }

    fn get_data(&self) -> &[u8] {
        &self.data
    }

    fn get_oid(&self) -> [u8; 20] {
        self.oid
    }
}

pub trait Object {
    fn get_data(&self) -> &[u8];
    fn get_type(&self) -> &str;
    fn get_oid(&self) -> [u8; 20];
}

pub struct ObjectDatabase {
    pub path: PathBuf,
}

impl ObjectDatabase {
    pub fn new(path: PathBuf) -> ObjectDatabase {
        ObjectDatabase { path }
    }

    pub fn write_object(&self, object: &impl Object) -> Result<()> {
        let hex_hash: String = object
            .get_oid()
            .iter()
            .map(|&byte| format!("{:02x}", byte))
            .collect();

        let parent = self.path.join(hex_hash[..2].to_string());

        create_dir_all(&parent)?;

        let randomness: String = thread_rng().sample_iter(&Alphanumeric).take(6).collect();

        let mut filename = "tmp_obj_".to_string();
        filename.push_str(&randomness);

        let tempfilename = parent.join(filename);

        let tempfile = File::create(&tempfilename)?;
        let mut e = Encoder::new(tempfile)?;
        std::io::copy(&mut object.get_data(), &mut e)?;
        e.finish().into_result()?;

        let object_filename = parent.join(hex_hash[2..].to_string());

        rename(tempfilename, object_filename)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_blob() {
        let contents = "what is up, doc?".as_bytes().to_vec();
        let blob = Blob::new(contents);
        let hex_hash: String = blob
            .get_oid()
            .iter()
            .map(|&byte| format!("{:02x}", byte))
            .collect();
        assert_eq!(hex_hash, "bd9dbf5aae1a3862dd1526723246b20206e5fc37");
    }

}
