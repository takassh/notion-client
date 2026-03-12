use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{
    database::DatabaseProperty, emoji::Emoji, file::File, parent::Parent, rich_text::RichText,
    user::User,
};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct DataSource {
    pub object: Option<String>,
    pub id: String,
    pub created_time: DateTime<Utc>,
    pub created_by: Option<User>,
    pub last_edited_time: DateTime<Utc>,
    pub last_edited_by: Option<User>,
    pub title: Vec<RichText>,
    pub description: Vec<RichText>,
    pub icon: Option<Icon>,
    pub cover: Option<File>,
    pub properties: HashMap<String, DatabaseProperty>,
    pub parent: Parent,
    pub url: Option<String>,
    pub public_url: Option<String>,
    pub is_inline: Option<bool>,
    pub in_trash: bool,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
#[serde(rename_all = "snake_case", untagged)]
pub enum Icon {
    #[default]
    None,
    File(File),
    Emoji(Emoji),
}
