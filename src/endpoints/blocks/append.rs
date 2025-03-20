pub mod request;
pub mod response;

use crate::{endpoints::NOTION_URI, NotionClientError};

use self::{request::AppendBlockChildrenRequest, response::AppendBlockChildrenResponse};

use super::BlocksEndpoint;

impl BlocksEndpoint {
    pub async fn append_block_children(
        &self,
        block_id: &str,
        request: AppendBlockChildrenRequest,
    ) -> Result<AppendBlockChildrenResponse, NotionClientError> {
        let json = serde_json::to_string(&request)
            .map_err(|e| NotionClientError::FailedToSerialize { source: e })?;

        let result = self
            .client
            .patch(format!(
                "{notion_uri}/blocks/{block_id}/children",
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

        let response = serde_json::from_str::<AppendBlockChildrenResponse>(&body)
            .map_err(|e| NotionClientError::FailedToDeserialize { source: e, body })?;

        Ok(response)
    }
}
