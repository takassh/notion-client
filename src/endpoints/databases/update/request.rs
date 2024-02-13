use std::collections::BTreeMap;

use serde::Serialize;

use crate::objects::{database::DatabaseProperty, rich_text::RichText};

#[derive(Serialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct UpdateADatabaseRequest {
    pub title: Option<Vec<RichText>>,
    pub description: Option<Vec<RichText>>,
    pub properties: BTreeMap<String, Option<DatabaseProperty>>,
}
