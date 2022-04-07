use std::fmt::{Debug, Display, Formatter};

use actix_web::{HttpResponse, ResponseError};
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;

use crate::bean::Rst;

pub enum MyError {
    NotValidParam,
    NoNonce,
    Other(String),
}

impl MyError {
    pub fn name(&self) -> String {
        match self {
            MyError::NotValidParam => "NotValidParam".to_string(),
            MyError::NoNonce => "NoNonce".to_string(),
            MyError::Other(_) => "Other".to_string(),
        }
    }
}

impl Debug for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::Other(msg) => write!(f, "{}", msg),
            _ => write!(f, "{}", self.name()),
        }
    }
}

impl ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        StatusCode::NOT_ACCEPTABLE
    }
    fn error_response(&self) -> HttpResponse<BoxBody> {
        match self {
            MyError::Other(msg) => {
                let out = format!("{}", msg);
                HttpResponse::build(self.status_code()).json(Rst { err_code: 0, err_msg: out })
            }
            _ => HttpResponse::build(self.status_code()).json(Rst { err_code: 0, err_msg: self.name()})
        }
    }
}