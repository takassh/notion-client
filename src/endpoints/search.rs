use reqwest::Client;

#[cfg(test)]
mod tests;
pub mod title;

#[derive(Debug, Clone)]
pub struct SearchEndpoint {
    pub(super) client: Client,
}
