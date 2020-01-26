use super::schema::*;

#[derive(Clone, Debug, Default, Associations, Identifiable, Queryable)]
pub struct Credential {
    pub id: i32,
    pub user_id: String,
    pub token_id: i32,
}

#[derive(Clone, Debug, Default, Insertable)]
#[table_name = "credentials"]
pub struct NewCredential {
    pub user_id: String,
    pub token_id: i32,
}

#[derive(Clone, Debug, Default, Associations, Identifiable, Queryable)]
pub struct Token {
    pub id: i32,
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Clone, Debug, Default, Insertable)]
#[table_name = "tokens"]
pub struct NewToken {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Clone, Debug, Default, Associations, Identifiable, Queryable)]
pub struct User {
    pub id: String,
}

#[derive(Clone, Debug, Default, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub id: String,
}
