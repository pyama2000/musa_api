#[macro_use]
extern crate diesel;

use std::env;

use actix_redis::RedisSession;
use actix_web::{
    http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
    middleware::{DefaultHeaders, Logger},
    web::{get, post, put, resource, scope},
    App, HttpServer,
};
use dotenv::dotenv;

mod database;
mod handler;
use crate::handler::*;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info,actix_redis=info");

    dotenv().ok();
    better_panic::install();
    env_logger::init();

    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "0.0.0.0:6379".to_string());

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(RedisSession::new(&redis_url, &[0; 32]))
            .wrap(actix_cors::Cors::default())
            .wrap(DefaultHeaders::new().header(ACCESS_CONTROL_ALLOW_CREDENTIALS, "true"))
            .service(
                resource("/auth")
                    .route(get().to(auth::get_login_url))
                    .route(post().to(auth::login)),
            )
            .service(
                scope("/player")
                    .route("/current", get().to(player::get_current_playing))
                    .route("/pause", put().to(player::pause)),
            )
            .service(resource("/playlists").route(get().to(playlist::get_playlists)))
            .service(resource("/playlist").route(get().to(playlist::get_playlist)))
            .service(resource("/tracks").route(get().to(playlist::get_tracks)))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
