//! R.2.P1.11 Collapse view() 测试 — TDD RED 阶段

use har_ui_components::collapse::{Collapse, CollapseItem, CollapseMessage};
use har_ui_core::theme::Theme;

#[test]
fn test_collapse_view_empty_renders() {
    let theme = Theme::element_light();
    let c = Collapse::new();
    let _element = c.view(&theme, |_| ());
}

#[test]
fn test_collapse_view_with_items_renders() {
    let theme = Theme::element_light();
    let mut c = Collapse::new();
    c.add_item(CollapseItem::new("a", "Section A"));
    c.add_item(CollapseItem::new("b", "Section B"));
    let _element = c.view(&theme, |_| ());
}

#[test]
fn test_collapse_view_active_item_renders() {
    let theme = Theme::element_light();
    let mut c = Collapse::new();
    c.add_item(CollapseItem::new("a", "Section A"));
    c.handle(CollapseMessage::Open("a".to_string()));
    let _element = c.view(&theme, |_| ());
}

#[test]
fn test_collapse_view_disabled_item_renders() {
    let theme = Theme::element_light();
    let mut c = Collapse::new();
    c.add_item(CollapseItem::new("a", "Section A").with_disabled(true));
    let _element = c.view(&theme, |_| ());
}

#[test]
fn test_collapse_view_accordion_renders() {
    let theme = Theme::element_light();
    let mut c = Collapse::new().with_accordion(true);
    c.add_item(CollapseItem::new("a", "A"));
    c.add_item(CollapseItem::new("b", "B"));
    c.handle(CollapseMessage::Open("a".to_string()));
    let _element = c.view(&theme, |_| ());
}

#[test]
fn test_collapse_view_default_active_renders() {
    let theme = Theme::element_light();
    let mut c = Collapse::new();
    c.add_item(CollapseItem::new("a", "A").with_default_active(true));
    let _element = c.view(&theme, |_| ());
}

#[test]
fn test_collapse_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let mut c = Collapse::new();
    c.add_item(CollapseItem::new("a", "A"));
    let _element = c.view(&theme, |_| ());
}
