use std::sync::Arc;

use reqwest::Client;

pub mod append;
pub mod delete;
pub mod retrieve;
#[cfg(test)]
mod tests;
pub mod update;

pub struct BlocksEndpoint {
    pub(super) client: Arc<Client>,
}
