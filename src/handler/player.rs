use actix_session::Session;
use actix_web::{Error, HttpResponse, Responder};
use serde_json::json;

use spotify_api::{object::Track, player::PlayerClient};

pub async fn get_current_playing(session: Session) -> Result<impl Responder, Error> {
    if session.get::<String>("user_id")?.is_none() {
        return Ok(HttpResponse::NoContent().finish());
    }

    let access_token = session.get::<String>("access_token")?.unwrap();
    let refresh_token = session.get::<String>("refresh_token")?.unwrap();

    let mut client = PlayerClient::new(&access_token, &refresh_token);
    if let Some(response) = client.get_currently_playing_track(None) {
        if response.currently_playing_type.eq("track") {
            let track: Track = serde_json::from_value(response.item.unwrap()).unwrap();

            let response = json!({
                "is_playing": response.is_playing,
                "track": {
                    "id": track.id,
                    "image": track.album.unwrap().images.first().unwrap().url,
                    "name": track.name,
                }
            });

            return Ok(HttpResponse::Ok().json(response));
        }
    }

    Ok(HttpResponse::NoContent().finish())
}

pub async fn pause(session: Session) -> Result<impl Responder, Error> {
    if session.get::<String>("user_id")?.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let access_token = session.get::<String>("access_token")?.unwrap();
    let refresh_token = session.get::<String>("refresh_token")?.unwrap();

    let mut client = PlayerClient::new(&access_token, &refresh_token);
    client.pause(None);

    Ok(HttpResponse::NoContent().finish())
}
