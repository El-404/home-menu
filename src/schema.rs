// @generated automatically by Diesel CLI.

diesel::table! {
    token_key (id) {
        id -> Int4,
        user_id -> Int4,
        token -> Bytea,
        expiration -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password_hash -> Varchar,
    }
}

diesel::joinable!(token_key -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(token_key, users,);
