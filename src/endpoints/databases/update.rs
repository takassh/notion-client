pub mod request;

use crate::{
    endpoints::NOTION_URI,
    objects::{database::Database, Response},
    NotionClientError,
};

use self::request::UpdateADatabaseRequest;

use super::DatabasesEndpoint;

impl DatabasesEndpoint {
    pub async fn update_a_database(
        &self,
        database_id: &str,
        request: UpdateADatabaseRequest,
    ) -> Result<Database, NotionClientError> {
        let json = serde_json::to_string(&request)
            .map_err(|e| NotionClientError::FailedToSerialize { source: e })?;

        let result = self
            .client
            .post(format!(
                "{notion_uri}/databases/{database_id}",
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

        let response = serde_json::from_str(&body)
            .map_err(|e| NotionClientError::FailedToDeserialize { source: e, body })?;

        match response {
            Response::Success(r) => Ok(r),
            Response::Error(e) => Err(NotionClientError::InvalidStatusCode { error: e }),
        }
    }
}
