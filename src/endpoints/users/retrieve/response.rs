use serde::{Deserialize, Serialize};

use crate::objects::block::Block;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct RetrieveBlockChilerenResponse {
    pub object: String,
    pub results: Vec<Block>,
    pub next_cursor: Option<String>,
    pub has_more: bool,
}
