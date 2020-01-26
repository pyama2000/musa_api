use actix_session::Session;
use actix_web::{web::Query, HttpResponse, Result};
use base64;
use rand::{rngs::OsRng, RngCore};
use serde::{Deserialize, Serialize};
use spotify_api::authentication::{Scope, SpotifyOAuth};

#[derive(Deserialize)]
pub struct Identity {
    code: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SessionResponse {
    session_id: Option<String>,
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

pub async fn login(code: Query<Identity>, session: Session) -> Result<HttpResponse> {
    let mut session_id = vec![0u8; 32];
    OsRng.fill_bytes(&mut session_id);

    let session_id = base64::encode(&session_id);

    session.set("session_id", &session_id)?;
    session.renew();

    Ok(HttpResponse::Ok().finish())
}
