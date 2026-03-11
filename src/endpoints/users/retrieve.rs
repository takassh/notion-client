use crate::{
    endpoints::{parse_response, NOTION_URI},
    objects::user::User,
    NotionClientError,
};

use super::UsersEndpoint;

impl UsersEndpoint {
    pub async fn retrieve_a_user(&self, user_id: &str) -> Result<User, NotionClientError> {
        let result = self
            .client
            .get(format!(
                "{notion_uri}/users/{user_id}",
                notion_uri = NOTION_URI,
                user_id = user_id
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

    pub async fn retrieve_your_tokens_bot_user(&self) -> Result<User, NotionClientError> {
        let result = self
            .client
            .get(format!("{notion_uri}/users/me", notion_uri = NOTION_URI,))
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
