pub mod request;

use crate::{endpoints::NOTION_URI, objects::comment::Comment, NotionClientError};

use self::request::CreateCommentRequest;

use super::CommentsEndpoint;

impl CommentsEndpoint {
    pub async fn create_a_comment(
        &self,
        request: CreateCommentRequest,
    ) -> Result<Comment, NotionClientError> {
        let json = serde_json::to_string(&request)
            .map_err(|e| NotionClientError::FailedToSerialize { source: e })?;

        let result = self
            .client
            .post(format!("{notion_uri}/comments", notion_uri = NOTION_URI))
            .body(json)
            .send()
            .await
            .map_err(|e| NotionClientError::FailedToRequest { source: e })?;

        let body = result
            .text()
            .await
            .map_err(|e| NotionClientError::FailedToText { source: e })?;

        let response = serde_json::from_str::<Comment>(&body)
            .map_err(|e| NotionClientError::FailedToDeserialize { source: e, body })?;

        Ok(response)
    }
}
