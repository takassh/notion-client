use notion_client::endpoints::{
    search::title::request::{Filter, SearchByTitleRequestBuilder, Sort, SortDirection, Timestamp},
    Client,
};
use reqwest::ClientBuilder;

const NOTION_TOKEN: &str = "";

#[tokio::main]
async fn main() {
    // Initialize client
    let client = Client::new(NOTION_TOKEN.to_string(), Some(ClientBuilder::new()));
    let Ok(client) = client else {
        panic!("error");
    };

    // Set up request parameters
    let mut request = SearchByTitleRequestBuilder::default();
    request.filter(Filter {
        value: notion_client::endpoints::search::title::request::FilterValue::Database,
        property: notion_client::endpoints::search::title::request::FilterProperty::Object,
    });
    request.sort(Sort {
        timestamp: Timestamp::LastEditedTime,
        direction: SortDirection::Ascending,
    });

    // Send request
    let res = client
        .search
        .search_by_title(request.build().unwrap())
        .await;

    // See result
    print!("{:#?}", res);
}
