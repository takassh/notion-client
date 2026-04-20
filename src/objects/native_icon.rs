use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(tag = "type", rename = "icon", rename_all = "snake_case")]
pub enum NativeIcon {
    Icon { icon: Native },
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Native {
    pub name: String,
    pub color: Option<String>,
}
