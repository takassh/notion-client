use reqwest::Client;

pub mod create;
pub mod retrieve;
#[cfg(test)]
mod tests;
pub mod update;

#[derive(Debug, Clone)]
pub struct PagesEndpoint {
    pub(super) client: Client,
}
