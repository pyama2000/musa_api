use diesel::prelude::*;

use super::models::{NewUser, User};
use super::schema;

pub fn create_user(connection: &PgConnection, user_id: &str) -> User {
    use schema::users::dsl::users;

    let user = NewUser {
        id: user_id.to_owned(),
    };

    diesel::insert_into(users)
        .values(&user)
        .get_result(connection)
        .expect("Error saving user")
}
