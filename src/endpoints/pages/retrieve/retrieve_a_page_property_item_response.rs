use serde::{Deserialize, Serialize};

use crate::objects::property::Property;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "object", rename_all = "snake_case")]
pub enum RetrieveAPagePropertyItemResponse {
    PropertyItem {
        #[serde(flatten)]
        item: Property,
    },
    List {
        results: Vec<RetrieveAPagePropertyItemResponse>,
        next_cursor: Option<String>,
        has_more: bool,
        next_url: Option<String>,
    },
}
