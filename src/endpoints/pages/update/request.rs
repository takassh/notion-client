use std::collections::BTreeMap;

use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::objects::{
    file::File,
    page::{Icon, PageProperty},
};

#[skip_serializing_none]
#[derive(Serialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct UpdatePagePropertiesRequest {
    pub properties: BTreeMap<String, PageProperty>,
    pub archived: Option<bool>,
    pub icon: Option<Icon>,
    pub color: Option<File>,
}
