pub mod request;

use crate::{
    endpoints::{parse_response, NOTION_URI},
    objects::data_source::DataSource,
    NotionClientError,
};

use self::request::UpdateADataSourceRequest;

use super::DataSourcesEndpoint;

impl DataSourcesEndpoint {
    pub async fn update_a_data_source(
        &self,
        data_source_id: &str,
        request: UpdateADataSourceRequest,
    ) -> Result<DataSource, NotionClientError> {
        let json = serde_json::to_string(&request)
            .map_err(|e| NotionClientError::FailedToSerialize { source: e })?;

        let result = self
            .client
            .patch(format!(
                "{notion_uri}/data_sources/{data_source_id}",
                notion_uri = NOTION_URI,
                data_source_id = data_source_id
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
