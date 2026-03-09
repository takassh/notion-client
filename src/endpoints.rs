use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    ClientBuilder, StatusCode,
};
use serde::de::DeserializeOwned;

use crate::{objects::error::Error, NotionClientError};

use self::{
    blocks::BlocksEndpoint, comments::CommentsEndpoint, databases::DatabasesEndpoint,
    pages::PagesEndpoint, search::SearchEndpoint, users::UsersEndpoint,
};

pub mod blocks;
pub mod comments;
pub mod databases;
pub mod pages;
pub mod search;
pub mod users;

const NOTION_URI: &str = "https://api.notion.com/v1";

pub(crate) fn parse_response<T: DeserializeOwned>(
    status: StatusCode,
    body: String,
) -> Result<T, NotionClientError> {
    if !status.is_success() {
        let error = serde_json::from_str::<Error>(&body)
            .map_err(|e| NotionClientError::FailedToDeserialize { source: e, body })?;
        return Err(NotionClientError::InvalidStatusCode { error });
    }
    serde_json::from_str::<T>(&body)
        .map_err(|e| NotionClientError::FailedToDeserialize { source: e, body })
}
const NOTION_VERSION: &str = "2022-06-28";

#[derive(Debug, Clone)]
pub struct Client {
    pub blocks: BlocksEndpoint,
    pub comments: CommentsEndpoint,
    pub databases: DatabasesEndpoint,
    pub pages: PagesEndpoint,
    pub search: SearchEndpoint,
    pub users: UsersEndpoint,
}

impl Client {
    pub fn new(
        token: String,
        mut builder: Option<ClientBuilder>,
    ) -> Result<Self, NotionClientError> {
        let mut headers = HeaderMap::new();
        headers.insert("Notion-Version", HeaderValue::from_static(NOTION_VERSION));
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        let mut auth_value = HeaderValue::from_str(&format!("Bearer {}", token))
            .map_err(|e| NotionClientError::InvalidHeader { source: e })?;
        auth_value.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, auth_value);

        if builder.is_none() {
            builder = Some(ClientBuilder::new().default_headers(headers));
        } else {
            builder = Some(builder.unwrap().default_headers(headers));
        }

        let client = builder
            .unwrap()
            .build()
            .map_err(|e| NotionClientError::FailedToBuildRequest { source: e })?;

        Ok(Self {
            blocks: BlocksEndpoint {
                client: client.clone(),
            },
            comments: CommentsEndpoint {
                client: client.clone(),
            },
            databases: DatabasesEndpoint {
                client: client.clone(),
            },
            pages: PagesEndpoint {
                client: client.clone(),
            },
            search: SearchEndpoint {
                client: client.clone(),
            },
            users: UsersEndpoint {
                client: client.clone(),
            },
        })
    }
}
