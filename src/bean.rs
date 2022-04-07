use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Rst {
    pub err_code: i8,
    pub err_msg: String,
}


impl Rst {
    pub fn ok(msg: Option<String>) -> Rst {
        Rst {
            err_code: 0,
            err_msg: msg.unwrap_or(String::from("succeed"))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    username: String,
    password: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct SomeUser {
    username: Option<String>,
    password: Option<String>,
}