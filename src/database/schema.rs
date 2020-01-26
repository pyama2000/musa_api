table! {
    credentials (id) {
        id -> Integer,
        user_id -> Varchar,
        token_id -> Integer,
    }
}

table! {
    tokens (id) {
        id -> Integer,
        access_token -> Varchar,
        refresh_token -> Varchar,
    }
}

table! {
    users (id) {
        id -> Varchar,
    }
}

joinable!(credentials -> tokens (token_id));
joinable!(credentials -> users (user_id));

allow_tables_to_appear_in_same_query!(credentials, tokens, users,);
