use std::sync::Arc;

use reqwest::Client;

pub mod create;
pub mod query;
pub mod retrieve;
#[cfg(test)]
mod tests;
pub mod update;

pub struct DatabasesEndpoint {
    pub(super) client: Arc<Client>,
}
