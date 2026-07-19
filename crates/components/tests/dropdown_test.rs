//! Dropdown 下拉菜单 — 参考 Element Plus `<el-dropdown>`。
//!
//! 覆盖：trigger（hover/click/contextmenu）、菜单项点击、disabled、placement、hide_on_click。

use har_ui_components::dropdown::{Dropdown, DropdownItem, DropdownMessage, DropdownTrigger};

#[test]
fn test_dropdown_default() {
    let d = Dropdown::new();
    assert_eq!(d.trigger(), DropdownTrigger::Hover);
    assert!(!d.visible());
    assert!(d.items().is_empty());
}

#[test]
fn test_dropdown_with_items() {
    let d = Dropdown::new()
        .with_item(DropdownItem::new("new", "新建"))
        .with_item(DropdownItem::new("edit", "编辑").with_disabled(true))
        .with_item(DropdownItem::new("del", "删除").with_divided(true));
    assert_eq!(d.items().len(), 3);
    assert_eq!(d.items()[0].command(), "new");
    assert_eq!(d.items()[0].label(), "新建");
    assert!(d.items()[1].disabled());
    assert!(d.items()[2].divided());
}

#[test]
fn test_dropdown_hover_trigger() {
    let mut d = Dropdown::new().with_trigger(DropdownTrigger::Hover);
    d.handle(DropdownMessage::MouseEnter);
    assert!(d.visible());
    d.handle(DropdownMessage::MouseLeave);
    assert!(!d.visible());
}

#[test]
fn test_dropdown_click_trigger() {
    let mut d = Dropdown::new().with_trigger(DropdownTrigger::Click);
    d.handle(DropdownMessage::MouseEnter); // click 模式 hover 不应触发
    assert!(!d.visible());
    d.handle(DropdownMessage::Click);
    assert!(d.visible());
    d.handle(DropdownMessage::ClickOutside);
    assert!(!d.visible());
}

#[test]
fn test_dropdown_contextmenu_trigger() {
    let mut d = Dropdown::new().with_trigger(DropdownTrigger::ContextMenu);
    d.handle(DropdownMessage::Click); // contextmenu 模式 click 不应触发
    assert!(!d.visible());
    d.handle(DropdownMessage::ContextMenu);
    assert!(d.visible());
}

#[test]
fn test_dropdown_select_item() {
    let mut d = Dropdown::new()
        .with_item(DropdownItem::new("a", "A"))
        .with_item(DropdownItem::new("b", "B"));
    d.handle(DropdownMessage::Select(1));
    assert_eq!(d.last_command(), Some("b"));
    // 默认 hide_on_click=true，选中后关闭
    assert!(!d.visible());
}

#[test]
fn test_dropdown_select_disabled_item_ignored() {
    let mut d = Dropdown::new()
        .with_item(DropdownItem::new("a", "A").with_disabled(true));
    d.handle(DropdownMessage::Select(0));
    assert_eq!(d.last_command(), None);
}

#[test]
fn test_dropdown_hide_on_click_false() {
    let mut d = Dropdown::new()
        .with_item(DropdownItem::new("a", "A"))
        .with_trigger(DropdownTrigger::Click)
        .with_hide_on_click(false);
    d.handle(DropdownMessage::Click);
    assert!(d.visible());
    d.handle(DropdownMessage::Select(0));
    assert_eq!(d.last_command(), Some("a"));
    assert!(d.visible()); // 不隐藏
}

#[test]
fn test_dropdown_manual_show_hide() {
    let mut d = Dropdown::new();
    d.handle(DropdownMessage::Show);
    assert!(d.visible());
    d.handle(DropdownMessage::Hide);
    assert!(!d.visible());
}

#[test]
fn test_dropdown_clear_last_command() {
    let mut d = Dropdown::new().with_item(DropdownItem::new("a", "A"));
    d.handle(DropdownMessage::Select(0));
    assert!(d.last_command().is_some());
    d.handle(DropdownMessage::ClearCommand);
    assert!(d.last_command().is_none());
}
