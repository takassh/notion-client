pub mod request;

use crate::{
    endpoints::{parse_response, NOTION_URI},
    objects::comment::Comment,
    NotionClientError,
};

use self::request::CreateCommentRequest;

use super::CommentsEndpoint;

impl CommentsEndpoint {
    pub async fn create_comment(
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

        let status = result.status();
        let body = result
            .text()
            .await
            .map_err(|e| NotionClientError::FailedToText { source: e })?;

        parse_response(status, body)
    }
}
