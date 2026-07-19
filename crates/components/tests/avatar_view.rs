//! R.2.P1.3 Avatar view() 测试 — TDD RED 阶段

use har_ui_components::avatar::{Avatar, AvatarFit, AvatarShape, AvatarSize};
use har_ui_core::theme::Theme;

#[test]
fn test_avatar_view_default_renders() {
    let theme = Theme::element_light();
    let a = Avatar::new();
    let _element = a.view(&theme);
}

#[test]
fn test_avatar_view_with_text_renders() {
    let theme = Theme::element_light();
    let a = Avatar::new().with_text("A");
    let _element = a.view(&theme);
}

#[test]
fn test_avatar_view_with_initials_renders() {
    let theme = Theme::element_light();
    let a = Avatar::new().with_initials("Alice");
    let _element = a.view(&theme);
}

#[test]
fn test_avatar_view_with_icon_renders() {
    let theme = Theme::element_light();
    let a = Avatar::new().with_icon("👤");
    let _element = a.view(&theme);
}

#[test]
fn test_avatar_view_with_image_renders() {
    let theme = Theme::element_light();
    let a = Avatar::new().with_image_url("/img.png");
    let _element = a.view(&theme);
}

#[test]
fn test_avatar_view_large_size_renders() {
    let theme = Theme::element_light();
    let a = Avatar::new().with_text("A").with_size(AvatarSize::Large);
    let _element = a.view(&theme);
}

#[test]
fn test_avatar_view_square_shape_renders() {
    let theme = Theme::element_light();
    let a = Avatar::new().with_text("A").with_shape(AvatarShape::Square);
    let _element = a.view(&theme);
}

#[test]
fn test_avatar_view_pixel_size_renders() {
    let theme = Theme::element_light();
    let a = Avatar::new().with_text("A").with_pixel_size(64);
    let _element = a.view(&theme);
}

#[test]
fn test_avatar_view_with_fit_renders() {
    let theme = Theme::element_light();
    let a = Avatar::new().with_image_url("/img.png").with_fit(AvatarFit::Contain);
    let _element = a.view(&theme);
}

#[test]
fn test_avatar_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let a = Avatar::new().with_text("A");
    let _element = a.view(&theme);
}

#[test]
fn test_avatar_view_with_fallback_renders() {
    let theme = Theme::element_light();
    let mut a = Avatar::new().with_image_url("/img.png").with_fallback_text("A");
    a.handle_load_error();
    let _element = a.view(&theme);
}
