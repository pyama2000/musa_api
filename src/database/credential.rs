use diesel::{mysql::MysqlConnection, prelude::*};

use super::models::NewCredential;
use super::schema;

pub fn create_credential(connection: &MysqlConnection, user_id: &str, token_id: i32) {
    use schema::credentials;

    let credential = NewCredential {
        user_id: user_id.to_owned(),
        token_id,
    };

    diesel::insert_into(credentials::table)
        .values(&credential)
        .execute(connection)
        .expect("Error saving credential");
}
