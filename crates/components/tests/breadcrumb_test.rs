//! Breadcrumb 面包屑 — 参考 Element Plus `<el-breadcrumb>`。
//!
//! 覆盖：基础渲染、分隔符、to 跳转、点击事件、icon。

use har_ui_components::breadcrumb::{Breadcrumb, BreadcrumbItem, BreadcrumbMessage};

#[test]
fn test_breadcrumb_empty() {
    let b = Breadcrumb::new();
    assert!(b.items().is_empty());
    assert_eq!(b.separator(), "/");
}

#[test]
fn test_breadcrumb_custom_separator() {
    let b = Breadcrumb::new().with_separator(">");
    assert_eq!(b.separator(), ">");
}

#[test]
fn test_breadcrumb_with_items() {
    let b = Breadcrumb::new()
        .with_item(BreadcrumbItem::new("首页").with_to("/"))
        .with_item(BreadcrumbItem::new("商品管理").with_to("/goods"))
        .with_item(BreadcrumbItem::new("商品列表")); // 最后一项无 to
    assert_eq!(b.items().len(), 3);
    assert_eq!(b.items()[0].text(), "首页");
    assert_eq!(b.items()[0].to(), Some("/"));
    assert_eq!(b.items()[2].to(), None);
}

#[test]
fn test_breadcrumb_click_event() {
    let mut b = Breadcrumb::new()
        .with_item(BreadcrumbItem::new("首页").with_to("/home"))
        .with_item(BreadcrumbItem::new("列表").with_to("/list"))
        .with_item(BreadcrumbItem::new("详情")); // 当前页不可点

    // 点击"首页"
    b.handle(BreadcrumbMessage::Click(0));
    assert_eq!(b.last_clicked(), Some(0));
    assert_eq!(b.navigate_target(), Some("/home"));

    // 点击最后一项（无 to）→ 不导航
    b.handle(BreadcrumbMessage::Click(2));
    assert_eq!(b.last_clicked(), Some(2));
    assert_eq!(b.navigate_target(), None);
}

#[test]
fn test_breadcrumb_item_with_icon() {
    let item = BreadcrumbItem::new("首页").with_icon("home");
    assert_eq!(item.icon(), Some("home"));
}

#[test]
fn test_breadcrumb_replace_last_item() {
    let mut b = Breadcrumb::new()
        .with_item(BreadcrumbItem::new("a"))
        .with_item(BreadcrumbItem::new("b"));
    b.handle(BreadcrumbMessage::Replace(BreadcrumbItem::new("c")));
    assert_eq!(b.items().len(), 2);
    assert_eq!(b.items()[1].text(), "c");
}

#[test]
fn test_breadcrumb_clear_click_state() {
    let mut b = Breadcrumb::new().with_item(BreadcrumbItem::new("a").with_to("/a"));
    b.handle(BreadcrumbMessage::Click(0));
    assert!(b.last_clicked().is_some());
    b.handle(BreadcrumbMessage::ClearClick);
    assert!(b.last_clicked().is_none());
    assert!(b.navigate_target().is_none());
}
