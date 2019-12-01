table! {
    blob (blob_oid) {
        blob_oid -> Binary,
        blob_data -> Binary,
    }
}

table! {
    stage (path) {
        path -> Text,
        blob_oid -> Binary,
    }
}

table! {
    tree (tree_oid) {
        tree_oid -> Binary,
    }
}

allow_tables_to_appear_in_same_query!(
    blob,
    stage,
    tree,
);
