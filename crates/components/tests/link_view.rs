//! Link view() 测试

use har_ui_components::link::{Link, LinkType};
use har_ui_core::theme::Theme;

#[derive(Clone, Debug)]
enum AppMsg {
    Click,
}

#[test]
fn test_link_view_default_renders() {
    let theme = Theme::element_light();
    let l = Link::new().with_text("链接");
    let _element = l.view(&theme, ());
}

#[test]
fn test_link_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let l = Link::new().with_text("链接");
    let _element = l.view(&theme, ());
}

#[test]
fn test_link_view_primary_type_renders() {
    let theme = Theme::element_light();
    let l = Link::new()
        .with_text("Primary")
        .with_type(LinkType::Primary);
    let _element = l.view(&theme, ());
}

#[test]
fn test_link_view_success_type_renders() {
    let theme = Theme::element_light();
    let l = Link::new()
        .with_text("Success")
        .with_type(LinkType::Success);
    let _element = l.view(&theme, ());
}

#[test]
fn test_link_view_danger_type_renders() {
    let theme = Theme::element_light();
    let l = Link::new().with_text("Danger").with_type(LinkType::Danger);
    let _element = l.view(&theme, ());
}

#[test]
fn test_link_view_disabled_renders() {
    let theme = Theme::element_light();
    let l = Link::new().with_text("Disabled").with_disabled(true);
    let _element = l.view(&theme, ());
}

#[test]
fn test_link_view_underline_false_renders() {
    let theme = Theme::element_light();
    let l = Link::new().with_text("No underline").with_underline(false);
    let _element = l.view(&theme, ());
}

#[test]
fn test_link_view_with_icon_renders() {
    let theme = Theme::element_light();
    let l = Link::new().with_text("With icon").with_icon("🔗");
    let _element = l.view(&theme, ());
}

#[test]
fn test_link_view_with_href_renders() {
    let theme = Theme::element_light();
    let l = Link::new()
        .with_text("Href")
        .with_href("https://example.com");
    let _element = l.view(&theme, ());
}

#[test]
fn test_link_view_custom_message_renders() {
    let theme = Theme::element_light();
    let l = Link::new().with_text("Custom msg");
    let _element = l.view(&theme, AppMsg::Click);
}

#[test]
fn test_link_view_disabled_with_underline_renders() {
    let theme = Theme::element_light();
    let l = Link::new()
        .with_text("Disabled underline")
        .with_disabled(true)
        .with_underline(true);
    let _element = l.view(&theme, ());
}

#[test]
fn test_link_view_info_warning_types_renders() {
    let theme = Theme::element_light();
    let l1 = Link::new().with_text("Info").with_type(LinkType::Info);
    let l2 = Link::new()
        .with_text("Warning")
        .with_type(LinkType::Warning);
    let _e1 = l1.view(&theme, ());
    let _e2 = l2.view(&theme, ());
}
