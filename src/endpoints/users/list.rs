use crate::{endpoints::NOTION_URI, objects::Response, NotionClientError};

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

        let result = self
            .client
            .get(format!("{notion_uri}/users", notion_uri = NOTION_URI,))
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
