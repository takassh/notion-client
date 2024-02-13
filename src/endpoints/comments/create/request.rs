use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::objects::{parent::Parent, rich_text::RichText};

#[skip_serializing_none]
#[derive(Serialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct CreateCommentRequest {
    pub parent: Option<Parent>,
    pub discussion_id: Option<Vec<RichText>>,
    pub rich_text: RichText,
}
