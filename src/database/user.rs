use diesel::{mysql::MysqlConnection, prelude::*};

use super::models::{NewUser, User};
use super::schema;

pub fn create_user(connection: &MysqlConnection, user_id: &str) -> User {
    use schema::users::dsl::{id, users};

    let user = NewUser {
        id: user_id.to_owned(),
    };

    diesel::insert_into(users)
        .values(&user)
        .execute(connection)
        .expect("Error saving user");

    users.order(id.desc()).first(connection).unwrap()
}
