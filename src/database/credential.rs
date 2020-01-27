use diesel::prelude::*;

use super::models::{Credential, NewCredential};
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
