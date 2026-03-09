use reqwest::Url;

use crate::{endpoints::{parse_response, NOTION_URI}, NotionClientError};

pub mod response;

use self::response::ListAllUsersResponse;

use super::UsersEndpoint;

impl UsersEndpoint {
    pub async fn list_all_users(
        &self,
        start_cursor: Option<&str>,
        page_size: Option<u32>,
    ) -> Result<ListAllUsersResponse, NotionClientError> {
        let mut query = vec![];
        if let Some(start_cursor) = start_cursor {
            query.insert(0, ("start_cursor", start_cursor));
        }
        let page_size = page_size.map(|p| p.to_string());
        if let Some(page_size) = &page_size {
            query.insert(0, ("page_size", page_size));
        }

        let url = Url::parse_with_params(&format!("{NOTION_URI}/users"), query)?;
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
