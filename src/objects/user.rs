use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Default)]

pub struct User {
    pub object: String,
    pub id: String,
    #[serde(flatten)]
    pub user_type: Option<UserType>,
    pub name: Option<String>,
    pub avator_url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum UserType {
    Person { person: Person },
    Bot { bot: Bot },
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Person {
    pub email: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Bot {
    pub owner: OwnerType,
    pub workspace_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OwnerType {
    Workspace { workspace: bool },
    User { workspace: bool },
}
