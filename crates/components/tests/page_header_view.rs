//! PageHeader view() 测试 — TDD RED 阶段

use har_ui_components::page_header::{PageHeader, PageHeaderMessage};
use har_ui_core::theme::Theme;

#[test]
fn test_page_header_view_default_renders() {
    let theme = Theme::element_light();
    let p = PageHeader::new();
    let _element = p.view(&theme, || ());
}

#[test]
fn test_page_header_view_with_title_renders() {
    let theme = Theme::element_light();
    let p = PageHeader::new().with_title("Detail Page");
    let _element = p.view(&theme, || ());
}

#[test]
fn test_page_header_view_with_subtitle_renders() {
    let theme = Theme::element_light();
    let p = PageHeader::new()
        .with_title("Title")
        .with_subtitle("Subtitle");
    let _element = p.view(&theme, || ());
}

#[test]
fn test_page_header_view_with_content_renders() {
    let theme = Theme::element_light();
    let p = PageHeader::new()
        .with_title("Title")
        .with_content("Body content");
    let _element = p.view(&theme, || ());
}

#[test]
fn test_page_header_view_with_extra_slot_renders() {
    let theme = Theme::element_light();
    let p = PageHeader::new().with_title("T").with_has_extra(true);
    let _element = p.view(&theme, || ());
}

#[test]
fn test_page_header_view_with_custom_icon_renders() {
    let theme = Theme::element_light();
    let p = PageHeader::new().with_title("T").with_icon("←");
    let _element = p.view(&theme, || ());
}

#[test]
fn test_page_header_view_after_back_clicked_renders() {
    let theme = Theme::element_light();
    let mut p = PageHeader::new().with_title("T");
    p.handle(PageHeaderMessage::Back);
    assert!(p.back_clicked());
    let _element = p.view(&theme, || ());
}

#[test]
fn test_page_header_view_after_reset_back_renders() {
    let theme = Theme::element_light();
    let mut p = PageHeader::new().with_title("T");
    p.handle(PageHeaderMessage::Back);
    p.handle(PageHeaderMessage::ResetBack);
    assert!(!p.back_clicked());
    let _element = p.view(&theme, || ());
}

#[test]
fn test_page_header_view_full_configuration_renders() {
    let theme = Theme::element_light();
    let p = PageHeader::new()
        .with_title("Title")
        .with_subtitle("Subtitle")
        .with_content("Content")
        .with_icon("arrow-left")
        .with_has_extra(true);
    let _element = p.view(&theme, || ());
}

#[test]
fn test_page_header_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let p = PageHeader::new()
        .with_title("Title")
        .with_subtitle("Sub")
        .with_content("Body");
    let _element = p.view(&theme, || ());
}
