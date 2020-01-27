#[macro_use]
extern crate diesel;

use std::env;

use actix_redis::RedisSession;
use actix_web::{
    middleware::{DefaultHeaders, Logger},
    web::{get, post, resource},
    App, HttpServer,
};
use dotenv::dotenv;

mod database;
mod login;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    better_panic::install();

    env::set_var("RUST_LOG", "actix_web=info,actix_redis=info");
    env_logger::init();

    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");

    HttpServer::new(move || {
        App::new()
            .wrap(RedisSession::new(&redis_url, &[0; 32]))
            .wrap(DefaultHeaders::new().header("Access-Control-Allow-Origin", "*"))
            .wrap(Logger::default())
            .service(resource("/auth").route(get().to(login::get_login_url)))
            .service(resource("/login").route(post().to(login::login)))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
