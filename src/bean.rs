use actix_web::{Error, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::my_error::MyError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Rst {
    pub err_code: i8,
    pub err_msg: String,
}


impl Rst {
    pub fn ok(msg: Option<String>) -> Rst {
        Rst {
            err_code: 0,
            err_msg: msg.unwrap_or(String::from("succeed")),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub password: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct SomeUser {
    pub username: Option<String>,
    pub password: Option<String>,
}

impl SomeUser {

    pub fn password(&self) -> Result<&String, MyError> {
        match self.password.as_ref() {
            None => Err(MyError::NotValidParam),
            Some(str) => Ok(str)
        }
    }

    pub fn is_valid(&self) -> Result<(), MyError> {
        if self.username.is_none() {
            Err(MyError::Other("username".to_string()))
        } else {
            Ok(())
        }
    }
}