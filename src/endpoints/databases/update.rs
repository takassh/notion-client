pub mod request;

use crate::{
    endpoints::{parse_response, NOTION_URI},
    objects::database::Database,
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
            .patch(format!(
                "{notion_uri}/databases/{database_id}",
                notion_uri = NOTION_URI,
                database_id = database_id
            ))
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
