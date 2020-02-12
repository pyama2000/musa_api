use actix_web::{web::Query, HttpResponse, Result};
use serde::Deserialize;
use serde_json::json;
use spotify_api::playlist::PlaylistClient;

use crate::{database, handler::User};

#[derive(Deserialize)]
pub struct Playlist {
    playlist_id: String,
    #[serde(flatten)]
    user: User,
}

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

    let mut user_playlists = Vec::new();
    let mut followed_playlists = Vec::new();

    let playlists = client
        .get_current_user_playlists(None, None)
        .get_all_items();

    for playlist in playlists {
        let image_url = match playlist.images.last() {
            Some(image) => &image.url,
            None => "",
        };

        let playlist_json = json!({
            "id": playlist.id,
            "image_url": image_url,
            "name": playlist.name,
        });

        if playlist.owner.id == user_id {
            user_playlists.push(playlist_json);
        } else {
            followed_playlists.push(playlist_json);
        }
    }

    let json = json!({
        "user_playlists": user_playlists,
        "followed_playlists": followed_playlists,
    });

    Ok(HttpResponse::Ok().json(json))
}

pub async fn get_playlist(Query(struct_playlist): Query<Playlist>) -> Result<HttpResponse> {
    let user_id = struct_playlist.user.user_id;

    let connection = database::establish_connection();
    let token_id = database::credential::find_token_id_by_user_id(&connection, &user_id)
        .unwrap()
        .unwrap();
    let token = database::token::find_token(&connection, token_id)
        .unwrap()
        .unwrap();


    let mut client = PlaylistClient::new(&token.access_token, &token.refresh_token);
    let playlist = client.get_playlist(&struct_playlist.playlist_id, None);
    let json = json!({
        "playlist": playlist,
    });

    Ok(HttpResponse::Ok().json(json))
}
