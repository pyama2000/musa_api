use actix_session::Session;
use actix_web::{web::Query, Error, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;
use spotify_api::search::SearchClient;

#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    query: String,
}

pub async fn search(
    Query(request): Query<SearchRequest>,
    session: Session,
) -> Result<impl Responder, Error> {
    if session.get::<String>("user_id")?.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let access_token = session.get::<String>("access_token")?.unwrap();
    let refresh_token = session.get::<String>("refresh_token")?.unwrap();

    let queries: Vec<String> = request
        .query
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    let mut client = SearchClient::new(&access_token, &refresh_token);
    let client = client.set_limit(5);

    for query in queries {
        client.set_keyword(&query);
    }

    let tracks = client.search_track().await.unwrap().get_items();
    let mut track_jsons = Vec::new();
    for track in tracks {
        let json = json!({
            "id": track.id,
            "name": track.name,
        });

        track_jsons.push(json);
    }

    let artists = client.search_artist().await.unwrap().get_items();
    let mut artist_jsons = Vec::new();
    for artist in artists {
        let json = json!({
            "id": artist.id,
            "name": artist.name,
        });

        artist_jsons.push(json);
    }

    let albums = client.search_album().await.unwrap().get_items();
    let mut album_jsons = Vec::new();
    for album in albums {
        let json = json!({
            "id": album.id,
            "name": album.name,
        });

        album_jsons.push(json);
    }

    let response = json!({
        "albums": album_jsons,
        "artists": artist_jsons,
        "tracks": track_jsons,
    });

    Ok(HttpResponse::Ok().json(response))
}
