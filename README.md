# notion-client
[![Build](https://github.com/takassh/notion-client/actions/workflows/build.yml/badge.svg)](https://github.com/takassh/notion-client/actions/workflows/build.yml)
[![Notion API](https://files.readme.io/a267aac-notion-devs-logo.svg)](https://developers.notion.com)
[![Crates.io](https://img.shields.io/crates/v/notion-client?style=for-the-badge)](https://crates.io/crates/notion-client)

Notion API client library for rust.

## Getting Started

### Example for Query a Database
- Corresponding API
    - [Query a database](https://developers.notion.com/reference/post-database-query)

```rust

use notion_client::endpoints::{
    databases::query::request::{
        Filter, FilterType, PropertyCondition, QueryDatabaseRequest, SelectCondition, Sort,
        SortDirection, Timestamp,
    },
    Client,
};

const NOTION_ARTICLES_DB_ID: &str = "";
const NOTION_TOKEN: &str = "";

#[tokio::main]
async fn main() {
    // Initialize client
    let client = Client::new(NOTION_TOKEN.to_string());
    let Ok(client) = client else {
        panic!("error");
    };

    // Set up request parameters
    let request = QueryDatabaseRequest {
        filter: Some(Filter::Value {
            filter_type: FilterType::Property {
                property: "status".to_string(),
                condition: PropertyCondition::Select(SelectCondition::Equals(
                    "article".to_string(),
                )),
            },
        }),
        sorts: Some(vec![Sort::Timestamp {
            timestamp: Timestamp::CreatedTime,
            direction: SortDirection::Ascending,
        }]),
        ..Default::default()
    };

    // Send request
    let res = client
        .databases
        .query_a_database(NOTION_ARTICLES_DB_ID, request)
        .await;

    // See result
    print!("{:#?}", res);
}

```

## TODO

- [ ] add more examples
- [x] support blocks endpoint
- [x] support pages endpoint
- [x] support databases endpoint
- [x] support users endpoint
- [x] support comments endpoint
- [x] support search endpoint
- [ ] support authentication endpoint
- [x] add test to blocks endpoint
- [x] add test to pages endpoint
- [x] add test to databases endpoint
- [ ] add test to users endpoint
- [ ] add test to comments endpoint
- [ ] add test to search endpoint
- [ ] add test to endpoint

## Contributing

Contributions are always welcome!
If you have an idea, it's best to float it by us before working on it to ensure no effort is wasted.
If there's already an open issue for it, knock yourself out.

If you have any questions, feel free to use [Discussions](https://github.com/takassh/notion-client/discussions).
Please don't hesitate to ask questions!