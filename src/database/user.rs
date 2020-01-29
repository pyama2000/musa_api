use diesel::prelude::*;

use super::models::{NewUser, Selectable, User};
use super::schema;

pub fn create_user(connection: &PgConnection, user_id: &str) -> User {
    use schema::users::dsl::users;

    let user = NewUser {
        user_id: user_id.to_owned(),
    };

    diesel::insert_into(users)
        .values(&user)
        .get_result(connection)
        .expect("Error saving user")
}

pub fn find_user(
    connection: &PgConnection,
    spotify_user_id: &str,
) -> Result<Option<User>, diesel::result::Error> {
    use schema::users::dsl::*;

    users
        .select(User::columns())
        .filter(user_id.eq(spotify_user_id.to_owned()))
        .get_result::<User>(connection)
        .optional()
}
