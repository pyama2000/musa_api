use diesel::prelude::*;

use super::models::{NewToken, Token};
use super::schema;

pub fn create_token(connection: &PgConnection, access_token: &str, refresh_token: &str) -> Token {
    use schema::tokens::dsl::tokens;

    let token = NewToken {
        access_token: access_token.to_owned(),
        refresh_token: refresh_token.to_owned(),
    };

    diesel::insert_into(tokens)
        .values(&token)
        .get_result(connection)
        .expect("Error saving token")
}
