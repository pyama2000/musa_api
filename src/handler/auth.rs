use actix_session::Session;
use actix_web::{web::Json, Error, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;
use spotify_api::authentication::{Scope, SpotifyOAuth};

#[derive(Debug, Deserialize)]
pub struct GetTokenRequest {
    code: String,
}

pub async fn get_login_url() -> Result<impl Responder, Error> {
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

pub async fn login(Json(request): Json<GetTokenRequest>, session: Session) -> Result<impl Responder, Error> {
    let tokens = spotify_api::authentication::request_tokens(&request.code).unwrap();

    session.set("access_token", &tokens.access_token)?;
    session.set("access_token", &tokens.refresh_token)?;

    Ok(HttpResponse::Ok().finish())
}
