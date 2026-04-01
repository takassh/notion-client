use std::collections::BTreeMap;

use derive_builder::Builder;
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::objects::{
    data_source::Icon, database::DatabaseProperty, file::File, parent::Parent, rich_text::RichText,
};

#[skip_serializing_none]
#[derive(Serialize, Debug, Eq, PartialEq, Clone, Default, Builder)]
#[builder(setter(strip_option))]
#[builder(default)]
pub struct CreateADataSourceRequest {
    pub parent: Parent,
    pub icon: Option<Icon>,
    pub cover: Option<File>,
    pub title: Option<Vec<RichText>>,
    pub properties: BTreeMap<String, DatabaseProperty>,
}
