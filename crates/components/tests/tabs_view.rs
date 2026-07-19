//! R.2.P1.12 Tabs view() 测试 — TDD RED 阶段

use har_ui_components::tabs::{TabItem, Tabs, TabsMessage, TabsType};
use har_ui_core::theme::Theme;

#[test]
fn test_tabs_view_empty_renders() {
    let theme = Theme::element_light();
    let t = Tabs::new();
    let _element = t.view(&theme, |_| ());
}

#[test]
fn test_tabs_view_default_renders() {
    let theme = Theme::element_light();
    let t = Tabs::new()
        .with_item(TabItem::new("t1", "Tab 1"))
        .with_item(TabItem::new("t2", "Tab 2"));
    let _element = t.view(&theme, |_| ());
}

#[test]
fn test_tabs_view_card_type_renders() {
    let theme = Theme::element_light();
    let t = Tabs::new()
        .with_type(TabsType::Card)
        .with_item(TabItem::new("t1", "Tab 1"))
        .with_item(TabItem::new("t2", "Tab 2"));
    let _element = t.view(&theme, |_| ());
}

#[test]
fn test_tabs_view_border_card_type_renders() {
    let theme = Theme::element_light();
    let t = Tabs::new()
        .with_type(TabsType::BorderCard)
        .with_item(TabItem::new("t1", "Tab 1"));
    let _element = t.view(&theme, |_| ());
}

#[test]
fn test_tabs_view_active_highlighted_renders() {
    let theme = Theme::element_light();
    let mut t = Tabs::new()
        .with_item(TabItem::new("t1", "Tab 1"))
        .with_item(TabItem::new("t2", "Tab 2"));
    t.handle(TabsMessage::Select("t2".to_string()));
    let _element = t.view(&theme, |_| ());
}

#[test]
fn test_tabs_view_disabled_item_renders() {
    let theme = Theme::element_light();
    let t = Tabs::new()
        .with_item(TabItem::new("t1", "Tab 1"))
        .with_item(TabItem::new("t2", "Tab 2").disabled(true));
    let _element = t.view(&theme, |_| ());
}

#[test]
fn test_tabs_view_closable_item_renders() {
    let theme = Theme::element_light();
    let t = Tabs::new()
        .with_closable(true)
        .with_item(TabItem::new("t1", "Tab 1").closable(true))
        .with_item(TabItem::new("t2", "Tab 2").closable(true));
    let _element = t.view(&theme, |_| ());
}

#[test]
fn test_tabs_view_addable_renders() {
    let theme = Theme::element_light();
    let t = Tabs::new()
        .with_addable(true)
        .with_item(TabItem::new("t1", "Tab 1"));
    let _element = t.view(&theme, |_| ());
}

#[test]
fn test_tabs_view_lazy_renders() {
    let theme = Theme::element_light();
    let t = Tabs::new()
        .with_lazy(true)
        .with_item(TabItem::new("t1", "Tab 1"))
        .with_item(TabItem::new("t2", "Tab 2"));
    let _element = t.view(&theme, |_| ());
}

#[test]
fn test_tabs_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let t = Tabs::new()
        .with_item(TabItem::new("t1", "Tab 1"))
        .with_item(TabItem::new("t2", "Tab 2"));
    let _element = t.view(&theme, |_| ());
}

#[test]
fn test_tabs_view_custom_message_type() {
    let theme = Theme::element_light();
    let t = Tabs::new().with_item(TabItem::new("t1", "Tab 1"));
    #[derive(Clone, Debug)]
    #[allow(dead_code)]
    enum AppMsg {
        TabSelect(String),
    }
    let _element = t.view(&theme, |id| AppMsg::TabSelect(id));
}
