pub mod request;
pub mod response;

use crate::{endpoints::{parse_response, NOTION_URI}, NotionClientError};

use self::{request::SearchByTitleRequest, response::SearchByTitleResponse};

use super::SearchEndpoint;

impl SearchEndpoint {
    pub async fn search_by_title(
        &self,
        request: SearchByTitleRequest,
    ) -> Result<SearchByTitleResponse, NotionClientError> {
        let json = serde_json::to_string(&request)
            .map_err(|e| NotionClientError::FailedToSerialize { source: e })?;

        let result = self
            .client
            .post(format!("{notion_uri}/search", notion_uri = NOTION_URI))
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
