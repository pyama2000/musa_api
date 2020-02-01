use actix_web::{web::Query, HttpResponse, Result};
use serde::Deserialize;
use serde_json::json;
use spotify_api::{
    authentication::{request_tokens, Scope, SpotifyOAuth},
    user::UserClient,
};

use crate::database;

#[derive(Deserialize)]
pub struct Callback {
    code: String,
    state: String,
}

#[derive(Deserialize)]
pub struct State {
    state: String,
}

pub async fn get_login_url() -> Result<HttpResponse> {
    let scopes = vec![
        Scope::UserReadPrivate,
        Scope::UserReadBirthdate,
        Scope::UserReadEmail,
        Scope::Streaming,
        Scope::AppRemoteControl,
        Scope::UserTopRead,
        Scope::UserReadRecentlyPlayed,
        Scope::UserLibraryRead,
        Scope::UserLibraryModify,
        Scope::PlaylistReadCollaborative,
        Scope::PlaylistReadPrivate,
        Scope::PlaylistModifyPublic,
        Scope::PlaylistModifyPrivate,
        Scope::UserReadCurrentlyPlaying,
        Scope::UserReadPlaybackState,
        Scope::UserModifyPlaybackState,
        Scope::UserFollowRead,
        Scope::UserFollowModify,
    ];

    let mut oauth = SpotifyOAuth::new();
    oauth.set_scopes(&scopes);

    let url = oauth.generate_auth_url().unwrap();
    let state = oauth.get_state();

    let json = json!({
        "url": url,
        "state": state,
    });

    Ok(HttpResponse::Ok().json(json))
}

pub async fn callback(Query(callback): Query<Callback>) -> Result<HttpResponse> {
    let Callback { code, state } = callback;

    let connection = database::establish_connection();
    let _ = database::callback::create_callback(&connection, &code, &state);

    Ok(HttpResponse::Ok().finish())
}

pub async fn login(Query(state): Query<State>) -> Result<HttpResponse> {
    let connection = database::establish_connection();
    let code = database::callback::find_code_by_state(&connection, &state.state)
        .unwrap()
        .unwrap();

    let tokens = request_tokens(&code).unwrap();

    let access_token = &tokens.access_token;
    let refresh_token = &tokens.refresh_token.clone().unwrap();

    let user_id = UserClient::new(access_token, refresh_token)
        .get_current_user()
        .id;

    let user = database::user::find_user(&connection, &user_id).unwrap();
    if user.is_none() {
        let _ = database::user::create_user(&connection, &user_id);
        let token = database::token::create_token(&connection, access_token, refresh_token);
        let _ = database::credential::create_credential(&connection, &user_id, token.id);

        return Ok(HttpResponse::Created().json(user_id));
    }

    Ok(HttpResponse::Ok().json(user_id))
}
