use crate::{
    endpoints::blocks::{
        append::{request::AppendBlockChildrenRequest, response::AppendBlockChildrenResponse},
        retrieve::response::RetrieveBlockChilerenResponse,
        update::request::UpdateABlockRequest,
    },
    objects::{
        block::{Block, BlockType, HeadingsValue, ParagraphValue, ToDoValue},
        rich_text::{Link, RichText, Text},
    },
};

#[test]
fn test_append_request() {
    let request = AppendBlockChildrenRequest {
        children: vec![
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
        ],
        ..Default::default()
    };

    let result = serde_json::to_string_pretty(&request).unwrap();
    let actual = include_str!("tests/append_request.json");
    assert_eq!(result, actual.to_string())
}

#[test]
fn test_update_request() {
    let request = UpdateABlockRequest {
        block: Some(Block {
            block_type: BlockType::ToDo {
                to_do: ToDoValue {
                    rich_text: vec![RichText::Text {
                        text: Text {
                            content: "Lacinato kale".to_string(),
                            link: None,
                        },
                        annotations: None,
                        plain_text: None,
                        href: None,
                    }],
                    checked: Some(false),
                    ..Default::default()
                },
            },
            ..Default::default()
        }),
        ..Default::default()
    };

    let result = serde_json::to_string_pretty(&request).unwrap();
    let actual = include_str!("tests/update_request.json");
    assert_eq!(result, actual.to_string())
}

#[test]
fn test_append_200() {
    let result =
        serde_json::from_str::<AppendBlockChildrenResponse>(include_str!("tests/append_200.json"));
    assert!(result.is_ok())
}

#[test]
fn test_retrieve_200() {
    let result = serde_json::from_str::<Block>(include_str!("tests/retrieve_200.json"));
    assert!(result.is_ok())
}

#[test]
fn test_retrieve_block_children_200() {
    let result = serde_json::from_str::<RetrieveBlockChilerenResponse>(include_str!(
        "tests/retrieve_block_children_200.json"
    ));
    assert!(result.is_ok())
}

#[test]
fn test_update_200() {
    let result = serde_json::from_str::<Block>(include_str!("tests/update_200.json"));
    assert!(result.is_ok())
}

#[test]
fn test_delete_200() {
    let result = serde_json::from_str::<Block>(include_str!("tests/delete_200.json"));
    assert!(result.is_ok())
}

#[test]
fn test_link_mention_mention_deserialization() {
    let result = serde_json::from_str::<Block>(include_str!(
        "tests/link_mention_mention_deserialization.json"
    ));
    assert!(result.is_ok())
}
