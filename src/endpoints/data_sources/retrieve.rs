use crate::{
    endpoints::{parse_response, NOTION_URI},
    objects::data_source::DataSource,
    NotionClientError,
};

use super::DataSourcesEndpoint;

impl DataSourcesEndpoint {
    pub async fn retrieve_a_data_source(
        &self,
        data_source_id: &str,
    ) -> Result<DataSource, NotionClientError> {
        let result = self
            .client
            .get(format!(
                "{notion_uri}/data_sources/{data_source_id}",
                notion_uri = NOTION_URI,
                data_source_id = data_source_id
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
