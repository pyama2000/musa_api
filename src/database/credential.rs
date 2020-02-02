use diesel::prelude::*;

use super::models::{Credential, NewCredential, Selectable};
use super::schema;

pub fn create_credential(connection: &PgConnection, user_id: &str, token_id: i32) -> Credential {
    use schema::credentials::dsl::credentials;

    let credential = NewCredential {
        user_id: user_id.to_owned(),
        token_id,
    };

    diesel::insert_into(credentials)
        .values(&credential)
        .get_result(connection)
        .expect("Error saving credential")
}

pub fn find_credential_by_user_id(
    connection: &PgConnection,
    spotify_user_id: &str,
) -> Result<Option<Credential>, diesel::result::Error> {
    use schema::credentials::dsl::*;

    credentials
        .select(Credential::columns())
        .filter(user_id.eq(spotify_user_id.to_owned()))
        .get_result(connection)
        .optional()
}

pub fn update_token_id(
    connection: &PgConnection,
    credential_id: i32,
    new_token_id: i32,
) -> Result<Credential, diesel::result::Error> {
    use schema::credentials::dsl::{credentials, id, token_id};

    diesel::update(credentials.filter(id.eq(credential_id)))
        .set(token_id.eq(new_token_id))
        .get_result(connection)
}
