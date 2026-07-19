//! R.2.P1.2 Badge view() 测试 — TDD RED 阶段

use har_ui_components::badge::{Badge, BadgePosition, BadgeValue};
use har_ui_core::theme::Theme;

#[test]
fn test_badge_view_number_renders() {
    let theme = Theme::element_light();
    let b = Badge::new(BadgeValue::Number(5));
    let _element = b.view(&theme);
}

#[test]
fn test_badge_view_text_renders() {
    let theme = Theme::element_light();
    let b = Badge::new(BadgeValue::Text("new".into()));
    let _element = b.view(&theme);
}

#[test]
fn test_badge_view_is_dot_renders() {
    let theme = Theme::element_light();
    let b = Badge::new(BadgeValue::Number(0)).with_is_dot(true);
    let _element = b.view(&theme);
}

#[test]
fn test_badge_view_zero_hidden_renders() {
    let theme = Theme::element_light();
    let b = Badge::new(BadgeValue::Number(0));
    let _element = b.view(&theme);
}

#[test]
fn test_badge_view_overflow_renders() {
    let theme = Theme::element_light();
    let b = Badge::new(BadgeValue::Number(200)).with_max(99);
    let _element = b.view(&theme);
}

#[test]
fn test_badge_view_top_left_position_renders() {
    let theme = Theme::element_light();
    let b = Badge::new(BadgeValue::Number(5)).with_position(BadgePosition::TopLeft);
    let _element = b.view(&theme);
}

#[test]
fn test_badge_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let b = Badge::new(BadgeValue::Number(5));
    let _element = b.view(&theme);
}
