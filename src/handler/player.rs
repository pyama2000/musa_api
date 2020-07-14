use actix_session::Session;
use actix_web::{Error, HttpResponse, Responder};
use serde_json::json;

use spotify_api::{player::PlayerClient, track::Track};

pub async fn get_current_playing(session: Session) -> Result<impl Responder, Error> {
    if session.get::<String>("user_id")?.is_none() {
        return Ok(HttpResponse::NoContent().finish());
    }

    let access_token = session.get::<String>("access_token")?.unwrap();
    let refresh_token = session.get::<String>("refresh_token")?.unwrap();

    let mut client = PlayerClient::new(&access_token, &refresh_token);

    let request = spotify_api::player::GetCurrentlyRequest {
        ..Default::default()
    };

    if let Some(response) = client.get_currently_playing_track(request).await.unwrap() {
        match response.currently_playing_type {
            spotify_api::player::ObjectType::Track => {
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
            _ => return Ok(HttpResponse::NoContent().finish()),
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

    let request = spotify_api::player::PauseRequest {
        ..Default::default()
    };

    client.pause(request).await.unwrap();

    Ok(HttpResponse::NoContent().finish())
}

pub async fn resume(session: Session) -> Result<impl Responder, Error> {
    if session.get::<String>("user_id")?.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let access_token = session.get::<String>("access_token")?.unwrap();
    let refresh_token = session.get::<String>("refresh_token")?.unwrap();

    let mut client = PlayerClient::new(&access_token, &refresh_token);
    client.start(None).await.unwrap();

    Ok(HttpResponse::NoContent().finish())
}

pub async fn next(session: Session) -> Result<impl Responder, Error> {
    if session.get::<String>("user_id")?.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let access_token = session.get::<String>("access_token")?.unwrap();
    let refresh_token = session.get::<String>("refresh_token")?.unwrap();

    let mut client = PlayerClient::new(&access_token, &refresh_token);

    let skip_request = spotify_api::player::SkipRequest {
        ..Default::default()
    };

    client.skip_next(skip_request).await.unwrap();

    let get_currently_request = spotify_api::player::GetCurrentlyRequest {
        ..Default::default()
    };

    if let Some(response) = client
        .get_currently_playing_track(get_currently_request)
        .await
        .unwrap()
    {
        match response.currently_playing_type {
            spotify_api::player::ObjectType::Track => {
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
            _ => return Ok(HttpResponse::NoContent().finish()),
        }
    }

    Ok(HttpResponse::NoContent().finish())
}

pub async fn previous(session: Session) -> Result<impl Responder, Error> {
    if session.get::<String>("user_id")?.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let access_token = session.get::<String>("access_token")?.unwrap();
    let refresh_token = session.get::<String>("refresh_token")?.unwrap();

    let mut client = PlayerClient::new(&access_token, &refresh_token);

    let skip_request = spotify_api::player::SkipRequest {
        ..Default::default()
    };

    client.skip_previous(skip_request).await.unwrap();

    let get_currently_request = spotify_api::player::GetCurrentlyRequest {
        ..Default::default()
    };

    if let Some(response) = client
        .get_currently_playing_track(get_currently_request)
        .await
        .unwrap()
    {
        match response.currently_playing_type {
            spotify_api::player::ObjectType::Track => {
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
            _ => return Ok(HttpResponse::NoContent().finish()),
        }
    }

    Ok(HttpResponse::NoContent().finish())
}
