pub mod response;

use crate::{endpoints::NOTION_URI, objects::Response, NotionClientError};

use response::RetrieveCommentsResponse;

use super::CommentsEndpoint;

impl CommentsEndpoint {
    pub async fn retrieve_a_user(
        &self,
        block_id: &str,
        start_cursor: Option<&str>,
        page_size: Option<u32>,
    ) -> Result<RetrieveCommentsResponse, NotionClientError> {
        let mut query = vec![];
        if let Some(start_cursor) = start_cursor {
            query.insert(0, ("start_cursor", start_cursor));
        }
        let page_size = page_size.map(|p| p.to_string());
        if let Some(page_size) = &page_size {
            query.insert(0, ("page_size", page_size));
        }

        let result = self
            .client
            .get(format!(
                "{notion_uri}/comments?block_id={block_id}",
                notion_uri = NOTION_URI,
                block_id = block_id,
            ))
            .query(&query)
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
