//! R.2.P0.12 Card view() 测试 — TDD RED 阳段
//!
//! 验证 Card view() 正确渲染 header/body/footer。

use har_ui_components::card::{Card, CardShadow};
use har_ui_core::theme::Theme;

#[test]
fn test_card_view_body_only_renders() {
    let theme = Theme::element_light();
    let c = Card::new("Body content");
    let _element = c.view(&theme);
}

#[test]
fn test_card_view_with_header_renders() {
    let theme = Theme::element_light();
    let c = Card::new("Body").with_header("Header");
    let _element = c.view(&theme);
}

#[test]
fn test_card_view_with_footer_renders() {
    let theme = Theme::element_light();
    let c = Card::new("Body").with_footer("Footer");
    let _element = c.view(&theme);
}

#[test]
fn test_card_view_with_header_footer_renders() {
    let theme = Theme::element_light();
    let c = Card::new("Body").with_header("H").with_footer("F");
    let _element = c.view(&theme);
}

#[test]
fn test_card_view_with_image_renders() {
    let theme = Theme::element_light();
    let c = Card::new("Body").with_image("/img.png");
    let _element = c.view(&theme);
}

#[test]
fn test_card_view_shadow_always_renders() {
    let theme = Theme::element_light();
    let c = Card::new("Body").with_shadow(CardShadow::Always);
    let _element = c.view(&theme);
}

#[test]
fn test_card_view_shadow_hover_renders() {
    let theme = Theme::element_light();
    let c = Card::new("Body").with_shadow(CardShadow::Hover);
    let _element = c.view(&theme);
}

#[test]
fn test_card_view_shadow_never_renders() {
    let theme = Theme::element_light();
    let c = Card::new("Body").with_shadow(CardShadow::Never);
    let _element = c.view(&theme);
}

#[test]
fn test_card_view_empty_body_renders() {
    let theme = Theme::element_light();
    let c = Card::new("");
    let _element = c.view(&theme);
}

#[test]
fn test_card_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let c = Card::new("Body").with_header("H");
    let _element = c.view(&theme);
}
