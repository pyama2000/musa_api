table! {
    credentials (id) {
        id -> Int4,
        user_id -> Varchar,
        token_id -> Int4,
    }
}

table! {
    tokens (id) {
        id -> Int4,
        access_token -> Varchar,
        refresh_token -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        user_id -> Varchar,
    }
}

joinable!(credentials -> tokens (token_id));

allow_tables_to_appear_in_same_query!(credentials, tokens, users,);
