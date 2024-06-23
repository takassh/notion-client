use crate::{
    endpoints::NOTION_URI,
    objects::{block::Block, Response},
    NotionClientError,
};

use super::BlocksEndpoint;

impl BlocksEndpoint {
    pub async fn delete_a_block(&self, block_id: &str) -> Result<Block, NotionClientError> {
        let result = self
            .client
            .delete(format!(
                "{notion_uri}/blocks/{block_id}",
                notion_uri = NOTION_URI,
                block_id = block_id
            ))
            .send()
            .await
            .map_err(|e| NotionClientError::FailedToRequest { source: e })?;

        let body = result
            .text()
            .await
            .map_err(|e| NotionClientError::FailedToText { source: e })?;

        let response = serde_json::from_str(&body)
            .map_err(|e| NotionClientError::FailedToDeserialize { source: e, body })?;

        match response {
            Response::Success(r) => Ok(r),
            Response::Error(e) => Err(NotionClientError::InvalidStatusCode { error: e }),
        }
    }
}
