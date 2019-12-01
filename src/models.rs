use super::schema::blob_objects;

#[derive(Queryable, Debug)]
pub struct BlobObject {
    pub blob_oid: Vec<u8>,
    pub blob_data: Vec<u8>,
}

#[derive(Insertable)]
#[table_name="blob_objects"]
pub struct NewBlobObject {
    pub blob_oid: Vec<u8>,
    pub blob_data: Vec<u8>,
}


#[derive(Queryable)]
pub struct TreeObject {
    pub tree_oid: Vec<u8>,
}
