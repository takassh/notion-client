use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum File {
    External { external: ExternalFile },
    File { file: HostedFile },
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
