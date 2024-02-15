use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::objects::block::Block;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct UpdateABlockRequest {
    #[serde(flatten)]
    pub block: Option<Block>,
    pub archived: Option<bool>,
}
