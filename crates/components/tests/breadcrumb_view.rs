//! R.2.P1.14 Breadcrumb view() 测试 — TDD RED 阶段

use har_ui_components::breadcrumb::{Breadcrumb, BreadcrumbItem, BreadcrumbMessage};
use har_ui_core::theme::Theme;

#[test]
fn test_breadcrumb_view_empty_renders() {
    let theme = Theme::element_light();
    let b = Breadcrumb::new();
    let _element = b.view(&theme, |_| ());
}

#[test]
fn test_breadcrumb_view_default_separator_renders() {
    let theme = Theme::element_light();
    let b = Breadcrumb::new()
        .with_item(BreadcrumbItem::new("Home"))
        .with_item(BreadcrumbItem::new("List"))
        .with_item(BreadcrumbItem::new("Detail"));
    let _element = b.view(&theme, |_| ());
}

#[test]
fn test_breadcrumb_view_custom_separator_renders() {
    let theme = Theme::element_light();
    let b = Breadcrumb::new()
        .with_separator(">")
        .with_item(BreadcrumbItem::new("Home"))
        .with_item(BreadcrumbItem::new("Detail"));
    let _element = b.view(&theme, |_| ());
}

#[test]
fn test_breadcrumb_view_with_to_renders() {
    let theme = Theme::element_light();
    let b = Breadcrumb::new()
        .with_item(BreadcrumbItem::new("Home").with_to("/home"))
        .with_item(BreadcrumbItem::new("List").with_to("/list"))
        .with_item(BreadcrumbItem::new("Detail"));
    let _element = b.view(&theme, |_| ());
}

#[test]
fn test_breadcrumb_view_with_icon_renders() {
    let theme = Theme::element_light();
    let b = Breadcrumb::new()
        .with_item(BreadcrumbItem::new("Home").with_icon("🏠"))
        .with_item(BreadcrumbItem::new("Detail"));
    let _element = b.view(&theme, |_| ());
}

#[test]
fn test_breadcrumb_view_single_item_renders() {
    let theme = Theme::element_light();
    let b = Breadcrumb::new().with_item(BreadcrumbItem::new("Only"));
    let _element = b.view(&theme, |_| ());
}

#[test]
fn test_breadcrumb_view_clicked_state_renders() {
    let theme = Theme::element_light();
    let mut b = Breadcrumb::new()
        .with_item(BreadcrumbItem::new("Home").with_to("/home"))
        .with_item(BreadcrumbItem::new("Detail"));
    b.handle(BreadcrumbMessage::Click(0));
    let _element = b.view(&theme, |_| ());
}

#[test]
fn test_breadcrumb_view_replace_last_renders() {
    let theme = Theme::element_light();
    let mut b = Breadcrumb::new()
        .with_item(BreadcrumbItem::new("Home"))
        .with_item(BreadcrumbItem::new("Old"));
    b.handle(BreadcrumbMessage::Replace(BreadcrumbItem::new("New")));
    let _element = b.view(&theme, |_| ());
}

#[test]
fn test_breadcrumb_view_clear_click_renders() {
    let theme = Theme::element_light();
    let mut b = Breadcrumb::new()
        .with_item(BreadcrumbItem::new("Home").with_to("/home"))
        .with_item(BreadcrumbItem::new("Detail"));
    b.handle(BreadcrumbMessage::Click(0));
    b.handle(BreadcrumbMessage::ClearClick);
    let _element = b.view(&theme, |_| ());
}

#[test]
fn test_breadcrumb_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let b = Breadcrumb::new()
        .with_item(BreadcrumbItem::new("Home").with_to("/home"))
        .with_item(BreadcrumbItem::new("Detail"));
    let _element = b.view(&theme, |_| ());
}

#[test]
fn test_breadcrumb_view_custom_message_type() {
    let theme = Theme::element_light();
    let b = Breadcrumb::new()
        .with_item(BreadcrumbItem::new("Home").with_to("/home"))
        .with_item(BreadcrumbItem::new("Detail"));
    #[derive(Clone, Debug)]
    #[allow(dead_code)]
    enum AppMsg {
        Navigate(String),
    }
    let _element = b.view(&theme, |to| AppMsg::Navigate(to));
}
