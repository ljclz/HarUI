//! R.2.P1.21 Rate view() 测试

use har_ui_components::rate::{Rate, RateMessage};
use har_ui_core::theme::Theme;

#[test]
fn test_rate_view_default_renders() {
    let theme = Theme::element_light();
    let r = Rate::new();
    let _element = r.view(&theme, |_| ());
}

#[test]
fn test_rate_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let r = Rate::new();
    let _element = r.view(&theme, |_| ());
}

#[test]
fn test_rate_view_with_value_renders() {
    let theme = Theme::element_light();
    let mut r = Rate::new();
    r.handle(RateMessage::SetValue(3.0));
    let _element = r.view(&theme, |_| ());
}

#[test]
fn test_rate_view_max_value_renders() {
    let theme = Theme::element_light();
    let mut r = Rate::new();
    r.handle(RateMessage::SetValue(5.0));
    let _element = r.view(&theme, |_| ());
}

#[test]
fn test_rate_view_disabled_renders() {
    let theme = Theme::element_light();
    let r = Rate::new().with_disabled(true);
    let _element = r.view(&theme, |_| ());
}

#[test]
fn test_rate_view_allow_half_renders() {
    let theme = Theme::element_light();
    let mut r = Rate::new().with_allow_half(true);
    r.handle(RateMessage::SetValue(3.5));
    let _element = r.view(&theme, |_| ());
}

#[test]
fn test_rate_view_show_text_renders() {
    let theme = Theme::element_light();
    let mut r = Rate::new().with_show_text(true);
    r.handle(RateMessage::SetValue(4.0));
    let _element = r.view(&theme, |_| ());
}

#[test]
fn test_rate_view_show_score_renders() {
    let theme = Theme::element_light();
    let mut r = Rate::new().with_show_score(true).with_allow_half(true);
    r.handle(RateMessage::SetValue(3.5));
    let _element = r.view(&theme, |_| ());
}

#[test]
fn test_rate_view_custom_max_renders() {
    let theme = Theme::element_light();
    let r = Rate::new().with_max(10);
    let _element = r.view(&theme, |_| ());
}

#[test]
fn test_rate_view_clearable_renders() {
    let theme = Theme::element_light();
    let mut r = Rate::new().with_clearable(true);
    r.handle(RateMessage::SetValue(2.0));
    r.handle(RateMessage::Clear);
    let _element = r.view(&theme, |_| ());
}

#[test]
fn test_rate_view_zero_value_renders() {
    let theme = Theme::element_light();
    let r = Rate::new();
    let _element = r.view(&theme, |_| ());
}

#[test]
fn test_rate_view_with_custom_message_type() {
    let theme = Theme::element_light();
    let r = Rate::new();
    #[derive(Clone, Debug)]
    #[allow(dead_code)]
    enum AppMsg {
        Rate(u32),
    }
    let _element = r.view(&theme, AppMsg::Rate);
}
