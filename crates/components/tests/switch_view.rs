//! R.2.P0.10 Switch view() 测试 — TDD RED 阶段
//!
//! 验证 Switch view() 正确渲染开关滑块 + 文本。

use har_ui_components::switch::{Switch, SwitchValue};
use har_ui_core::theme::Theme;

#[test]
fn test_switch_view_off_renders() {
    let theme = Theme::element_light();
    let s = Switch::new();
    let _element = s.view(&theme, ());
}

#[test]
fn test_switch_view_on_renders() {
    let theme = Theme::element_light();
    let s = Switch::with_value(true);
    let _element = s.view(&theme, ());
}

#[test]
fn test_switch_view_disabled_renders() {
    let theme = Theme::element_light();
    let s = Switch::new().with_disabled(true);
    let _element = s.view(&theme, ());
}

#[test]
fn test_switch_view_loading_renders() {
    let theme = Theme::element_light();
    let s = Switch::new().with_loading(true);
    let _element = s.view(&theme, ());
}

#[test]
fn test_switch_view_with_active_text_renders() {
    let theme = Theme::element_light();
    let s = Switch::with_value(true).with_active_text("开");
    let _element = s.view(&theme, ());
}

#[test]
fn test_switch_view_with_inactive_text_renders() {
    let theme = Theme::element_light();
    let s = Switch::new().with_inactive_text("关");
    let _element = s.view(&theme, ());
}

#[test]
fn test_switch_view_with_both_texts_renders() {
    let theme = Theme::element_light();
    let s = Switch::with_value(true)
        .with_active_text("ON")
        .with_inactive_text("OFF");
    let _element = s.view(&theme, ());
}

#[test]
fn test_switch_view_custom_width_renders() {
    let theme = Theme::element_light();
    let s = Switch::new().with_width(60);
    let _element = s.view(&theme, ());
}

#[test]
fn test_switch_view_custom_colors_renders() {
    let theme = Theme::element_light();
    let s = Switch::with_value(true)
        .with_active_color("#13CE66")
        .with_inactive_color("#FF4949");
    let _element = s.view(&theme, ());
}

#[test]
fn test_switch_view_with_custom_values_renders() {
    let theme = Theme::element_light();
    let s = Switch::new()
        .with_active_value(SwitchValue::Text("yes".into()))
        .with_inactive_value(SwitchValue::Text("no".into()));
    let _element = s.view(&theme, ());
}

#[test]
fn test_switch_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let s = Switch::with_value(true);
    let _element = s.view(&theme, ());
}

#[test]
fn test_switch_view_with_custom_message_type() {
    let theme = Theme::element_light();
    let s = Switch::new();
    #[derive(Clone, Debug)]
    #[allow(dead_code)]
    enum AppMsg {
        Toggled,
    }
    let _element = s.view(&theme, AppMsg::Toggled);
}
