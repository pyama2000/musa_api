use actix_session::Session;
use actix_web::{web::Json, Error, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;
use spotify_api::{
    authentication::{RequestTokenResponse, Scope, SpotifyOAuth},
    user::UserClient,
};

#[derive(Debug, Deserialize)]
pub struct GetTokenRequest {
    code: String,
}

pub async fn get_login_url(session: Session) -> Result<impl Responder, Error> {
    let scopes = vec![
        Scope::UserReadPrivate,
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

    let json = json!({
        "url": url,
        "state": oauth.state,
    });

    session.renew();

    Ok(HttpResponse::Ok().json(json))
}

pub async fn login(
    Json(request): Json<GetTokenRequest>,
    session: Session,
) -> Result<impl Responder, Error> {
    if session.get::<String>("user_id")?.is_some() {
        return Ok(HttpResponse::NoContent().finish());
    }

    let RequestTokenResponse {
        access_token,
        refresh_token,
    } = spotify_api::authentication::request_tokens(&request.code).await?;

    let user_id = UserClient::new(&access_token, &refresh_token)
        .get_current_user()
        .await
        .unwrap()
        .id;

    session.set("user_id", &user_id)?;
    session.set("access_token", &access_token)?;
    session.set("refresh_token", &refresh_token)?;

    Ok(HttpResponse::Ok().finish())
}
