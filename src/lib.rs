pub mod endpoints;
pub mod objects;

use objects::error::Error;
use reqwest::header::InvalidHeaderValue;

#[derive(Debug, thiserror::Error)]
pub enum NotionClientError {
    #[error("Failed to serialize: {}", source)]
    FailedToSerialize { source: serde_json::Error },

    #[error("Failed to deserialize: {}", source)]
    FailedToDeserialize { source: serde_json::Error },

    #[error("Failed to request: {}", source)]
    FailedToRequest { source: reqwest::Error },

    #[error("Failed to text: {}", source)]
    FailedToText { source: reqwest::Error },

    #[error("Failed to build request {}", source)]
    FailedToBuildRequest { source: reqwest::Error },

    #[error("Invalid status code {}({}): {}", .error.code, .error.status, .error.message)]
    InvalidStatusCode { error: Error },

    #[error("Invalid header {}", source)]
    InvalidHeader { source: InvalidHeaderValue },
}
