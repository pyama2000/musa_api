use diesel::prelude::*;

use super::models::{Callback, NewCallback};
use super::schema;

pub fn create_callback(connection: &PgConnection, code: &str, state: &str) -> Callback {
    use schema::callbacks::dsl::callbacks;

    let callback = NewCallback {
        code: code.to_owned(),
        state: state.to_owned(),
    };

    diesel::insert_into(callbacks)
        .values(&callback)
        .get_result(connection)
        .expect("Error saving callback")
}

pub fn find_code_by_state(
    connection: &PgConnection,
    auth_state: &str,
) -> Result<Option<String>, diesel::result::Error> {
    use schema::callbacks::dsl::{callbacks, code, state};

    callbacks
        .select(code)
        .filter(state.eq(auth_state.to_owned()))
        .get_result(connection)
        .optional()
}
