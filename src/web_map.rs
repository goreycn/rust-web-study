use actix_web::{HttpResponse, web};
use log::info;

use bean::Rst;

use crate::bean;
use crate::bean::*;

pub(crate) async fn login(user: web::Json<User>) -> HttpResponse {
    info!("user: {:?}", &user);
    HttpResponse::Ok().json(Rst::ok(None))
}

pub(crate) async fn login_some_user(user: web::Json<SomeUser>) -> HttpResponse {
    info!("some user: {:?}", &user);
    HttpResponse::Ok().json(Rst::ok(None))
}
