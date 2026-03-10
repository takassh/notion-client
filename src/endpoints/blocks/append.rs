pub mod request;
pub mod response;

use crate::{
    endpoints::{parse_response, NOTION_URI},
    NotionClientError,
};

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

        let status = result.status();
        let body = result
            .text()
            .await
            .map_err(|e| NotionClientError::FailedToText { source: e })?;

        parse_response(status, body)
    }
}
