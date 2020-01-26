use actix_redis::RedisSession;
use actix_web::{middleware, App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    better_panic::install();

    std::env::set_var("RUST_LOG", "actix_web=info,actix_redis=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(RedisSession::new("127.0.0.1:6379", &[0; 32]))
            .wrap(middleware::Logger::default())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
