use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct Error {
    pub object: String,
    pub status: u32,
    pub code: String,
    pub message: String,
    pub request_id: Option<String>,
}
