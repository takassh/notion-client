use reqwest::Client;

pub mod create;
pub mod query;
pub mod retrieve;
#[cfg(test)]
mod tests;
pub mod update;

#[derive(Debug, Clone)]
pub struct DatabasesEndpoint {
    pub(super) client: Client,
}
