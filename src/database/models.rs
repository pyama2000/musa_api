#[derive(Clone, Debug, Default, Queryable)]
pub struct Credential {
    id: i32,
    user_id: String,
    token_id: String,
}

#[derive(Clone, Debug, Default, Queryable)]
pub struct Token {
    id: i32,
    access_token: String,
    token_id: String,
}

#[derive(Clone, Debug, Default, Queryable)]
pub struct User {
    id: String,
}
