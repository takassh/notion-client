pub mod request;
pub mod response;

use crate::{endpoints::NOTION_URI, NotionClientError};

use self::{request::QueryDatabaseRequest, response::QueryDatabaseResponse};

use super::DatabasesEndpoint;

impl DatabasesEndpoint {
    pub async fn query_a_database(
        &self,
        database_id: &str,
        request: QueryDatabaseRequest,
    ) -> Result<QueryDatabaseResponse, NotionClientError> {
        let json = serde_json::to_string(&request)
            .map_err(|e| NotionClientError::FailedToSerialize { source: e })?;

        let result = self
            .client
            .post(format!(
                "{notion_uri}/databases/{database_id}/query",
                notion_uri = NOTION_URI,
                database_id = database_id
            ))
            .body(json)
            .send()
            .await
            .map_err(|e| NotionClientError::FailedToRequest { source: e })?;

        let body = result
            .text()
            .await
            .map_err(|e| NotionClientError::FailedToText { source: e })?;

        let response = serde_json::from_str::<QueryDatabaseResponse>(&body)
            .map_err(|e| NotionClientError::FailedToDeserialize { source: e, body })?;

        Ok(response)
    }
}
