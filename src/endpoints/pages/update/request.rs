use std::collections::BTreeMap;

use derive_builder::Builder;
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::objects::{
    file::File,
    page::{Icon, PageProperty},
};

#[skip_serializing_none]
#[derive(Serialize, Debug, Eq, PartialEq, Clone, Default, Builder)]
#[builder(setter(strip_option))]
#[builder(default)]
pub struct UpdatePagePropertiesRequest {
    pub properties: BTreeMap<String, Option<PageProperty>>,
    pub archived: Option<bool>,
    pub icon: Option<Icon>,
    pub cover: Option<File>,
}
