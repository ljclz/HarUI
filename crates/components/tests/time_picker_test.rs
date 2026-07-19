//! TimePicker 时间选择器 — 参考 Element Plus `<el-time-picker>`。
//!
//! 覆盖：基础选择、is_range 范围、format、disabled、clearable、placeholder。

use har_ui_components::time_picker::{TimePicker, TimePickerMessage, TimeValue};

#[test]
fn test_time_picker_default() {
    let p = TimePicker::new();
    assert_eq!(p.value(), None);
    assert!(!p.visible());
    assert!(!p.disabled());
    assert!(!p.is_range());
}

#[test]
fn test_time_picker_open_close() {
    let mut p = TimePicker::new();
    p.handle(TimePickerMessage::Open);
    assert!(p.visible());
    p.handle(TimePickerMessage::Close);
    assert!(!p.visible());
}

#[test]
fn test_time_picker_select() {
    let mut p = TimePicker::new();
    p.handle(TimePickerMessage::Select(TimeValue::new(14, 30, 0)));
    assert_eq!(p.value(), Some(&TimeValue::new(14, 30, 0)));
    assert!(!p.visible()); // 自动关闭
}

#[test]
fn test_time_picker_disabled_blocks_select() {
    let mut p = TimePicker::new().with_disabled(true);
    p.handle(TimePickerMessage::Select(TimeValue::new(14, 30, 0)));
    assert_eq!(p.value(), None);
}

#[test]
fn test_time_picker_clear() {
    let mut p = TimePicker::new().with_clearable(true);
    p.handle(TimePickerMessage::Select(TimeValue::new(14, 30, 0)));
    p.handle(TimePickerMessage::Clear);
    assert_eq!(p.value(), None);
}

#[test]
fn test_time_picker_range_mode() {
    let mut p = TimePicker::new().with_range(true);
    assert!(p.is_range());
    p.handle(TimePickerMessage::SelectStart(TimeValue::new(9, 0, 0)));
    p.handle(TimePickerMessage::SelectEnd(TimeValue::new(18, 0, 0)));
    assert_eq!(p.start_value(), Some(&TimeValue::new(9, 0, 0)));
    assert_eq!(p.end_value(), Some(&TimeValue::new(18, 0, 0)));
}

#[test]
fn test_time_picker_format_default() {
    let mut p = TimePicker::new();
    p.handle(TimePickerMessage::Select(TimeValue::new(14, 30, 0)));
    assert_eq!(p.formatted_value(), "14:30:00");
}

#[test]
fn test_time_picker_format_custom() {
    let mut p = TimePicker::new().with_format("HH:mm");
    p.handle(TimePickerMessage::Select(TimeValue::new(14, 30, 0)));
    assert_eq!(p.formatted_value(), "14:30");
}

#[test]
fn test_time_picker_placeholder() {
    let p = TimePicker::new().with_placeholder("选择时间");
    assert_eq!(p.placeholder(), "选择时间");
}

#[test]
fn test_time_picker_value_clamped() {
    let mut p = TimePicker::new();
    // 25:99:99 应被钳制到 23:59:59
    p.handle(TimePickerMessage::Select(TimeValue::new(25, 99, 99)));
    assert_eq!(p.value(), Some(&TimeValue::new(23, 59, 59)));
}

#[test]
fn test_time_picker_arrow_controls() {
    let mut p = TimePicker::new();
    p.handle(TimePickerMessage::Select(TimeValue::new(10, 30, 0)));
    // 增加小时
    p.handle(TimePickerMessage::IncHour);
    assert_eq!(p.value(), Some(&TimeValue::new(11, 30, 0)));
    // 减少分钟
    p.handle(TimePickerMessage::DecMinute);
    assert_eq!(p.value(), Some(&TimeValue::new(11, 29, 0)));
    // 0 时减小时应回绕到 23
    p.handle(TimePickerMessage::Select(TimeValue::new(0, 0, 0)));
    p.handle(TimePickerMessage::DecHour);
    assert_eq!(p.value(), Some(&TimeValue::new(23, 0, 0)));
}
