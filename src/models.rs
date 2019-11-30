#[derive(Queryable, Debug)]
pub struct BlobObject {
    pub blob_oid: Vec<u8>,
    pub blob_data: Vec<u8>,
}

#[derive(Queryable)]
pub struct TreeObject {
    pub tree_oid: Vec<u8>,
}
