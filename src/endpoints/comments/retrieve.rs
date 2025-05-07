pub mod response;

use crate::{endpoints::NOTION_URI, NotionClientError};

use self::response::RetrieveCommentsResponse;

use super::CommentsEndpoint;

impl CommentsEndpoint {
    pub async fn retrieve_comments(
        &self,
        block_id: Option<&str>,
        page_size: Option<u32>,
        start_cursor: Option<&str>,
    ) -> Result<RetrieveCommentsResponse, NotionClientError> {
        let mut query = vec![];
        if let Some(block_id) = block_id {
            query.insert(0, ("block_id", block_id));
        }
        let page_size = page_size.map(|p| p.to_string());
        if let Some(page_size) = &page_size {
            query.insert(0, ("page_size", page_size));
        }
        if let Some(start_cursor) = start_cursor {
            query.insert(0, ("start_cursor", start_cursor));
        }

        let result = self
            .client
            .get(format!("{notion_uri}/comments", notion_uri = NOTION_URI))
            .query(&query)
            .send()
            .await
            .map_err(|e| NotionClientError::FailedToRequest { source: e })?;

        let body = result
            .text()
            .await
            .map_err(|e| NotionClientError::FailedToText { source: e })?;

        let response = serde_json::from_str::<RetrieveCommentsResponse>(&body)
            .map_err(|e| NotionClientError::FailedToDeserialize { source: e, body })?;

        Ok(response)
    }
}
