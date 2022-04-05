use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use crate::bean;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    username: String,
    password: String,
}

pub(crate) async fn login(user: web::Json<User>) -> HttpResponse {
    print!("user: {:?}", &user);
    let rst = bean::Result {
        err_code: 0,
        err_msg: String::from("OK"),
    };

    HttpResponse::Ok().json(rst)
}