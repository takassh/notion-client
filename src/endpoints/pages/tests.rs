use std::collections::BTreeMap;

use serde_json::Number;

use crate::{
    endpoints::pages::{
        create::request::CreateAPageRequest, retrieve::response::RetrieveAPagePropertyItemResponse,
        update::request::UpdatePagePropertiesRequest,
    },
    objects::{
        block::{Block, BlockType, HeadingsValue, ParagraphValue},
        emoji::Emoji,
        file::{ExternalFile, File},
        page::{Icon, Page, PageProperty, SelectPropertyValue},
        parent::Parent,
        rich_text::{Link, RichText, Text},
    },
};

#[test]
fn test_create_request() {
    let mut properties = BTreeMap::new();
    properties.insert(
        "Name".to_string(),
        PageProperty::Title {
            id: None,
            title: vec![RichText::Text {
                text: Text {
                    content: "Tuscan Kale".to_string(),
                    link: None,
                },
                annotations: None,
                plain_text: None,
                href: None,
            }],
        },
    );
    properties.insert(
        "Description".to_string(),
        PageProperty::RichText {
            id: None,

            rich_text: vec![RichText::Text {
                text: Text {
                    content: "A dark green leafy vegetable".to_string(),
                    link: None,
                },
                annotations: None,
                plain_text: None,
                href: None,
            }],
        },
    );
    properties.insert(
        "Food group".to_string(),
        PageProperty::Select {
            id: None,
            select: Some(SelectPropertyValue {
                name: Some("Vegetable".to_string()),
                ..Default::default()
            }),
        },
    );
    properties.insert(
        "Price".to_string(),
        PageProperty::Number {
            id: None,
            number: Some(Number::from_f64(2.5).unwrap()),
        },
    );

    let request = CreateAPageRequest {
        parent: Parent::DatabaseId {
            database_id: "d9824bdc84454327be8b5b47500af6ce".to_string(),
        },
        icon: Some(Icon::Emoji(Emoji {
            emoji: "🥬".to_string(),
        })),
        cover: Some(File::External {
            external: ExternalFile {
                url: "https://upload.wikimedia.org/wikipedia/commons/6/62/Tuscankale.jpg"
                    .to_string(),
            },
        }),
        properties,
        children: Some(vec![
            Block {
                object: Some("block".to_string()),
                block_type: BlockType::Heading2 {
                    heading_2: HeadingsValue {
                        rich_text: vec![RichText::Text {
                            text: Text {
                                content: "Lacinato kale".to_string(),
                                link: None,
                            },
                            annotations: None,
                            plain_text: None,
                            href: None,
                        }],
                        ..Default::default()
                    },
                },
                ..Default::default()
            },
            Block {
                object: Some("block".to_string()),
                block_type: BlockType::Paragraph {
                    paragraph: ParagraphValue {
                        rich_text: vec![RichText::Text {
                            text: Text {
                                content: "Lacinato kale is a variety of kale with a long tradition in Italian cuisine, especially that of Tuscany. It is also known as Tuscan kale, Italian kale, dinosaur kale, kale, flat back kale, palm tree kale, or black Tuscan palm.".to_string(),
                                link: Some(Link {
                                    url: "https://en.wikipedia.org/wiki/Lacinato_kale".to_string(),
                                }),
                            },
                            annotations: None,
                            plain_text: None,
                            href: None,
                        }],
                        ..Default::default()
                    },
                },
                ..Default::default()
            },
        ]),
    };

    let result = serde_json::to_string_pretty(&request).unwrap();
    let actual = include_str!("tests/create_request.json");
    assert_eq!(result, actual.to_string())
}

#[test]
fn test_update_null_request() {
    let mut properties = BTreeMap::new();
    properties.insert("Date".to_string(), None);

    let request = UpdatePagePropertiesRequest {
        properties: properties,
        ..Default::default()
    };

    let result = serde_json::to_string_pretty(&request).unwrap();
    let actual = include_str!("tests/update_null_request.json");
    assert_eq!(result, actual.to_string())
}

#[test]
fn test_update_request() {
    let mut properties = BTreeMap::new();
    properties.insert(
        "In stock".to_string(),
        Some(PageProperty::Checkbox {
            id: None,
            checkbox: true,
        }),
    );

    let request = UpdatePagePropertiesRequest {
        properties,
        ..Default::default()
    };

    let result = serde_json::to_string_pretty(&request).unwrap();
    let actual = include_str!("tests/update_request.json");
    assert_eq!(result, actual.to_string())
}

#[test]
fn test_create_200() {
    let result = serde_json::from_str::<Page>(include_str!("tests/create_200.json"));
    assert!(result.is_ok())
}

#[test]
fn test_retrieve_200() {
    let result = serde_json::from_str::<Page>(include_str!("tests/retrieve_200.json"));
    assert!(result.is_ok())
}

#[test]
fn test_retrieve_a_page_property_list_200() {
    let result = serde_json::from_str::<RetrieveAPagePropertyItemResponse>(include_str!(
        "tests/retrieve_a_page_property_list_200.json"
    ));
    assert!(result.is_ok())
}

#[test]
fn test_retrieve_a_page_property_item_200() {
    let result = serde_json::from_str::<RetrieveAPagePropertyItemResponse>(include_str!(
        "tests/retrieve_a_page_property_item_200.json"
    ));
    assert!(result.is_ok())
}

#[test]
fn test_retrieve_a_page_property_rollup_list_200() {
    let result = serde_json::from_str::<RetrieveAPagePropertyItemResponse>(include_str!(
        "tests/retrieve_a_page_property_rollup_list_200.json"
    ));
    assert!(result.is_ok())
}

#[test]
fn test_update_200() {
    let result = serde_json::from_str::<Page>(include_str!("tests/update_200.json"));
    assert!(result.is_ok())
}
