use std::rc::Rc;

use reqwest::Client;

pub mod create;
pub mod retrieve;
#[cfg(test)]
mod tests;
pub mod update;

pub struct PagesEndpoint {
    pub(super) client: Rc<Client>,
}
