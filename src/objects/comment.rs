use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{parent::Parent, rich_text::RichText, user::User};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Comment {
    pub object: String,
    pub id: String,
    pub parent: Parent,
    pub discussion_id: String,
    pub created_time: DateTime<Utc>,
    pub last_edited_time: DateTime<Utc>,
    pub created_by: User,
    pub rich_text: Vec<RichText>,
}
