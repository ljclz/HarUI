//! Text 文本 — 参考 Element Plus `<el-text>`。
//!
//! 覆盖：type、size、truncated、tag、copyable。

use har_ui_components::text::{Text, TextSize, TextTag, TextType};

#[test]
fn test_text_default() {
    let t = Text::new();
    assert_eq!(t.text_type(), TextType::Default);
    assert_eq!(t.size(), TextSize::Default);
    assert!(!t.truncated());
    assert_eq!(t.tag(), TextTag::Span);
    assert!(!t.copyable());
}

#[test]
fn test_text_with_content() {
    let t = Text::new().with_content("hello");
    assert_eq!(t.content(), "hello");
}

#[test]
fn test_text_types() {
    assert_eq!(
        Text::new().with_type(TextType::Primary).text_type(),
        TextType::Primary
    );
    assert_eq!(
        Text::new().with_type(TextType::Success).text_type(),
        TextType::Success
    );
    assert_eq!(
        Text::new().with_type(TextType::Warning).text_type(),
        TextType::Warning
    );
    assert_eq!(
        Text::new().with_type(TextType::Danger).text_type(),
        TextType::Danger
    );
    assert_eq!(
        Text::new().with_type(TextType::Info).text_type(),
        TextType::Info
    );
}

#[test]
fn test_text_sizes() {
    assert_eq!(
        Text::new().with_size(TextSize::Large).size(),
        TextSize::Large
    );
    assert_eq!(
        Text::new().with_size(TextSize::Small).size(),
        TextSize::Small
    );
}

#[test]
fn test_text_truncated() {
    let t = Text::new().with_truncated(true);
    assert!(t.truncated());
}

#[test]
fn test_text_tag_div() {
    let t = Text::new().with_tag(TextTag::Div);
    assert_eq!(t.tag(), TextTag::Div);
}

#[test]
fn test_text_tag_p() {
    let t = Text::new().with_tag(TextTag::P);
    assert_eq!(t.tag(), TextTag::P);
}

#[test]
fn test_text_copyable() {
    let t = Text::new().with_copyable(true);
    assert!(t.copyable());
}

#[test]
fn test_text_ellipsis_lines() {
    let t = Text::new().with_max_lines(2);
    assert_eq!(t.max_lines(), Some(2));
}

#[test]
fn test_text_safe_default_content() {
    let t = Text::new();
    assert_eq!(t.content(), "");
}
