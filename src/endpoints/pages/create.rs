pub mod request;

use crate::{
    endpoints::NOTION_URI,
    objects::{page::Page, Response},
    NotionClientError,
};

use self::request::CreateAPageRequest;

use super::PagesEndpoint;

impl PagesEndpoint {
    pub async fn create_a_page(
        &self,
        request: CreateAPageRequest,
    ) -> Result<Page, NotionClientError> {
        let json = serde_json::to_string(&request)
            .map_err(|e| NotionClientError::FailedToSerialize { source: e })?;

        let result = self
            .client
            .post(format!("{notion_uri}/pages", notion_uri = NOTION_URI,))
            .body(json)
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
