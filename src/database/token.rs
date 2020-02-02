use diesel::prelude::*;

use super::models::{NewToken, Selectable, Token};
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

pub fn update_token(
    connection: &PgConnection,
    token_id: i32,
    new_access_token: &str,
    new_refresh_token: &str,
) -> Result<Token, diesel::result::Error> {
    use schema::tokens::dsl::*;

    diesel::update(tokens.filter(id.eq(token_id)))
        .set((
            access_token.eq(new_access_token),
            refresh_token.eq(new_refresh_token),
        ))
        .get_result(connection)
}
