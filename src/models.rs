use super::schema::{blob, stage};
use sha3::{Digest, Sha3_256};
use super::workspace::SanitizedPath;

#[derive(Queryable, Debug)]
pub struct Blob {
    pub blob_oid: Vec<u8>,
    pub blob_data: Vec<u8>,
}

#[derive(Queryable, Debug)]
pub struct Tree {
    pub tree_oid: Vec<u8>,
}

#[derive(Queryable, Debug)]
pub struct StageBlob {
    pub path: String,
    pub blob_oid: Vec<u8>,
}


#[derive(Insertable)]
#[table_name="blob"]
pub struct NewBlob {
    pub blob_oid: Vec<u8>,
    pub blob_data: Vec<u8>,
}

impl NewBlob {

    pub fn new(blob_data: Vec<u8>) -> NewBlob {
	let mut hasher = Sha3_256::new();
	hasher.input(&blob_data);	
	let blob_oid = hasher.result().to_vec();
	NewBlob {
	    blob_data,
	    blob_oid
	}
    }
}

#[derive(Insertable)]
#[table_name="stage"]
pub struct NewStageBlob {
    pub path: String,
    pub blob_oid: Vec<u8>,
}

impl NewStageBlob {

    pub fn new(path: SanitizedPath, blob_oid: Vec<u8>) -> NewStageBlob {
	let path = path.0.to_string();
	NewStageBlob {
	    path,
	    blob_oid
	}
    }
}


