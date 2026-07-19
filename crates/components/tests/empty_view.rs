//! R.2.P1.6 Empty view() 测试 — TDD RED 阶段

use har_ui_components::empty::{Empty, EmptyImage, EmptySize};
use har_ui_core::theme::Theme;

#[test]
fn test_empty_view_default_renders() {
    let theme = Theme::element_light();
    let e = Empty::new();
    let _element = e.view(&theme);
}

#[test]
fn test_empty_view_with_description_renders() {
    let theme = Theme::element_light();
    let e = Empty::new().with_description("无数据");
    let _element = e.view(&theme);
}

#[test]
fn test_empty_view_error_image_renders() {
    let theme = Theme::element_light();
    let e = Empty::new().with_image(EmptyImage::Error);
    let _element = e.view(&theme);
}

#[test]
fn test_empty_view_network_image_renders() {
    let theme = Theme::element_light();
    let e = Empty::new().with_image(EmptyImage::Network);
    let _element = e.view(&theme);
}

#[test]
fn test_empty_view_custom_image_url_renders() {
    let theme = Theme::element_light();
    let e = Empty::new().with_image_url("/empty.png");
    let _element = e.view(&theme);
}

#[test]
fn test_empty_view_small_size_renders() {
    let theme = Theme::element_light();
    let e = Empty::new().with_size(EmptySize::Small);
    let _element = e.view(&theme);
}

#[test]
fn test_empty_view_large_size_renders() {
    let theme = Theme::element_light();
    let e = Empty::new().with_size(EmptySize::Large);
    let _element = e.view(&theme);
}

#[test]
fn test_empty_view_with_extra_renders() {
    let theme = Theme::element_light();
    let e = Empty::new().with_has_extra(true);
    let _element = e.view(&theme);
}

#[test]
fn test_empty_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let e = Empty::new();
    let _element = e.view(&theme);
}
