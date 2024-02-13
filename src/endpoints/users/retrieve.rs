use crate::{
    endpoints::NOTION_URI,
    objects::{user::User, Response},
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

        let body = result
            .text()
            .await
            .map_err(|e| NotionClientError::FailedToText { source: e })?;

        let response = serde_json::from_str(&body)
            .map_err(|e| NotionClientError::FailedToDeserialize { source: e })?;

        match response {
            Response::Success(r) => Ok(r),
            Response::Error(e) => Err(NotionClientError::InvalidStatusCode { error: e }),
        }
    }

    pub async fn retrieve_your_tokens_bot_user(&self) -> Result<User, NotionClientError> {
        let result = self
            .client
            .get(format!("{notion_uri}/users/me", notion_uri = NOTION_URI,))
            .send()
            .await
            .map_err(|e| NotionClientError::FailedToRequest { source: e })?;

        let body = result
            .text()
            .await
            .map_err(|e| NotionClientError::FailedToText { source: e })?;

        let response = serde_json::from_str(&body)
            .map_err(|e| NotionClientError::FailedToDeserialize { source: e })?;

        match response {
            Response::Success(r) => Ok(r),
            Response::Error(e) => Err(NotionClientError::InvalidStatusCode { error: e }),
        }
    }
}
