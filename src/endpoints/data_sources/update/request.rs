use std::collections::BTreeMap;

use derive_builder::Builder;
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::objects::{database::DatabaseProperty, rich_text::RichText};

#[skip_serializing_none]
#[derive(Serialize, Debug, Eq, PartialEq, Clone, Default, Builder)]
#[builder(setter(strip_option))]
#[builder(default)]
pub struct UpdateADataSourceRequest {
    pub title: Option<Vec<RichText>>,
    pub description: Option<Vec<RichText>>,
    pub properties: BTreeMap<String, Option<DatabaseProperty>>,
}
