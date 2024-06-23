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
    pub after: Option<String>,
}
