use super::schema::*;

#[derive(Clone, Debug, Default, Queryable)]
pub struct Credential {
    id: i32,
    user_id: String,
    token_id: String,
}

#[derive(Clone, Debug, Default, Associations, Identifiable, Queryable)]
pub struct Token {
    id: i32,
    access_token: String,
    token_id: String,
}

#[derive(Clone, Debug, Default, Associations, Identifiable, Queryable)]
pub struct User {
    id: String,
}

#[derive(Clone, Debug, Default, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub id: String,
}
