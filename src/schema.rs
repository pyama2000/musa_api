table! {
    sessions (id) {
        id -> Integer,
        session_id -> Integer,
        token_id -> Integer,
        last_access -> Datetime,
    }
}

table! {
    tokens (id) {
        id -> Integer,
        access_token -> Text,
        refresh_token -> Text,
    }
}

joinable!(sessions -> tokens (token_id));

allow_tables_to_appear_in_same_query!(
    sessions,
    tokens,
);
