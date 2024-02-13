use crate::{
    endpoints::NOTION_URI,
    objects::{page::Page, Response},
    NotionClientError,
};

use super::PagesEndpoint;

impl PagesEndpoint {
    pub async fn retrieve_a_page(
        &self,
        page_id: &str,
        filter_properties: Option<Vec<(&str, &str)>>,
    ) -> Result<Page, NotionClientError> {
        let mut query = vec![];
        if let Some(filter_properties) = filter_properties {
            query = filter_properties;
        }

        let result = self
            .client
            .get(format!(
                "{notion_uri}/pages/{page_id}",
                notion_uri = NOTION_URI,
                page_id = page_id
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
            .map_err(|e| NotionClientError::FailedToDeserialize { source: e })?;

        match response {
            Response::Success(r) => Ok(r),
            Response::Error(e) => Err(NotionClientError::InvalidStatusCode { error: e }),
        }
    }

    pub async fn retrieve_a_page_property_item(&self) -> Result<(), NotionClientError> {
        // TODO:
        Ok(())
    }
}
