use actix_web::{HttpMessage, HttpResponse, web};
use mobc_redis::mobc::Pool;
use mobc_redis::{redis, RedisConnectionManager};
use crate::redis::aio::Connection;

pub async fn add2( client: web::Data<redis::Client>) -> HttpResponse {
    let mut conn = client.get_async_connection().await.expect("error");
    let () = redis::cmd("incr").arg("mobc_incr").query_async(&mut conn).await.expect("error2");
    HttpResponse::Ok().body("ok")
}

pub async fn get2() -> HttpResponse {
    HttpResponse::Ok().body("ok")
}