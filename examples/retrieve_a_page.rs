use notion_client::endpoints::Client;
use reqwest::ClientBuilder;

const NOTION_PAGE_ID: &str = "";
const NOTION_TOKEN: &str = "";

#[tokio::main]
async fn main() {
    // Initialize client
    let client = Client::new(NOTION_TOKEN.to_string(), Some(ClientBuilder::new()));
    let Ok(client) = client else {
        panic!("error");
    };

    // Send request
    let res = client
        .pages
        .retrieve_a_page(NOTION_PAGE_ID, Some(vec!["%3B%60%3BG"]))
        .await;

    // See result
    print!("{:#?}", res);
}
