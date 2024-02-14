use std::rc::Rc;

use reqwest::Client;

pub mod append;
pub mod delete;
pub mod retrieve;
#[cfg(test)]
mod tests;
pub mod update;

pub struct BlocksEndpoint {
    pub(super) client: Rc<Client>,
}
