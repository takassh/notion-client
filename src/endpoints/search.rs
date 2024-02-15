use std::rc::Rc;

use reqwest::Client;

#[cfg(test)]
mod tests;
pub mod title;
pub struct SearchEndpoint {
    pub(super) client: Rc<Client>,
}
