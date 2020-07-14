use actix_session::Session;
use actix_web::{web::Query, Error, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;
use spotify_api::{
    browse::BrowseClient,
    playlist::{Playlist, PlaylistClient},
    track::Track,
};

#[derive(Debug, Deserialize)]
pub struct GetPlaylistRequest {
    playlist_id: String,
}

pub async fn get_playlists(session: Session) -> Result<impl Responder, Error> {
    if session.get::<String>("user_id")?.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let user_id = session.get::<String>("user_id")?.unwrap();
    let access_token = session.get::<String>("access_token")?.unwrap();
    let refresh_token = session.get::<String>("refresh_token")?.unwrap();

    let mut client = PlaylistClient::new(&access_token, &refresh_token);

    let mut user_playlists = Vec::new();
    let mut followed_playlists = Vec::new();

    let request = spotify_api::playlist::GetPlaylistsRequest {
        ..Default::default()
    };

    let playlists = client
        .get_playlists(request)
        .await
        .unwrap()
        .get_all_items(&access_token, &refresh_token)
        .await
        .unwrap();

    for playlist in playlists {
        let image_url = match playlist.images.first() {
            Some(image) => &image.url,
            None => "",
        };

        let playlist_json = json!({
            "id": playlist.id,
            "image_url": image_url,
            "name": playlist.name,
        });

        if playlist.owner.id == user_id {
            user_playlists.push(playlist_json);
        } else {
            followed_playlists.push(playlist_json);
        }
    }

    let json = json!({
        "user_playlists": user_playlists,
        "followed_playlists": followed_playlists,
    });

    Ok(HttpResponse::Ok().json(json))
}

pub async fn get_featured_playlists(session: Session) -> Result<impl Responder, Error> {
    if session.get::<String>("user_id")?.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let access_token = session.get::<String>("access_token")?.unwrap();
    let refresh_token = session.get::<String>("refresh_token")?.unwrap();

    let mut client = BrowseClient::new(&access_token, &refresh_token);

    let request = spotify_api::browse::GetFeaturedPlaylistRequest {
        limit: Some(10),
        ..Default::default()
    };

    let playlists = client
        .get_featured_playlists(request)
        .await
        .unwrap()
        .playlists
        .get_items();

    let mut jsons = Vec::new();
    for playlist in playlists {
        let image_url = match playlist.images.first() {
            Some(image) => &image.url,
            None => "",
        };

        let json = json!({
            "id": playlist.id,
            "image_url": image_url,
            "name": playlist.name,
        });

        jsons.push(json);
    }

    let response = json!({
        "playlists": jsons,
    });

    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_playlist(
    Query(request): Query<GetPlaylistRequest>,
    session: Session,
) -> Result<impl Responder, Error> {
    if session.get::<String>("user_id")?.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let access_token = session.get::<String>("access_token")?.unwrap();
    let refresh_token = session.get::<String>("refresh_token")?.unwrap();

    let mut client = PlaylistClient::new(&access_token, &refresh_token);

    let request = spotify_api::playlist::GetPlaylistRequest {
        playlist_id: request.playlist_id,
        ..Default::default()
    };

    let Playlist {
        description,
        followers,
        id,
        images,
        name,
        owner,
        uri,
        ..
    } = client.get_playlist(request).await.unwrap();
    let response = json!({
        "playlist": {
            "description": description,
            "followers": {
                "total": followers.total,
            },
            "id": id,
            "image": images.first(),
            "name": name,
            "owner": {
                "name": owner.display_name,
                "id": owner.id,
            },
            "uri": uri,
        },
    });

    Ok(HttpResponse::Ok().json(response))
}

#[derive(Debug, Deserialize)]
pub struct GetTracksRequest {
    playlist_id: String,
}

pub async fn get_tracks(
    Query(request): Query<GetTracksRequest>,
    session: Session,
) -> Result<impl Responder, Error> {
    if session.get::<String>("user_id")?.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let access_token = session.get::<String>("access_token")?.unwrap();
    let refresh_token = session.get::<String>("refresh_token")?.unwrap();

    let mut client = PlaylistClient::new(&access_token, &refresh_token);

    let get_request = spotify_api::playlist::GetPlaylistTracksRequest {
        playlist_id: request.playlist_id,
        ..Default::default()
    };

    let tracks = client.get_tracks(get_request).await.unwrap().get_items();

    let mut track_jsons = Vec::new();
    for track in tracks {
        let Track {
            album,
            artists,
            id,
            name,
            uri,
            ..
        } = track.track;
        let album = album.unwrap();
        let artist = artists.first().unwrap();

        let artist_json = json!({
            "id": artist.id,
            "name": artist.name,
            "uri": artist.uri,
        });

        let track = json!({
            "album": {
                "id": album.id,
                "image": album.images.first(),
                "name": album.name,
            },
            "artist": artist_json,
            "id": id,
            "name": name,
            "uri": uri,
        });

        track_jsons.push(track);
    }

    let response = json!({
        "tracks": track_jsons,
    });

    Ok(HttpResponse::Ok().json(response))
}
