use ::log::info;
use actix_redis::RedisActor;
use actix_web::{App, HttpServer, web};

use web_index::index;
use web_map::login;
use web_redis::*;

mod web_index;
mod web_map;
mod bean;
mod log_tool;
mod web_redis;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    log_tool::init().expect("log init failed");

    info!("starting HTTP server at http://localhost:18080");

    HttpServer::new(|| {
        let redis_addr = RedisActor::start("127.0.0.1:6379");

        App::new()
            .app_data(web::Data::new(redis_addr))
            .route("/", web::get().to(index))
            .route("/login", web::post().to(login))
            .route("/redis/add", web::get().to(add))
            .route("/redis/get", web::get().to(get))
    })
        .bind(("0.0.0.0", 18080))?
        .run()
        .await
}