#[macro_use]
extern crate diesel;

use std::env;

use actix_web::{
    middleware::{DefaultHeaders, Logger},
    web::{get, post, resource},
    App, HttpServer,
};
use dotenv::dotenv;

use crate::handler::*;

mod database;
mod handler;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    better_panic::install();

    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(DefaultHeaders::new().header("Access-Control-Allow-Origin", "*"))
            .wrap(Logger::default())
            .service(resource("/auth").route(get().to(login::get_login_url)))
            .service(resource("/callback").route(post().to(login::callback)))
            .service(resource("/login").route(post().to(login::login)))
            .service(resource("/playlists").route(get().to(playlist::get_playlists)))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
