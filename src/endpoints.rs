use std::sync::Arc;

use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    ClientBuilder,
};

use crate::NotionClientError;

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
const NOTION_VERSION: &str = "2022-06-28";

pub struct Client {
    pub blocks: BlocksEndpoint,
    pub comments: CommentsEndpoint,
    pub databases: DatabasesEndpoint,
    pub pages: PagesEndpoint,
    pub search: SearchEndpoint,
    pub users: UsersEndpoint,
}

impl Client {
    pub fn new(token: String) -> Result<Self, NotionClientError> {
        let mut headers = HeaderMap::new();
        headers.insert("Notion-Version", HeaderValue::from_static(NOTION_VERSION));
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        let mut auth_value = HeaderValue::from_str(&format!("Bearer {}", token))
            .map_err(|e| NotionClientError::InvalidHeader { source: e })?;
        auth_value.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, auth_value);

        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .map_err(|e| NotionClientError::FailedToBuildRequest { source: e })?;

        let client = Arc::new(client);

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
