use diesel::{mysql::MysqlConnection, prelude::*};

use super::models::NewUser;
use super::schema;

pub fn create_user(connection: &MysqlConnection, user_id: &str) {
    use schema::users;

    let user = NewUser {
        id: user_id.to_owned(),
    };

    diesel::insert_into(users::table)
        .values(&user)
        .execute(connection)
        .expect("Error saving user");
}
