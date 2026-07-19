//! Space 间距 — 参考 Element Plus `<el-space>`。
//!
//! 覆盖：方向、size、wrap、fill、alignment、items 增删。

use har_ui_components::space::{Space, SpaceAlignment, SpaceDirection, SpaceMessage};

#[test]
fn test_space_default() {
    let s = Space::new();
    assert_eq!(s.direction(), SpaceDirection::Horizontal);
    assert_eq!(s.size(), 8);
    assert!(!s.wrap());
    assert!(!s.fill());
    assert_eq!(s.alignment(), SpaceAlignment::Center);
    assert!(s.items().is_empty());
}

#[test]
fn test_space_with_direction() {
    let s = Space::new().with_direction(SpaceDirection::Vertical);
    assert_eq!(s.direction(), SpaceDirection::Vertical);
}

#[test]
fn test_space_with_size() {
    let s = Space::new().with_size(20);
    assert_eq!(s.size(), 20);
}

#[test]
fn test_space_with_wrap() {
    let s = Space::new().with_wrap(true);
    assert!(s.wrap());
}

#[test]
fn test_space_with_fill() {
    let s = Space::new().with_fill(true);
    assert!(s.fill());
}

#[test]
fn test_space_with_alignment() {
    let s = Space::new().with_alignment(SpaceAlignment::Start);
    assert_eq!(s.alignment(), SpaceAlignment::Start);
}

#[test]
fn test_space_add_items() {
    let mut s = Space::new();
    s.handle(SpaceMessage::AddItem("A".into()));
    s.handle(SpaceMessage::AddItem("B".into()));
    s.handle(SpaceMessage::AddItem("C".into()));
    assert_eq!(s.items().len(), 3);
    assert_eq!(s.items()[0], "A");
    assert_eq!(s.items()[2], "C");
}

#[test]
fn test_space_remove_item() {
    let mut s = Space::new();
    s.handle(SpaceMessage::AddItem("A".into()));
    s.handle(SpaceMessage::AddItem("B".into()));
    s.handle(SpaceMessage::AddItem("C".into()));
    s.handle(SpaceMessage::RemoveItem("B".into()));
    assert_eq!(s.items().len(), 2);
    assert_eq!(s.items()[0], "A");
    assert_eq!(s.items()[1], "C");
}

#[test]
fn test_space_clear() {
    let mut s = Space::new();
    s.handle(SpaceMessage::AddItem("A".into()));
    s.handle(SpaceMessage::AddItem("B".into()));
    s.handle(SpaceMessage::Clear);
    assert!(s.items().is_empty());
}

#[test]
fn test_space_remove_nonexistent_noop() {
    let mut s = Space::new();
    s.handle(SpaceMessage::AddItem("A".into()));
    s.handle(SpaceMessage::RemoveItem("X".into()));
    assert_eq!(s.items().len(), 1);
}

#[test]
fn test_space_remove_duplicate_removes_first() {
    let mut s = Space::new();
    s.handle(SpaceMessage::AddItem("A".into()));
    s.handle(SpaceMessage::AddItem("A".into()));
    s.handle(SpaceMessage::AddItem("A".into()));
    s.handle(SpaceMessage::RemoveItem("A".into()));
    assert_eq!(s.items().len(), 2);
}
