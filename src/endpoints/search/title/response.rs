use serde::{Deserialize, Serialize};

use crate::objects::{database::Database, page::Page};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct SearchByTitleResponse {
    pub object: String,
    pub results: Vec<PageOrDatabase>,
    pub next_cursor: Option<String>,
    pub has_more: bool,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "object", rename_all = "snake_case")]
pub enum PageOrDatabase {
    Page(Page),
    Database(Database),
}
