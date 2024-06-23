use notion_client::endpoints::{
    databases::query::request::{QueryDatabaseRequestBuilder, Sort, SortDirection, Timestamp},
    Client,
};
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

    // Set up request parameters
    let mut request = QueryDatabaseRequestBuilder::default();
    request.sorts(vec![Sort::Timestamp {
        timestamp: Timestamp::CreatedTime,
        direction: SortDirection::Ascending,
    }]);

    // Send request
    let res = client
        .databases
        .query_a_database(NOTION_DB_ID, request.build().unwrap())
        .await;

    // See result
    print!("{:#?}", res);
}
