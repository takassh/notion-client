pub mod response;

use urlencoding::decode;

use crate::{endpoints::NOTION_URI, objects::page::Page, NotionClientError};

use self::response::RetrieveAPagePropertyItemResponse;

use super::PagesEndpoint;

impl PagesEndpoint {
    pub async fn retrieve_a_page(
        &self,
        page_id: &str,
        filter_properties: Option<Vec<&str>>,
    ) -> Result<Page, NotionClientError> {
        let filter_properties: Vec<_> = filter_properties
            .iter()
            .flatten()
            .map(|p| ("filter_properties", decode(p).unwrap()))
            .collect();

        let result = self
            .client
            .get(format!(
                "{notion_uri}/pages/{page_id}",
                notion_uri = NOTION_URI,
                page_id = page_id
            ))
            .query(&filter_properties)
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

    pub async fn retrieve_a_page_property_item(
        &self,
        page_id: &str,
        property_id: &str,
        page_size: Option<u32>,
        start_cursor: Option<&str>,
    ) -> Result<RetrieveAPagePropertyItemResponse, NotionClientError> {
        let mut query = vec![];
        let page_size = page_size.map(|p| p.to_string());
        if let Some(start_cursor) = start_cursor {
            query.insert(0, ("start_cursor", start_cursor));
        }
        if let Some(page_size) = &page_size {
            query.insert(0, ("page_size", page_size));
        }

        let result = self
            .client
            .get(format!(
                "{notion_uri}/pages/{page_id}/properties/{property_id}",
                notion_uri = NOTION_URI,
                page_id = page_id,
                property_id = property_id,
            ))
            .query(&query)
            .send()
            .await
            .map_err(|e| NotionClientError::FailedToRequest { source: e })?;

        let body = result
            .text()
            .await
            .map_err(|e| NotionClientError::FailedToText { source: e })?;

        let response = serde_json::from_str::<RetrieveAPagePropertyItemResponse>(&body)
            .map_err(|e| NotionClientError::FailedToDeserialize { source: e, body })?;

        Ok(response)
    }
}
