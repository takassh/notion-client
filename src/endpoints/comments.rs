use std::sync::Arc;

use reqwest::Client;

pub mod create;
pub mod retrieve;
#[cfg(test)]
mod tests;

#[derive(Clone)]
pub struct CommentsEndpoint {
    pub(super) client: Arc<Client>,
}
