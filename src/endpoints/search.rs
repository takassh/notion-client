use std::rc::Rc;

use reqwest::Client;

pub mod title;
pub struct SearchEndpoint {
    pub(super) client: Rc<Client>,
}
