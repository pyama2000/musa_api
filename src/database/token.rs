use diesel::{mysql::MysqlConnection, prelude::*};

use super::models::NewToken;
use super::schema;

pub fn create_token(connection: &MysqlConnection, access_token: &str, refresh_token: &str) {
    use schema::tokens;

    let token = NewToken {
        access_token: access_token.to_owned(),
        refresh_token: refresh_token.to_owned(),
    };

    diesel::insert_into(tokens::table)
        .values(&token)
        .execute(connection)
        .expect("Error saving token");
}
