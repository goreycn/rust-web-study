use std::fs::File;
use std::io::Read;
use ::log::info;
use actix_redis::RedisActor;
use actix_web::{App, HttpServer, web};
<<<<<<< HEAD
use log::warn;
=======
use actix_web::dev::Service;
>>>>>>> c0ba7a656ad253a1cea7832b4423d872f10a18b3
use mobc_redis::redis;
use mobc_redis::mobc::Pool;
use redis::AsyncCommands;

use web_index::index;
use web_map::*;
use web_redis::*;
use web_redis_mobc::*;
use web_error_demo::*;
use crate::bean::Config;

mod web_index;
mod web_map;
mod bean;
mod log_tool;
mod web_redis;
mod web_redis_mobc;
mod custom_error;
mod web_error_demo;
mod my_error;
<<<<<<< HEAD
mod load_config;
=======
mod aop;
>>>>>>> c0ba7a656ad253a1cea7832b4423d872f10a18b3


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    log_tool::init().expect("log init failed");

    let config = load_config::load_config();

    println!("{}", config.address);
    println!("{}", config.address);

    info!("starting HTTP server at http://localhost:18080");

    HttpServer::new(|| {
        // redis 1
        let redis_addr = RedisActor::start("127.0.0.1:6379");
        // redis 2
        let client = redis::Client::open("redis://127.0.0.1/").unwrap();


        App::new()
            .wrap_fn(|req, srv|{
                get_request_body(&mut req);

                info!("uri: {}", req.path());
                let fut = srv.call(req);
                Box::pin(async move {
                    let res= fut.await?;
                    info!("response body");
                    Ok(res)
                })
            })
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
            .route("/custom", web::get().to(error_with_message))
            .route("/custom2", web::get().to(error_with_message2))
    })
        .bind(("0.0.0.0", 18080))?
        .run()
        .await
}