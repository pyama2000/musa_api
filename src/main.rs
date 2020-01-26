use actix_redis::RedisSession;
use actix_web::{
    middleware::{DefaultHeaders, Logger},
    web::{get, post, resource},
    App, HttpServer,
};

mod login;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    better_panic::install();

    std::env::set_var("RUST_LOG", "actix_web=info,actix_redis=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(RedisSession::new("127.0.0.1:6379", &[0; 32]))
            .wrap(DefaultHeaders::new().header("Access-Control-Allow-Origin", "*"))
            .wrap(Logger::default())
            .service(resource("/auth").route(get().to(login::get_login_url)))
            .service(resource("/tokens").route(post().to(login::login)))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
