pub mod request;

use crate::{endpoints::NOTION_URI, objects::page::Page, NotionClientError};

use self::request::UpdatePagePropertiesRequest;

use super::PagesEndpoint;

impl PagesEndpoint {
    pub async fn update_page_properties(
        &self,
        page_id: &str,
        request: UpdatePagePropertiesRequest,
    ) -> Result<Page, NotionClientError> {
        let json = serde_json::to_string(&request)
            .map_err(|e| NotionClientError::FailedToSerialize { source: e })?;

        let result = self
            .client
            .patch(format!(
                "{notion_uri}/pages/{page_id}",
                notion_uri = NOTION_URI,
                page_id = page_id
            ))
            .body(json)
            .send()
            .await
            .map_err(|e| NotionClientError::FailedToRequest { source: e })?;

        let body = result
            .text()
            .await
            .map_err(|e| NotionClientError::FailedToText { source: e })?;

        let response = serde_json::from_str::<Page>(&body)
            .map_err(|e| NotionClientError::FailedToDeserialize { source: e, body })?;

        Ok(response)
    }
}
