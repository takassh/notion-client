use crate::{
    endpoints::NOTION_URI,
    objects::{block::Block, Response},
    NotionClientError,
};

use super::BlocksEndpoint;

impl BlocksEndpoint {
    pub async fn update_a_block(
        &self,
        block_id: &str,
        request: Block,
    ) -> Result<Block, NotionClientError> {
        let json = serde_json::to_string(&request)
            .map_err(|e| NotionClientError::FailedToSerialize { source: e })?;

        let result = self
            .client
            .patch(format!(
                "{notion_uri}/blocks/{block_id}",
                notion_uri = NOTION_URI,
                block_id = block_id
            ))
            .body(json)
            .send()
            .await
            .map_err(|e| NotionClientError::FailedToRequest { source: e })?;

        let body = result
            .text()
            .await
            .map_err(|e| NotionClientError::FailedToText { source: e })?;

        let response = serde_json::from_str(&body)
            .map_err(|e| NotionClientError::FailedToDeserialize { source: e })?;

        match response {
            Response::Success(r) => Ok(r),
            Response::Error(e) => Err(NotionClientError::InvalidStatusCode { error: e }),
        }
    }
}
