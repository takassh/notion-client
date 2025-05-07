# notion-client
[![Build](https://github.com/takassh/notion-client/actions/workflows/build.yml/badge.svg)](https://github.com/takassh/notion-client/actions/workflows/build.yml)
[![Notion API](https://files.readme.io/a267aac-notion-devs-logo.svg)](https://developers.notion.com)
[![Crates.io](https://img.shields.io/crates/v/notion-client?style=for-the-badge)](https://crates.io/crates/notion-client)

Notion API client library for rust.
Now, this library supports all endpoints except authentication!

## Getting Started

### Example for Query a Database
- Corresponding API
    - [Query a database](https://developers.notion.com/reference/post-database-query)

```rust

use notion_client::endpoints::{
    databases::query::request::{QueryDatabaseRequest, Sort, SortDirection, Timestamp},
    Client,
};

const NOTION_DB_ID: &str = ""; // ⚠️ Set your DB id which can be accessible from API
const NOTION_TOKEN: &str = ""; // ⚠️ Set your notion token

#[tokio::main]
async fn main() {
    // Initialize client
    let client = Client::new(NOTION_TOKEN.to_string(), None);
    let Ok(client) = client else {
        panic!("error");
    };

    // Set up request parameters
    let request = QueryDatabaseRequest {
        sorts: Some(vec![Sort::Timestamp {
            timestamp: Timestamp::CreatedTime,
            direction: SortDirection::Ascending,
        }]),
        ..Default::default()
    };

    // Send request
    let res = client
        .databases
        .query_a_database(NOTION_DB_ID, request)
        .await;

    // See result
    print!("{:#?}", res);
}

```

See more [examples](examples)

## TODO

- [x] support threadsafe
- [x] add more examples
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
- [x] add test to users endpoint
- [x] add test to comments endpoint
- [x] add test to search endpoint
- [x] support builder pattern

## Contributing

Contributions are always welcome!
If you have an idea, it's best to float it by us before working on it to ensure no effort is wasted.
If there's already an open issue for it, knock yourself out.

Here is [`CONTRIBUTING` Guide](./CONTRIBUTING.md).