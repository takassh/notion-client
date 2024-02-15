use serde::{Deserialize, Serialize};

use crate::objects::comment::Comment;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct RetrieveCommentsResponse {
    pub object: String,
    pub results: Vec<Comment>,
    pub next_cursor: Option<String>,
    pub has_more: bool,
}
