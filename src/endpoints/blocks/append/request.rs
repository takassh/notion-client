use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::objects::block::Block;

#[skip_serializing_none]
#[derive(Serialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct AppendBlockChildrenRequest {
    pub children: Option<Block>,
    pub after: Option<String>,
}
