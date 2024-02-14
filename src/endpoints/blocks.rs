use std::rc::Rc;

use reqwest::Client;

pub mod append;
pub mod delete;
pub mod retrieve;
pub mod update;
#[cfg(test)]
mod tests;

pub struct BlocksEndpoint {
    pub(super) client: Rc<Client>,
}
