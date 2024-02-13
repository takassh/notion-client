pub mod request;
pub mod response;

use crate::{endpoints::NOTION_URI, objects::Response, NotionClientError};

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
            .post(format!("{notion_uri}/search", notion_uri = NOTION_URI,))
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
