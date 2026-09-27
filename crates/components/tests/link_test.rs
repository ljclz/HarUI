//! Link 文字链接 — 参考 Element Plus `<el-link>`。
//!
//! 覆盖：type、underline、disabled、href、icon、点击事件。

use har_ui_components::link::{Link, LinkMessage, LinkType};

#[test]
fn test_link_default() {
    let l = Link::new();
    assert_eq!(l.link_type(), LinkType::Default);
    assert!(l.underline());
    assert!(!l.disabled());
    assert_eq!(l.text(), "");
    assert_eq!(l.href(), None);
}

#[test]
fn test_link_with_text() {
    let l = Link::new().with_text("百度");
    assert_eq!(l.text(), "百度");
}

#[test]
fn test_link_with_href() {
    let l = Link::new().with_href("https://baidu.com");
    assert_eq!(l.href(), Some("https://baidu.com"));
}

#[test]
fn test_link_types() {
    assert_eq!(
        Link::new().with_type(LinkType::Primary).link_type(),
        LinkType::Primary
    );
    assert_eq!(
        Link::new().with_type(LinkType::Success).link_type(),
        LinkType::Success
    );
    assert_eq!(
        Link::new().with_type(LinkType::Warning).link_type(),
        LinkType::Warning
    );
    assert_eq!(
        Link::new().with_type(LinkType::Danger).link_type(),
        LinkType::Danger
    );
    assert_eq!(
        Link::new().with_type(LinkType::Info).link_type(),
        LinkType::Info
    );
}

#[test]
fn test_link_no_underline() {
    let l = Link::new().with_underline(false);
    assert!(!l.underline());
}

#[test]
fn test_link_disabled() {
    let l = Link::new().with_disabled(true);
    assert!(l.disabled());
}

#[test]
fn test_link_with_icon() {
    let l = Link::new().with_icon("search");
    assert_eq!(l.icon(), Some("search"));
}

#[test]
fn test_link_click_event() {
    let mut l = Link::new().with_text("按钮");
    assert!(!l.clicked());
    l.handle(LinkMessage::Click);
    assert!(l.clicked());
    l.handle(LinkMessage::ResetClick);
    assert!(!l.clicked());
}

#[test]
fn test_link_disabled_blocks_click() {
    let mut l = Link::new().with_disabled(true);
    l.handle(LinkMessage::Click);
    assert!(!l.clicked());
}
