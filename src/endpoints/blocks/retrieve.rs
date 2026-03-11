pub mod response;

use reqwest::Url;

use crate::{
    endpoints::{parse_response, NOTION_URI},
    objects::block::Block,
    NotionClientError,
};

use self::response::RetrieveBlockChilerenResponse;

use super::BlocksEndpoint;

impl BlocksEndpoint {
    pub async fn retrieve_a_block(&self, block_id: &str) -> Result<Block, NotionClientError> {
        let result = self
            .client
            .get(format!(
                "{notion_uri}/blocks/{block_id}",
                notion_uri = NOTION_URI,
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

    pub async fn retrieve_block_children(
        &self,
        block_id: &str,
        start_cursor: Option<&str>,
        page_size: Option<u32>,
    ) -> Result<RetrieveBlockChilerenResponse, NotionClientError> {
        let mut query = vec![];
        if let Some(start_cursor) = start_cursor {
            query.insert(0, ("start_cursor", start_cursor));
        }
        let page_size = page_size.map(|p| p.to_string());
        if let Some(page_size) = &page_size {
            query.insert(0, ("page_size", page_size));
        }

        let url = Url::parse_with_params(
            &format!(
                "{notion_uri}/blocks/{block_id}/children",
                notion_uri = NOTION_URI,
            ),
            query,
        )?;
        let result = self
            .client
            .get(url)
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
