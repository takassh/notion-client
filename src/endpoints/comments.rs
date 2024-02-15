use std::rc::Rc;

use reqwest::Client;

pub mod create;
pub mod retrieve;
#[cfg(test)]
mod tests;
pub struct CommentsEndpoint {
    pub(super) client: Rc<Client>,
}
