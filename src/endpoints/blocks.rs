use reqwest::Client;

pub mod append;
pub mod delete;
pub mod retrieve;
#[cfg(test)]
mod tests;
pub mod update;

#[derive(Debug, Clone)]
pub struct BlocksEndpoint {
    pub(super) client: Client,
}
