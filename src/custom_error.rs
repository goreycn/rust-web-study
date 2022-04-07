use std::fmt::{Display, Formatter};
use std::io::Read;
use std::os::macos::raw::stat;

use actix_web::{App, error::ResponseError, get, http::StatusCode, HttpResponse, HttpServer, web};
use actix_web::body::BoxBody;
use derive_more::{Display, Error};
// use thiserror::Error;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct ErrorResponse {
    code: u16,
    message: String,
    error: String,
}


#[derive(Error, Debug)]
pub enum CustomError {
    // #[error("Requested file wat not found")]
    NotFound,
    // #[error("You are forbidden")]
    Forbidden,
    // #[error("Unknown Internal Error")]
    Unknown,
}

impl CustomError {
    pub fn name(&self) -> String {
        match self {
            CustomError::NotFound => "NotFound".to_string(),
            CustomError::Forbidden => "Forbidden".to_string(),
            CustomError::Unknown => "Unknown".to_string(),
        }
    }
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "reason : - {}", self.name())
    }
}

impl ResponseError for CustomError {
    fn status_code(&self) -> StatusCode {
        match self {
            CustomError::NotFound => StatusCode::NOT_FOUND,
            CustomError::Forbidden => StatusCode::FORBIDDEN,
            CustomError::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let status_code = self.status_code();
        let error_response = ErrorResponse {
            code: status_code.as_u16(),
            message: self.to_string(),
            error: self.name(),
        };
        HttpResponse::build(status_code).json(error_response)
    }
}

