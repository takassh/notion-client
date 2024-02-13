use std::rc::Rc;

use reqwest::Client;

pub mod list;
pub mod retrieve;
pub struct UsersEndpoint {
    pub(super) client: Rc<Client>,
}
