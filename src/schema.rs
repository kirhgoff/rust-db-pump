table! {
    input_table (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

table! {
    output_table (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    input_table,
    output_table,
);
