pub mod callback;
pub mod credential;
mod models;
mod schema;
pub mod token;
pub mod user;

use std::env;

use diesel::prelude::*;
use dotenv::dotenv;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://admin:admin@localhost:5432/musa".to_owned());

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[test]
fn test_connection() {
    establish_connection();
}
