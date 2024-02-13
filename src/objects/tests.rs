use crate::objects::{
    block::Block, comment::Comment, emoji::Emoji, error::Error, file::File, page::Page,
    rich_text::RichText,
};

#[test]
fn test_error() {
    let result = serde_json::from_str::<Error>(include_str!("tests/error.json"));
    assert!(result.is_ok())
}

#[test]
fn test_block() {
    let result = serde_json::from_str::<Block>(include_str!("tests/block.json"));
    assert!(result.is_ok())
}

#[test]
fn test_rich_text() {
    let result = serde_json::from_str::<RichText>(include_str!("tests/rich_text.json"));
    assert!(result.is_ok())
}

#[test]
fn test_page() {
    let result = serde_json::from_str::<Page>(include_str!("tests/page.json"));
    assert!(result.is_ok())
}

#[test]
fn test_comment() {
    let result = serde_json::from_str::<Comment>(include_str!("tests/comment.json"));
    assert!(result.is_ok())
}

#[test]
fn test_file() {
    let result = serde_json::from_str::<File>(include_str!("tests/file.json"));
    assert!(result.is_ok())
}

#[test]
fn test_external_file() {
    let result = serde_json::from_str::<File>(include_str!("tests/external_file.json"));
    assert!(result.is_ok())
}

#[test]
fn test_emoji() {
    let result = serde_json::from_str::<Emoji>(include_str!("tests/emoji.json"));
    assert!(result.is_ok())
}
