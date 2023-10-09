// @generated automatically by Diesel CLI.

diesel::table! {
    documents (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
    }
}

diesel::table! {
    symbols (id) {
        id -> Int4,
        document -> Int4,
        name -> Varchar,
        terminality -> Bool,
    }
}

diesel::joinable!(symbols -> documents (document));

diesel::allow_tables_to_appear_in_same_query!(
    documents,
    symbols,
);
