//! Alert view() 测试

use har_ui_components::alert::{Alert, AlertEffect, AlertMessage, AlertType};
use har_ui_core::theme::Theme;

#[test]
fn test_alert_view_default_renders() {
    let theme = Theme::element_light();
    let a = Alert::new().with_title("标题");
    let _element = a.view(&theme, || ());
}

#[test]
fn test_alert_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let a = Alert::new().with_title("标题");
    let _element = a.view(&theme, || ());
}

#[test]
fn test_alert_view_with_description_renders() {
    let theme = Theme::element_light();
    let a = Alert::new()
        .with_title("标题")
        .with_description("详细描述");
    let _element = a.view(&theme, || ());
}

#[test]
fn test_alert_view_success_type_renders() {
    let theme = Theme::element_light();
    let a = Alert::new().with_title("成功").with_type(AlertType::Success);
    let _element = a.view(&theme, || ());
}

#[test]
fn test_alert_view_warning_type_renders() {
    let theme = Theme::element_light();
    let a = Alert::new().with_title("警告").with_type(AlertType::Warning);
    let _element = a.view(&theme, || ());
}

#[test]
fn test_alert_view_error_type_renders() {
    let theme = Theme::element_light();
    let a = Alert::new().with_title("错误").with_type(AlertType::Error);
    let _element = a.view(&theme, || ());
}

#[test]
fn test_alert_view_info_type_renders() {
    let theme = Theme::element_light();
    let a = Alert::new().with_title("信息").with_type(AlertType::Info);
    let _element = a.view(&theme, || ());
}

#[test]
fn test_alert_view_closable_false_renders() {
    let theme = Theme::element_light();
    let a = Alert::new().with_title("标题").with_closable(false);
    let _element = a.view(&theme, || ());
}

#[test]
fn test_alert_view_show_icon_false_renders() {
    let theme = Theme::element_light();
    let a = Alert::new().with_title("标题").with_show_icon(false);
    let _element = a.view(&theme, || ());
}

#[test]
fn test_alert_view_center_renders() {
    let theme = Theme::element_light();
    let a = Alert::new().with_title("标题").with_center(true);
    let _element = a.view(&theme, || ());
}

#[test]
fn test_alert_view_dark_effect_renders() {
    let theme = Theme::element_light();
    let a = Alert::new()
        .with_title("标题")
        .with_effect(AlertEffect::Dark);
    let _element = a.view(&theme, || ());
}

#[test]
fn test_alert_view_after_close_renders() {
    let theme = Theme::element_light();
    let mut a = Alert::new().with_title("标题");
    a.handle(AlertMessage::Close);
    assert!(!a.visible());
    let _element = a.view(&theme, || ());
}

#[test]
fn test_alert_view_with_close_callback_renders() {
    let theme = Theme::element_light();
    let a = Alert::new().with_title("标题").with_closable(true);
    let _element = a.view(&theme, || ());
}
