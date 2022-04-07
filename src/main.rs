use ::log::info;
use actix_redis::RedisActor;
use actix_web::{App, HttpServer, web};
use mobc_redis::{redis, RedisConnectionManager};
use mobc_redis::mobc::Pool;
use redis::AsyncCommands;

use web_index::index;
use web_map::*;
use web_redis::*;
use web_redis_mobc::*;
use web_error_demo::*;

mod web_index;
mod web_map;
mod bean;
mod log_tool;
mod web_redis;
mod web_redis_mobc;
mod custom_error;
mod web_error_demo;


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    log_tool::init().expect("log init failed");

    info!("starting HTTP server at http://localhost:18080");

    HttpServer::new(|| {
        // redis 1
        let redis_addr = RedisActor::start("127.0.0.1:6379");
        // redis 2
        let client = redis::Client::open("redis://127.0.0.1/").unwrap();


        App::new()
            .app_data(web::Data::new(redis_addr))
            .app_data(web::Data::new(client))
            .route("/", web::get().to(index))
            .route("/login", web::post().to(login))
            .route("/login2", web::post().to(login_some_user))
            .route("/redis/add", web::get().to(add))
            .route("/redis/get", web::get().to(get))
            .route("/redis2/get", web::get().to(get2))
            .route("/redis2/add", web::get().to(add2))
            .route("/forbidden", web::get().to(error_forbidden))

    })
        .bind(("0.0.0.0", 18080))?
        .run()
        .await
}