//! Collapse 折叠面板 — 参考 Element Plus `<el-collapse>`。
//!
//! 覆盖：accordion 手风琴、active_keys、disabled item、Toggle/Open/Close。

use har_ui_components::collapse::{Collapse, CollapseItem, CollapseMessage};

#[test]
fn test_collapse_default() {
    let c = Collapse::new();
    assert!(!c.accordion());
    assert!(c.items().is_empty());
    assert!(c.active_keys().is_empty());
}

#[test]
fn test_collapse_with_accordion() {
    let c = Collapse::new().with_accordion(true);
    assert!(c.accordion());
}

#[test]
fn test_collapse_add_item() {
    let mut c = Collapse::new();
    c.add_item(CollapseItem::new("i1", "标题一"));
    c.add_item(CollapseItem::new("i2", "标题二").with_disabled(true));
    assert_eq!(c.items().len(), 2);
    assert_eq!(c.items()[0].name(), "i1");
    assert_eq!(c.items()[0].title(), "标题一");
    assert!(!c.items()[0].disabled());
    assert!(c.items()[1].disabled());
}

#[test]
fn test_collapse_toggle() {
    let mut c = Collapse::new();
    c.add_item(CollapseItem::new("i1", "标题一"));
    c.handle(CollapseMessage::Toggle("i1".into()));
    assert!(c.is_active("i1"));
    c.handle(CollapseMessage::Toggle("i1".into()));
    assert!(!c.is_active("i1"));
}

#[test]
fn test_collapse_toggle_disabled() {
    let mut c = Collapse::new();
    c.add_item(CollapseItem::new("i1", "标题一").with_disabled(true));
    c.handle(CollapseMessage::Toggle("i1".into()));
    // disabled 项不能被切换
    assert!(!c.is_active("i1"));
}

#[test]
fn test_collapse_open_close() {
    let mut c = Collapse::new();
    c.add_item(CollapseItem::new("i1", "标题一"));
    c.handle(CollapseMessage::Open("i1".into()));
    assert!(c.is_active("i1"));
    c.handle(CollapseMessage::Close("i1".into()));
    assert!(!c.is_active("i1"));
}

#[test]
fn test_collapse_accordion_only_one_open() {
    let mut c = Collapse::new().with_accordion(true);
    c.add_item(CollapseItem::new("i1", "一"));
    c.add_item(CollapseItem::new("i2", "二"));
    c.add_item(CollapseItem::new("i3", "三"));
    c.handle(CollapseMessage::Open("i1".into()));
    c.handle(CollapseMessage::Open("i2".into()));
    // 手风琴：只有 i2 应保持打开
    assert!(!c.is_active("i1"));
    assert!(c.is_active("i2"));
    assert!(!c.is_active("i3"));
}

#[test]
fn test_collapse_open_all_non_accordion() {
    let mut c = Collapse::new();
    c.add_item(CollapseItem::new("i1", "一"));
    c.add_item(CollapseItem::new("i2", "二"));
    c.handle(CollapseMessage::OpenAll);
    assert!(c.is_active("i1"));
    assert!(c.is_active("i2"));
}

#[test]
fn test_collapse_close_all() {
    let mut c = Collapse::new();
    c.add_item(CollapseItem::new("i1", "一"));
    c.add_item(CollapseItem::new("i2", "二"));
    c.handle(CollapseMessage::OpenAll);
    c.handle(CollapseMessage::CloseAll);
    assert!(c.active_keys().is_empty());
}

#[test]
fn test_collapse_open_all_accordion_ignores() {
    let mut c = Collapse::new().with_accordion(true);
    c.add_item(CollapseItem::new("i1", "一"));
    c.add_item(CollapseItem::new("i2", "二"));
    // 手风琴模式下 OpenAll 应被忽略（保持全部折叠）
    c.handle(CollapseMessage::OpenAll);
    assert!(c.active_keys().is_empty());
}

#[test]
fn test_collapse_toggle_nonexistent_noop() {
    let mut c = Collapse::new();
    c.add_item(CollapseItem::new("i1", "一"));
    c.handle(CollapseMessage::Toggle("not-exist".into()));
    assert!(!c.is_active("not-exist"));
    assert!(c.active_keys().is_empty());
}

#[test]
fn test_collapse_with_default_active() {
    let mut c = Collapse::new();
    c.add_item(CollapseItem::new("i1", "一").with_default_active(true));
    c.add_item(CollapseItem::new("i2", "二"));
    // 构造完成时已应用初始 active
    assert!(c.is_active("i1"));
    assert!(!c.is_active("i2"));
}
