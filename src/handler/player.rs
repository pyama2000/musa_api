use actix_session::Session;
use actix_web::{
    web::{Json, Query},
    Error, HttpResponse, Responder,
};
use serde::Deserialize;
use serde_json::json;

use spotify_api::{player::PlayerClient, track::Track};

#[derive(Debug, Deserialize)]
pub struct StartRequest {
    context_uri: Option<String>,
    uris: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct ToggleShuffleRequest {
    state: bool,
}

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

pub async fn start(request: Json<StartRequest>, session: Session) -> Result<impl Responder, Error> {
    if session.get::<String>("user_id")?.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let access_token = session.get::<String>("access_token")?.unwrap();
    let refresh_token = session.get::<String>("refresh_token")?.unwrap();

    let mut client = PlayerClient::new(&access_token, &refresh_token);

    let start_request = if request.context_uri.is_some() {
        spotify_api::player::StartRequest {
            context_uri: request.context_uri.clone(),
            ..Default::default()
        }
    } else {
        spotify_api::player::StartRequest {
            uris: request.uris.clone(),
            ..Default::default()
        }
    };

    client.start(Some(start_request)).await.unwrap();

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

pub async fn toggle_shuffle(
    Query(request): Query<ToggleShuffleRequest>,
    session: Session,
) -> Result<impl Responder, Error> {
    if session.get::<String>("user_id")?.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let access_token = session.get::<String>("access_token")?.unwrap();
    let refresh_token = session.get::<String>("refresh_token")?.unwrap();

    let mut client = PlayerClient::new(&access_token, &refresh_token);

    let start_request = spotify_api::player::ToggleShuffleRequest {
        state: request.state,
        ..Default::default()
    };

    client.toggle_shuffle(start_request).await.unwrap();

    Ok(HttpResponse::NoContent().finish())
}
