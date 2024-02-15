use crate::{
    endpoints::comments::{
        create::request::CreateCommentRequest, retrieve::response::RetrieveCommentsResponse,
    },
    objects::{
        comment::Comment,
        parent::Parent,
        rich_text::{RichText, Text},
    },
};

#[test]
fn test_create_comment_request() {
    let request = CreateCommentRequest {
        parent: Some(Parent::PageId {
            page_id: "5c6a28216bb14a7eb6e1c50111515c3d".to_string(),
        }),
        rich_text: vec![RichText::Text {
            text: Text {
                content: "Hello world".to_string(),
                link: None,
            },
            annotations: None,
            plain_text: None,
            href: None,
        }],
        ..Default::default()
    };

    let result = serde_json::to_string_pretty(&request).unwrap();
    let actual = include_str!("tests/create_comment_request.json");
    assert_eq!(result, actual.to_string())
}

#[test]
fn test_create_comment_200() {
    let result = serde_json::from_str::<Comment>(include_str!("tests/create_comment_200.json"));
    assert!(result.is_ok())
}

#[test]
fn test_retrive_comments_200() {
    let result = serde_json::from_str::<RetrieveCommentsResponse>(include_str!(
        "tests/retrieve_comments_200.json"
    ));
    assert!(result.is_ok())
}
