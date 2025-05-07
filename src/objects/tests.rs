use crate::objects::{
    block::Block,
    comment::Comment,
    emoji::Emoji,
    error::Error,
    file::File,
    icon::{Icon, IconWithDefault},
    page::Page,
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

#[test]
fn test_custom_emoji_icon() {
    // Test with the untagged Icon
    let result = serde_json::from_str::<Icon>(include_str!("tests/custom_emoji.json"));
    assert!(result.is_ok());

    if let Ok(Icon::CustomEmoji { custom_emoji }) = result {
        assert_eq!(custom_emoji.id, "1ba280e0-4829-8053-aa07-007a2ce9c564");
        assert_eq!(custom_emoji.name, "my-custom-emoji");
        assert_eq!(custom_emoji.url, "https://example.com/my-custom-emoji.png");
    } else {
        panic!("Expected Icon::CustomEmoji variant, got something else");
    }

    // Test with the tagged IconWithDefault
    let result = serde_json::from_str::<IconWithDefault>(include_str!("tests/custom_emoji.json"));
    assert!(result.is_ok());

    if let Ok(IconWithDefault::CustomEmoji { custom_emoji }) = result {
        assert_eq!(custom_emoji.id, "1ba280e0-4829-8053-aa07-007a2ce9c564");
        assert_eq!(custom_emoji.name, "my-custom-emoji");
        assert_eq!(custom_emoji.url, "https://example.com/my-custom-emoji.png");
    } else {
        panic!("Expected IconWithDefault::CustomEmoji variant, got something else");
    }
}

#[test]
fn test_external_icon() {
    // Test with the untagged Icon
    let result = serde_json::from_str::<Icon>(include_str!("tests/external_icon.json"));
    assert!(result.is_ok());

    if let Ok(Icon::File(File::External { external })) = result {
        assert_eq!(
            external.url,
            "https://www.notion.so/icons/log-out_yellow.svg"
        );
    } else {
        panic!("Expected Icon::File(File::External) variant, got something else");
    }

    // Test with the tagged IconWithDefault
    let result = serde_json::from_str::<IconWithDefault>(include_str!("tests/external_icon.json"));
    assert!(result.is_ok());

    if let Ok(IconWithDefault::External { external }) = result {
        assert_eq!(
            external.url,
            "https://www.notion.so/icons/log-out_yellow.svg"
        );
    } else {
        panic!("Expected IconWithDefault::External variant, got something else");
    }
}
