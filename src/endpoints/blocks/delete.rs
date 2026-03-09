use crate::{
    endpoints::{parse_response, NOTION_URI},
    objects::block::Block,
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

        let status = result.status();
        let body = result
            .text()
            .await
            .map_err(|e| NotionClientError::FailedToText { source: e })?;

        parse_response(status, body)
    }
}
