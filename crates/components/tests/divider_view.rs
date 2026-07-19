//! Divider view() 测试

use har_ui_components::divider::{
    Divider, DividerContentPosition, DividerDirection,
};
use har_ui_core::theme::Theme;

#[test]
fn test_divider_view_default_renders() {
    let theme = Theme::element_light();
    let d = Divider::new();
    let _element = d.view(&theme);
}

#[test]
fn test_divider_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let d = Divider::new();
    let _element = d.view(&theme);
}

#[test]
fn test_divider_view_vertical_renders() {
    let theme = Theme::element_light();
    let d = Divider::new().with_direction(DividerDirection::Vertical);
    let _element = d.view(&theme);
}

#[test]
fn test_divider_view_horizontal_no_text_renders() {
    let theme = Theme::element_light();
    let d = Divider::new().with_direction(DividerDirection::Horizontal);
    let _element = d.view(&theme);
}

#[test]
fn test_divider_view_with_text_center_renders() {
    let theme = Theme::element_light();
    let d = Divider::new()
        .with_text("分割")
        .with_content_position(DividerContentPosition::Center);
    let _element = d.view(&theme);
}

#[test]
fn test_divider_view_with_text_left_renders() {
    let theme = Theme::element_light();
    let d = Divider::new()
        .with_text("左对齐")
        .with_content_position(DividerContentPosition::Left);
    let _element = d.view(&theme);
}

#[test]
fn test_divider_view_with_text_right_renders() {
    let theme = Theme::element_light();
    let d = Divider::new()
        .with_text("右对齐")
        .with_content_position(DividerContentPosition::Right);
    let _element = d.view(&theme);
}

#[test]
fn test_divider_view_dashed_renders() {
    let theme = Theme::element_light();
    let d = Divider::new().with_border_dashed(true);
    let _element = d.view(&theme);
}

#[test]
fn test_divider_view_dashed_with_text_renders() {
    let theme = Theme::element_light();
    let d = Divider::new()
        .with_text("虚线")
        .with_border_dashed(true);
    let _element = d.view(&theme);
}

#[test]
fn test_divider_view_clear_text_renders() {
    let theme = Theme::element_light();
    let mut d = Divider::new().with_text("文本");
    d.clear_text();
    let _element = d.view(&theme);
}

#[test]
fn test_divider_view_vertical_dark_renders() {
    let theme = Theme::element_dark();
    let d = Divider::new()
        .with_direction(DividerDirection::Vertical)
        .with_text("垂直");
    let _element = d.view(&theme);
}
