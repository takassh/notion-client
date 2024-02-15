use std::collections::BTreeMap;

use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::objects::{
    block::Block, file::File, page::{Icon, PageProperty}, parent::Parent
};

#[skip_serializing_none]
#[derive(Serialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct CreateAPageRequest {
    pub parent: Parent,
    pub icon: Option<Icon>,
    pub cover: Option<File>,
    pub properties: BTreeMap<String, PageProperty>,
    pub children: Option<Vec<Block>>,
}
