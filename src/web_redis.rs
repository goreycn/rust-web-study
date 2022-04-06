use actix::Addr;
use actix_redis::*;
use actix_web::{HttpResponse, Responder, web, web::Data};
use log::info;

pub async fn add(redis: web::Data<Addr<RedisActor>>) -> HttpResponse {
    info!("{:p}", &redis);
    HttpResponse::Ok().body("ok")
}

pub async fn get(redis: web::Data<Addr<RedisActor>>) -> HttpResponse {
    HttpResponse::Ok().body("ok")
}