//! R.2.P1.15 Dropdown view() 测试 — TDD RED 阶段

use har_ui_components::dropdown::{Dropdown, DropdownItem, DropdownMessage, DropdownTrigger};
use har_ui_core::theme::Theme;

#[test]
fn test_dropdown_view_empty_renders() {
    let theme = Theme::element_light();
    let d = Dropdown::new();
    let _element = d.view(&theme, || ());
}

#[test]
fn test_dropdown_view_with_items_renders() {
    let theme = Theme::element_light();
    let d = Dropdown::new()
        .with_item(DropdownItem::new("cmd1", "Action 1"))
        .with_item(DropdownItem::new("cmd2", "Action 2"));
    let _element = d.view(&theme, || ());
}

#[test]
fn test_dropdown_view_visible_renders() {
    let theme = Theme::element_light();
    let mut d = Dropdown::new()
        .with_item(DropdownItem::new("cmd1", "Action 1"))
        .with_item(DropdownItem::new("cmd2", "Action 2"));
    d.handle(DropdownMessage::Show);
    let _element = d.view(&theme, || ());
}

#[test]
fn test_dropdown_view_click_trigger_renders() {
    let theme = Theme::element_light();
    let d = Dropdown::new()
        .with_trigger(DropdownTrigger::Click)
        .with_item(DropdownItem::new("cmd1", "Action 1"));
    let _element = d.view(&theme, || ());
}

#[test]
fn test_dropdown_view_contextmenu_trigger_renders() {
    let theme = Theme::element_light();
    let d = Dropdown::new()
        .with_trigger(DropdownTrigger::ContextMenu)
        .with_item(DropdownItem::new("cmd1", "Action 1"));
    let _element = d.view(&theme, || ());
}

#[test]
fn test_dropdown_view_hover_trigger_renders() {
    let theme = Theme::element_light();
    let d = Dropdown::new()
        .with_trigger(DropdownTrigger::Hover)
        .with_item(DropdownItem::new("cmd1", "Action 1"));
    let _element = d.view(&theme, || ());
}

#[test]
fn test_dropdown_view_disabled_item_renders() {
    let theme = Theme::element_light();
    let d = Dropdown::new()
        .with_item(DropdownItem::new("cmd1", "Action 1"))
        .with_item(DropdownItem::new("cmd2", "Action 2").with_disabled(true));
    let _element = d.view(&theme, || ());
}

#[test]
fn test_dropdown_view_divided_item_renders() {
    let theme = Theme::element_light();
    let mut d = Dropdown::new()
        .with_item(DropdownItem::new("cmd1", "Action 1"))
        .with_item(DropdownItem::new("cmd2", "Action 2").with_divided(true));
    d.handle(DropdownMessage::Show);
    let _element = d.view(&theme, || ());
}

#[test]
fn test_dropdown_view_last_command_renders() {
    let theme = Theme::element_light();
    let mut d = Dropdown::new()
        .with_item(DropdownItem::new("cmd1", "Action 1"))
        .with_item(DropdownItem::new("cmd2", "Action 2"));
    d.handle(DropdownMessage::Show);
    d.handle(DropdownMessage::Select(0));
    let _element = d.view(&theme, || ());
}

#[test]
fn test_dropdown_view_hide_on_click_false_renders() {
    let theme = Theme::element_light();
    let d = Dropdown::new()
        .with_hide_on_click(false)
        .with_item(DropdownItem::new("cmd1", "Action 1"));
    let _element = d.view(&theme, || ());
}

#[test]
fn test_dropdown_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let d = Dropdown::new()
        .with_item(DropdownItem::new("cmd1", "Action 1"))
        .with_item(DropdownItem::new("cmd2", "Action 2"));
    let _element = d.view(&theme, || ());
}

#[test]
fn test_dropdown_view_custom_message_type() {
    let theme = Theme::element_light();
    let d = Dropdown::new()
        .with_trigger(DropdownTrigger::Click)
        .with_item(DropdownItem::new("cmd1", "Action 1"));
    #[derive(Clone, Debug)]
    #[allow(dead_code)]
    enum AppMsg {
        Trigger,
    }
    let _element = d.view(&theme, || AppMsg::Trigger);
}
