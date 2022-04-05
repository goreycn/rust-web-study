use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Result {
    pub err_code: i8,
    pub err_msg: String,
}