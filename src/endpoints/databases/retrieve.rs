use crate::{
    endpoints::{parse_response, NOTION_URI},
    objects::database::Database,
    NotionClientError,
};

use super::DatabasesEndpoint;

impl DatabasesEndpoint {
    pub async fn retrieve_a_database(
        &self,
        database_id: &str,
    ) -> Result<Database, NotionClientError> {
        let result = self
            .client
            .get(format!(
                "{notion_uri}/databases/{database_id}",
                notion_uri = NOTION_URI,
                database_id = database_id
            ))
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
