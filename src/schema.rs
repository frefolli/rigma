// @generated automatically by Diesel CLI.

diesel::table! {
    assets (id) {
        id -> Int4,
        document -> Int4,
    }
}

diesel::table! {
    branches (id) {
        id -> Int4,
        production -> Int4,
        symbol -> Int4,
        index -> Int4,
    }
}

diesel::table! {
    documents (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
    }
}

diesel::table! {
    productions (id) {
        id -> Int4,
        asset -> Int4,
        left -> Int4,
    }
}

diesel::table! {
    symbols (id) {
        id -> Int4,
        asset -> Int4,
        name -> Varchar,
        terminality -> Bool,
    }
}

diesel::joinable!(assets -> documents (document));
diesel::joinable!(branches -> productions (production));
diesel::joinable!(branches -> symbols (symbol));
diesel::joinable!(productions -> assets (asset));
diesel::joinable!(productions -> symbols (left));
diesel::joinable!(symbols -> assets (asset));

diesel::allow_tables_to_appear_in_same_query!(
    assets,
    branches,
    documents,
    productions,
    symbols,
);
