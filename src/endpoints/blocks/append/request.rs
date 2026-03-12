use derive_builder::Builder;
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::objects::block::Block;

#[skip_serializing_none]
#[derive(Serialize, Debug, Eq, PartialEq, Clone, Default, Builder)]
#[builder(setter(strip_option))]
#[builder(default)]
pub struct AppendBlockChildrenRequest {
    pub children: Vec<Block>,
    pub position: Option<AppendPosition>,
}

#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AppendPosition {
    End,
    Start,
    AfterBlock { after_block: AfterBlockId },
}

#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
pub struct AfterBlockId {
    pub id: String,
}
