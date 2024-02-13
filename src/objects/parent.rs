use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Default)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Parent {
    #[default]
    None,
    DatabaseId {
        database_id: String,
    },
    PageId {
        page_id: String,
    },
    Workspace {
        workspace: bool,
    },
    BlockId {
        block_id: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct HostedFile {
    pub url: String,
    pub expiry_time: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ExternalFile {
    pub url: String,
}
