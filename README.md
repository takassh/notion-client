# notion-client
[![Build](https://github.com/takassh/notion-client/actions/workflows/build.yml/badge.svg)](https://github.com/takassh/notion-client/actions/workflows/build.yml)
[![Notion API](https://files.readme.io/a267aac-notion-devs-logo.svg)](https://developers.notion.com)
[![Crates.io](https://img.shields.io/crates/v/notion-client?style=for-the-badge)](https://crates.io/crates/notion-client)

A **Rust client library** for the [Notion API](https://developers.notion.com).
Supports (almost) all endpoints — everything except authentication, for now!

⚡️ **The only Rust crate that is actively maintained and always up-to-date with the latest Notion API version.**

## Features

* ✅ Supports **databases**, **pages**, **blocks**, **users**, **comments**, and **search** endpoints
* 🔒 Thread-safe client
* 🧰 Builder pattern support
* 📝 Rich set of examples
* 🚀 Actively maintained and growing!
* 🔄 Always in sync with the latest Notion API updates

## Getting Started

### Example for Query a Database

*Corresponding Notion API: [Query a database](https://developers.notion.com/reference/post-database-query)*

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

👉 See more [examples](./examples)

## Roadmap

* [x] Thread-safe support
* [x] More examples
* [x] Support blocks endpoint
* [x] Support pages endpoint
* [x] Support databases endpoint
* [x] Support users endpoint
* [x] Support comments endpoint
* [x] Support search endpoint
* [ ] Support authentication endpoint
* [x] Add tests to blocks endpoint
* [x] Add tests to pages endpoint
* [x] Add tests to databases endpoint
* [x] Add tests to users endpoint
* [x] Add tests to comments endpoint
* [x] Add tests to search endpoint
* [x] Support builder pattern

Feel free to suggest new features or improvements! 🙌

## Contributing

Contributions are **welcome and appreciated**! ❤️

If you have an idea:

* Please open an issue first to discuss it. This ensures no effort is wasted.
* If there’s already an open issue, feel free to grab it and start contributing!

Here’s our [**Contributing Guide**](./CONTRIBUTING.md).

Let’s make this library even better together!

## License

This project is licensed under the [MIT License](./LICENSE).

## FAQ

**Q: Is authentication supported?**
A: Not yet — it's on our roadmap!

**Q: Is this an official Notion SDK?**
A: No, this is a community-driven open-source library.

**Q: How is this different from other crates?**
A: This is the **only Rust crate actively maintained and updated to match the latest Notion API versions**. If you want up-to-date support, you’re in the right place!

## Support

If you find this library useful:

* ⭐ Star this repository
* 📢 Share it with fellow Rustaceans
* 🤝 Contribute back!