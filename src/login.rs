use actix_session::Session;
use actix_web::{web::Query, HttpResponse, Result};
use serde::Deserialize;
use spotify_api::{
    authentication::{request_tokens, Scope, SpotifyOAuth},
    user::UserClient,
};

use crate::database;

#[derive(Deserialize)]
pub struct Identity {
    code: String,
}

pub async fn get_login_url() -> String {
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
    oauth.generate_auth_url().unwrap()
}

pub async fn login(Query(code): Query<Identity>, session: Session) -> Result<HttpResponse> {
    let tokens = request_tokens(&code.code).unwrap();

    let access_token = &tokens.access_token;
    let refresh_token = &tokens.refresh_token.clone().unwrap();

    let user_id = UserClient::new(access_token, refresh_token)
        .get_current_user()
        .id;

    let connection = database::establish_connection();

    let user = database::user::find_user(&connection, &user_id).unwrap();
    dbg!(&user);
    if user.is_none() {
        let _ = database::user::create_user(&connection, &user_id);
        let token = database::token::create_token(&connection, access_token, refresh_token);
        let _ = database::credential::create_credential(&connection, &user_id, token.id);
    }

    session.set("user_id", &user_id)?;
    session.renew();

    Ok(HttpResponse::Ok().finish())
}
