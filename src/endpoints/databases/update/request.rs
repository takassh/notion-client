use std::collections::BTreeMap;

use derive_builder::Builder;
use serde::Serialize;

use crate::objects::{database::DatabaseProperty, rich_text::RichText};

#[derive(Serialize, Debug, Eq, PartialEq, Clone, Default, Builder)]
#[builder(setter(strip_option))]
#[builder(default)]
pub struct UpdateADatabaseRequest {
    pub title: Option<Vec<RichText>>,
    pub description: Option<Vec<RichText>>,
    pub properties: BTreeMap<String, Option<DatabaseProperty>>,
}
