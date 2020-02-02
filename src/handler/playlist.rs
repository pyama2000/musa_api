use actix_web::{web::Query, HttpResponse, Result};
use serde_json::json;
use spotify_api::playlist::PlaylistClient;

use crate::{database, handler::User};

pub async fn get_playlists(Query(user): Query<User>) -> Result<HttpResponse> {
    let user_id = user.user_id;

    let connection = database::establish_connection();
    let token_id = database::credential::find_token_id_by_user_id(&connection, &user_id)
        .unwrap()
        .unwrap();
    let token = database::token::find_token(&connection, token_id)
        .unwrap()
        .unwrap();

    let mut client = PlaylistClient::new(&token.access_token, &token.refresh_token);
    let playlists = client
        .get_current_user_playlists(None, None)
        .get_all_items();

    let json = json!({
        "playlists": playlists,
    });

    Ok(HttpResponse::Ok().json(json))
}
