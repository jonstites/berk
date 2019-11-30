table! {
    blob_objects (blob_oid) {
        blob_oid -> Binary,
        blob_data -> Binary,
    }
}

table! {
    tree_objects (tree_oid) {
        tree_oid -> Binary,
    }
}

allow_tables_to_appear_in_same_query!(
    blob_objects,
    tree_objects,
);
