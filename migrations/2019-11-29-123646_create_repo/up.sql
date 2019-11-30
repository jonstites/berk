CREATE TABLE blob_objects (
       blob_oid BLOB PRIMARY KEY NOT NULL,
       blob_data BLOB NOT NULL
);

CREATE TABLE tree_objects (
       tree_oid BLOB PRIMARY KEY NOT NULL
);
