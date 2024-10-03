use reqwest::Client;

pub mod create;
pub mod retrieve;
#[cfg(test)]
mod tests;

#[derive(Debug, Clone)]
pub struct CommentsEndpoint {
    pub(super) client: Client,
}
