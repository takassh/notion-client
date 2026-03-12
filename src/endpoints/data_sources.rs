use reqwest::Client;

pub mod create;
pub mod query;
pub mod retrieve;
pub mod update;

#[derive(Debug, Clone)]
pub struct DataSourcesEndpoint {
    pub(super) client: Client,
}
