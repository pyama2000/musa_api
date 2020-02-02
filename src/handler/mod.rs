use serde::Deserialize;

pub mod login;
pub mod playlist;

#[derive(Deserialize)]
pub struct User {
    user_id: String,
}
