use notion_client::endpoints::Client;
use reqwest::ClientBuilder;

const NOTION_DB_ID: &str = "";
const NOTION_TOKEN: &str = "";

#[tokio::main]
async fn main() {
    // Initialize client
    let client = Client::new(NOTION_TOKEN.to_string(), Some(ClientBuilder::new()));
    let Ok(client) = client else {
        panic!("error");
    };

    // Send request
    let res = client.databases.retrieve_a_database(NOTION_DB_ID).await;

    // See result
    print!("{:#?}", res);
}
