use serde::{Deserialize, Serialize};

use crate::objects::page::Page;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct QueryDatabaseResponse {
    pub object: String,
    pub results: Vec<Page>,
    pub next_cursor: Option<String>,
    pub has_more: bool,
}
