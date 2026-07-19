//! R.2.P1.16 Backtop view() 测试 — TDD RED 阶段

use har_ui_components::backtop::{Backtop, BacktopMessage};
use har_ui_core::theme::Theme;

#[test]
fn test_backtop_view_default_invisible_renders() {
    // 默认 scroll_y=0，不显示
    let theme = Theme::element_light();
    let b = Backtop::new();
    let _element = b.view(&theme, ());
}

#[test]
fn test_backtop_view_visible_renders() {
    let theme = Theme::element_light();
    let mut b = Backtop::new();
    b.handle(BacktopMessage::Scroll(300));
    let _element = b.view(&theme, ());
}

#[test]
fn test_backtop_view_threshold_exact_renders() {
    // 默认 visibility_height=200，scroll_y=200 不显示（严格大于）
    let theme = Theme::element_light();
    let mut b = Backtop::new();
    b.handle(BacktopMessage::Scroll(200));
    let _element = b.view(&theme, ());
}

#[test]
fn test_backtop_view_threshold_above_renders() {
    let theme = Theme::element_light();
    let mut b = Backtop::new();
    b.handle(BacktopMessage::Scroll(201));
    let _element = b.view(&theme, ());
}

#[test]
fn test_backtop_view_custom_visibility_height_renders() {
    let theme = Theme::element_light();
    let mut b = Backtop::new().with_visibility_height(500);
    b.handle(BacktopMessage::Scroll(600));
    let _element = b.view(&theme, ());
}

#[test]
fn test_backtop_view_smooth_renders() {
    let theme = Theme::element_light();
    let mut b = Backtop::new().with_smooth(true);
    b.handle(BacktopMessage::Scroll(300));
    let _element = b.view(&theme, ());
}

#[test]
fn test_backtop_view_custom_position_renders() {
    let theme = Theme::element_light();
    let mut b = Backtop::new()
        .with_right(80)
        .with_bottom(80);
    b.handle(BacktopMessage::Scroll(300));
    let _element = b.view(&theme, ());
}

#[test]
fn test_backtop_view_after_click_renders() {
    let theme = Theme::element_light();
    let mut b = Backtop::new();
    b.handle(BacktopMessage::Scroll(300));
    b.handle(BacktopMessage::Click);
    // 点击后 scroll_y=0，不显示
    let _element = b.view(&theme, ());
}

#[test]
fn test_backtop_view_set_visibility_height_msg_renders() {
    let theme = Theme::element_light();
    let mut b = Backtop::new();
    b.handle(BacktopMessage::SetVisibilityHeight(50));
    b.handle(BacktopMessage::Scroll(100));
    let _element = b.view(&theme, ());
}

#[test]
fn test_backtop_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let mut b = Backtop::new();
    b.handle(BacktopMessage::Scroll(300));
    let _element = b.view(&theme, ());
}

#[test]
fn test_backtop_view_custom_message_type() {
    let theme = Theme::element_light();
    let mut b = Backtop::new();
    b.handle(BacktopMessage::Scroll(300));
    #[derive(Clone, Debug)]
    #[allow(dead_code)]
    enum AppMsg {
        BackToTop,
    }
    let _element = b.view(&theme, AppMsg::BackToTop);
}
