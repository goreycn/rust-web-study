use actix_web::{HttpResponse, web};
use actix_web::body::MessageBody;
use actix_web::web::Data;

use crate::custom_error::*;
use crate::bean::*;


pub(crate) async fn error_forbidden() -> Result<HttpResponse, CustomError> {
    // let username = user.username.unwrap_or("".to_string());
    // if username.chars().count() > 0 {
    //     Ok(HttpResponse::Ok().json(Rst::ok(Some("ok".to_string()))))
    // }
    // else {
        Err(CustomError::NotFound)
    // }
}