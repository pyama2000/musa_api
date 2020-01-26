use diesel::{mysql::MysqlConnection, prelude::*};

use super::models::{Credential, NewCredential};
use super::schema;

pub fn create_credential(connection: &MysqlConnection, user_id: &str, token_id: i32) -> Credential {
    use schema::credentials::dsl::{credentials, id};

    let credential = NewCredential {
        user_id: user_id.to_owned(),
        token_id,
    };

    diesel::insert_into(credentials)
        .values(&credential)
        .execute(connection)
        .expect("Error saving credential");

    credentials.order(id.desc()).first(connection).unwrap()
}
