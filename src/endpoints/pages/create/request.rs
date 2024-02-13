use std::collections::HashMap;

use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::objects::{page::PageProperty, parent::Parent};

#[skip_serializing_none]
#[derive(Serialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct CreateAPageRequest {
    pub parent: Parent,
    pub properties: HashMap<String, PageProperty>,
    pub children: Option<Vec<String>>,
    pub icon: Option<String>,
    pub color: Option<String>,
}
