CREATE TABLE IF NOT EXISTS blob(
  blob_oid BLOB NOT NULL PRIMARY KEY,
  blob_data BLOB NOT NULL
);
CREATE TABLE IF NOT EXISTS tree(
  tree_oid BLOB NOT NULL PRIMARY KEY
);
CREATE TABLE IF NOT EXISTS stage(
  path TEXT NOT NULL PRIMARY KEY,
  blob_oid BLOB NOT NULL
);