use diesel::{mysql::MysqlConnection, prelude::*};

use super::models::{NewToken, Token};
use super::schema;

pub fn create_token(
    connection: &MysqlConnection,
    access_token: &str,
    refresh_token: &str,
) -> Token {
    use schema::tokens::dsl::{id, tokens};

    let token = NewToken {
        access_token: access_token.to_owned(),
        refresh_token: refresh_token.to_owned(),
    };

    diesel::insert_into(tokens)
        .values(&token)
        .execute(connection)
        .expect("Error saving token");

    tokens.order(id.desc()).first(connection).unwrap()
}
