use std::rc::Rc;

use reqwest::Client;

pub mod create;
pub mod retrieve;
pub struct CommentsEndpoint {
    pub(super) client: Rc<Client>,
}
