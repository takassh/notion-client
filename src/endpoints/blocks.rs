use std::rc::Rc;

use reqwest::Client;

pub mod append;
pub mod delete;
pub mod retrieve;

pub struct BlocksEndpoint {
    pub(super) client: Rc<Client>,
}

#[cfg(test)]
mod tests {
    use httpmock::prelude::*;

    #[test]
    fn append() {
        let server = MockServer::start();

        let _hello_mock = server.mock(|when, then| {
            when.method(GET).path("/");
            then.status(200).header("content-type", "text/json");
        });

        assert_eq!(3, 3);
    }
}
