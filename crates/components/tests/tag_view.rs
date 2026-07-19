//! R.2.P1.1 Tag view() 测试 — TDD RED 阶段

use har_ui_components::tag::{Tag, TagEffect, TagSize, TagType};
use har_ui_core::theme::Theme;

#[test]
fn test_tag_view_default_renders() {
    let theme = Theme::element_light();
    let t = Tag::new("hello");
    let _element = t.view(&theme, || ());
}

#[test]
fn test_tag_view_primary_type_renders() {
    let theme = Theme::element_light();
    let t = Tag::new("hello").with_type(TagType::Primary);
    let _element = t.view(&theme, || ());
}

#[test]
fn test_tag_view_success_type_renders() {
    let theme = Theme::element_light();
    let t = Tag::new("ok").with_type(TagType::Success);
    let _element = t.view(&theme, || ());
}

#[test]
fn test_tag_view_dark_effect_renders() {
    let theme = Theme::element_light();
    let t = Tag::new("hi").with_effect(TagEffect::Dark);
    let _element = t.view(&theme, || ());
}

#[test]
fn test_tag_view_plain_effect_renders() {
    let theme = Theme::element_light();
    let t = Tag::new("hi").with_effect(TagEffect::Plain);
    let _element = t.view(&theme, || ());
}

#[test]
fn test_tag_view_large_size_renders() {
    let theme = Theme::element_light();
    let t = Tag::new("hi").with_size(TagSize::Large);
    let _element = t.view(&theme, || ());
}

#[test]
fn test_tag_view_closable_renders() {
    let theme = Theme::element_light();
    let t = Tag::new("hi").with_closable(true);
    let _element = t.view(&theme, || ());
}

#[test]
fn test_tag_view_custom_color_renders() {
    let theme = Theme::element_light();
    let t = Tag::new("hi").with_color("#FF0000");
    let _element = t.view(&theme, || ());
}

#[test]
fn test_tag_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let t = Tag::new("hi").with_type(TagType::Primary);
    let _element = t.view(&theme, || ());
}

#[test]
fn test_tag_view_closed_renders() {
    let theme = Theme::element_light();
    let mut t = Tag::new("hi").with_closable(true);
    use har_ui_components::tag::TagMessage;
    t.handle(TagMessage::Close);
    let _element = t.view(&theme, || ());
}
