use std::rc::Rc;

use reqwest::Client;

pub mod list;
pub mod retrieve;
#[cfg(test)]
mod tests;
pub struct UsersEndpoint {
    pub(super) client: Rc<Client>,
}
