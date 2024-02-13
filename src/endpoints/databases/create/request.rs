use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::objects::{
    database::DatabaseProperty, database::Icon, file::File, parent::Parent, rich_text::RichText,
};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct CreateADatabaseRequest {
    pub parent: Parent,
    pub icon: Option<Icon>,
    pub cover: Option<File>,
    pub title: Option<Vec<RichText>>,
    pub properties: BTreeMap<String, DatabaseProperty>,
}
