//! PageHeader 页头 — 参考 Element Plus `<el-page-header>`。
//!
//! 覆盖：标题/副标题/icon、返回按钮事件、额外内容 slot。

use har_ui_components::page_header::{PageHeader, PageHeaderMessage};

#[test]
fn test_page_header_default() {
    let p = PageHeader::new();
    assert_eq!(p.title(), "返回");
    assert_eq!(p.content(), None);
    assert_eq!(p.icon(), "arrow-left");
}

#[test]
fn test_page_header_custom_title() {
    let p = PageHeader::new().with_title("商品详情");
    assert_eq!(p.title(), "商品详情");
}

#[test]
fn test_page_header_custom_content() {
    let p = PageHeader::new().with_content("商品 ID: G001");
    assert_eq!(p.content(), Some("商品 ID: G001"));
}

#[test]
fn test_page_header_custom_icon() {
    let p = PageHeader::new().with_icon("back");
    assert_eq!(p.icon(), "back");
}

#[test]
fn test_page_header_back_event() {
    let mut p = PageHeader::new();
    assert!(!p.back_clicked());
    p.handle(PageHeaderMessage::Back);
    assert!(p.back_clicked());
    p.handle(PageHeaderMessage::ResetBack);
    assert!(!p.back_clicked());
}

#[test]
fn test_page_header_extra_slot() {
    let p = PageHeader::new().with_has_extra(true);
    assert!(p.has_extra());
}

#[test]
fn test_page_header_subtitle() {
    let p = PageHeader::new().with_subtitle("最近编辑：张三");
    assert_eq!(p.subtitle(), Some("最近编辑：张三"));
}
