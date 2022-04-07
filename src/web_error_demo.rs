use actix_web::{HttpResponse, web};
use actix_web::body::MessageBody;
use actix_web::web::Data;

use crate::bean::*;
use crate::custom_error::*;
use crate::my_error::MyError;

pub(crate) async fn error_forbidden() -> Result<HttpResponse, CustomError> {
    // let username = user.username.unwrap_or("".to_string());
    // if username.chars().count() > 0 {
    //     Ok(HttpResponse::Ok().json(Rst::ok(Some("ok".to_string()))))
    // }
    // else {
    Err(CustomError::NotFound)
    // }
}

pub(crate) async fn error_with_message() -> Result<HttpResponse, MyError> {
    // let username = user.username.unwrap_or("".to_string());
    // if username.chars().count() > 0 {
    //     Ok(HttpResponse::Ok().json(Rst::ok(Some("ok".to_string()))))
    // }
    // else {
    // Err(CustomError::Other("系统异常".to_string()))
    // }

    // Ok(HttpResponse::Ok().json(Rst::ok(Some("Nice".to_string()))))

    Err(MyError::Other("数据库连接失败".to_string()))
}
pub(crate) async fn error_with_message2() -> Result<HttpResponse, MyError> {
    // let username = user.username.unwrap_or("".to_string());
    // if username.chars().count() > 0 {
    //     Ok(HttpResponse::Ok().json(Rst::ok(Some("ok".to_string()))))
    // }
    // else {
    // Err(CustomError::Other("系统异常".to_string()))
    // }

    // Ok(HttpResponse::Ok().json(Rst::ok(Some("Nice".to_string()))))

    Err(MyError::NoNonce)
}