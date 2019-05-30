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
}

impl From<std::io::Error> for BerkError {
    fn from(err: std::io::Error) -> BerkError {
        BerkError::IOError(err)
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

    fn get_oid(&self) -> &[u8] {
        &self.oid
    }
}

pub trait Object {
    fn get_data(&self) -> &[u8];
    fn get_type(&self) -> &str;
    fn get_oid(&self) -> &[u8];
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
