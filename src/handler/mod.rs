use serde::Deserialize;

pub mod auth;
pub mod playlist;

#[derive(Deserialize)]
pub struct User {
    user_id: String,
}
