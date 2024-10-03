use reqwest::Client;

pub mod list;
pub mod retrieve;
#[cfg(test)]
mod tests;

#[derive(Debug, Clone)]
pub struct UsersEndpoint {
    pub(super) client: Client,
}
