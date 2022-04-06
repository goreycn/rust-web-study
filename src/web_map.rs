use actix_web::{HttpResponse, web};
use log::info;

use bean::Rst;

use crate::bean;
use crate::bean::User;


pub(crate) async fn login(user: web::Json<User>) -> HttpResponse {
    info!("user: {:?}", &user);
    HttpResponse::Ok().json(Rst::ok(None))
}