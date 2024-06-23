use notion_client::endpoints::Client;
use reqwest::ClientBuilder;

const NOTION_TOKEN: &str = "";

#[tokio::main]
async fn main() {
    // Initialize client
    let client = Client::new(NOTION_TOKEN.to_string(), Some(ClientBuilder::new()));
    let Ok(client) = client else {
        panic!("error");
    };

    // Send request
    let res = client.users.list_all_users(None, None).await;

    // See result
    print!("{:#?}", res);
}
