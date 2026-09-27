//! TimePicker view() 测试 — TDD RED 阶段

use har_ui_components::time_picker::{TimePicker, TimePickerMessage, TimeValue};
use har_ui_core::theme::Theme;

#[test]
fn test_time_picker_view_default_renders() {
    let theme = Theme::element_light();
    let p = TimePicker::new();
    let _element = p.view(&theme, |_| ());
}

#[test]
fn test_time_picker_view_with_value_renders() {
    let theme = Theme::element_light();
    let mut p = TimePicker::new();
    p.handle(TimePickerMessage::Select(TimeValue::new(10, 30, 45)));
    let _element = p.view(&theme, |_| ());
}

#[test]
fn test_time_picker_view_open_renders() {
    let theme = Theme::element_light();
    let mut p = TimePicker::new();
    p.handle(TimePickerMessage::Open);
    let _element = p.view(&theme, |_| ());
}

#[test]
fn test_time_picker_view_disabled_renders() {
    let theme = Theme::element_light();
    let p = TimePicker::new().with_disabled(true);
    let _element = p.view(&theme, |_| ());
}

#[test]
fn test_time_picker_view_clearable_renders() {
    let theme = Theme::element_light();
    let p = TimePicker::new()
        .with_clearable(true)
        .with_placeholder("请选择时间");
    let _element = p.view(&theme, |_| ());
}

#[test]
fn test_time_picker_view_range_mode_renders() {
    let theme = Theme::element_light();
    let p = TimePicker::new().with_range(true);
    let _element = p.view(&theme, |_| ());
}

#[test]
fn test_time_picker_view_custom_format_renders() {
    let theme = Theme::element_light();
    let mut p = TimePicker::new().with_format("HH:mm");
    p.handle(TimePickerMessage::Select(TimeValue::new(8, 5, 0)));
    let _element = p.view(&theme, |_| ());
}

#[test]
fn test_time_picker_view_after_clear_renders() {
    let theme = Theme::element_light();
    let mut p = TimePicker::new().with_clearable(true);
    p.handle(TimePickerMessage::Select(TimeValue::new(10, 30, 0)));
    p.handle(TimePickerMessage::Clear);
    let _element = p.view(&theme, |_| ());
}

#[test]
fn test_time_picker_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let mut p = TimePicker::new();
    p.handle(TimePickerMessage::Select(TimeValue::new(10, 30, 45)));
    p.handle(TimePickerMessage::Open);
    let _element = p.view(&theme, |_| ());
}

#[test]
fn test_time_picker_view_custom_message_type() {
    let theme = Theme::element_light();
    let p = TimePicker::new();
    #[derive(Clone, Debug)]
    #[allow(dead_code)]
    enum AppMsg {
        Pick(String),
    }
    let _element = p.view(&theme, AppMsg::Pick);
}
