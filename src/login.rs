use actix_session::Session;
use actix_web::{web::Query, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use spotify_api::{
    authentication::{request_tokens, Scope, SpotifyOAuth},
    user::UserClient,
};

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

    let user_id = UserClient::new(&tokens.access_token, &tokens.refresh_token.unwrap())
        .get_current_user()
        .id;

    session.set("user_id", &user_id)?;
    session.renew();

    Ok(HttpResponse::Ok().finish())
}
