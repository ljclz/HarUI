//! R.2.P1.17 Affix view() 测试 — TDD RED 阶段

use har_ui_components::affix::{Affix, AffixMessage, AffixPosition};
use har_ui_core::theme::Theme;
use iced::Element;
use iced::widget::text;

fn make_content<'a>() -> Element<'a, ()> {
    text("Affix content").into()
}

#[test]
fn test_affix_view_default_renders() {
    let theme = Theme::element_light();
    let a = Affix::new();
    let _element = a.view(&theme, make_content());
}

#[test]
fn test_affix_view_fixed_state_renders() {
    let theme = Theme::element_light();
    let mut a = Affix::new().with_offset(100);
    a.handle(AffixMessage::Scroll { scroll_y: 200 });
    let _element = a.view(&theme, make_content());
}

#[test]
fn test_affix_view_not_fixed_state_renders() {
    let theme = Theme::element_light();
    let mut a = Affix::new().with_offset(100);
    a.handle(AffixMessage::Scroll { scroll_y: 50 });
    let _element = a.view(&theme, make_content());
}

#[test]
fn test_affix_view_top_position_renders() {
    let theme = Theme::element_light();
    let a = Affix::new().with_position(AffixPosition::Top);
    let _element = a.view(&theme, make_content());
}

#[test]
fn test_affix_view_bottom_position_renders() {
    let theme = Theme::element_light();
    let mut a = Affix::new()
        .with_position(AffixPosition::Bottom)
        .with_offset(500);
    // bottom 模式：scroll_y < offset 时固定
    a.handle(AffixMessage::Scroll { scroll_y: 100 });
    let _element = a.view(&theme, make_content());
}

#[test]
fn test_affix_view_with_offset_renders() {
    let theme = Theme::element_light();
    let a = Affix::new().with_offset(50);
    let _element = a.view(&theme, make_content());
}

#[test]
fn test_affix_view_with_zindex_renders() {
    let theme = Theme::element_light();
    let a = Affix::new().with_zindex(200);
    let _element = a.view(&theme, make_content());
}

#[test]
fn test_affix_view_with_target_renders() {
    let theme = Theme::element_light();
    let a = Affix::new().with_target(".container");
    let _element = a.view(&theme, make_content());
}

#[test]
fn test_affix_view_change_fired_renders() {
    let theme = Theme::element_light();
    let mut a = Affix::new().with_offset(100);
    a.handle(AffixMessage::Scroll { scroll_y: 200 });
    assert!(a.change_fired());
    a.clear_change_flag();
    let _element = a.view(&theme, make_content());
}

#[test]
fn test_affix_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let mut a = Affix::new().with_offset(100);
    a.handle(AffixMessage::Scroll { scroll_y: 200 });
    let _element = a.view(&theme, make_content());
}

#[test]
fn test_affix_view_custom_message_type() {
    let theme = Theme::element_light();
    let a = Affix::new();
    #[derive(Clone, Debug)]
    #[allow(dead_code)]
    enum AppMsg {
        Nop,
    }
    let content: Element<AppMsg> = text("content").into();
    let _element = a.view(&theme, content);
}
