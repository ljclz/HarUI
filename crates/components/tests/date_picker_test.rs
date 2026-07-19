//! DatePicker 日期选择器 — 参考 Element Plus `<el-date-picker>`。
//!
//! 覆盖：基础选择、placeholder、disabled、clearable、type（date/datetime/month/year）、范围选择。

use har_ui_components::date_picker::{DatePicker, DatePickerMessage, DatePickerType, SimpleDate};

#[test]
fn test_date_picker_default() {
    let p = DatePicker::new();
    assert_eq!(p.picker_type(), DatePickerType::Date);
    assert_eq!(p.value(), None);
    assert!(!p.visible());
    assert!(!p.disabled());
}

#[test]
fn test_date_picker_open_close() {
    let mut p = DatePicker::new();
    p.handle(DatePickerMessage::Open);
    assert!(p.visible());
    p.handle(DatePickerMessage::Close);
    assert!(!p.visible());
}

#[test]
fn test_date_picker_select_date() {
    let mut p = DatePicker::new();
    p.handle(DatePickerMessage::SelectDate(SimpleDate::new(2026, 7, 19)));
    assert_eq!(p.value(), Some(&SimpleDate::new(2026, 7, 19)));
    // 选中后自动关闭
    assert!(!p.visible());
}

#[test]
fn test_date_picker_clear() {
    let mut p = DatePicker::new().with_clearable(true);
    p.handle(DatePickerMessage::SelectDate(SimpleDate::new(2026, 7, 19)));
    p.handle(DatePickerMessage::Clear);
    assert_eq!(p.value(), None);
}

#[test]
fn test_date_picker_disabled_blocks_select() {
    let mut p = DatePicker::new().with_disabled(true);
    p.handle(DatePickerMessage::SelectDate(SimpleDate::new(2026, 7, 19)));
    assert_eq!(p.value(), None);
}

#[test]
fn test_date_picker_disabled_blocks_open() {
    let mut p = DatePicker::new().with_disabled(true);
    p.handle(DatePickerMessage::Open);
    assert!(!p.visible());
}

#[test]
fn test_date_picker_type_datetime() {
    let p = DatePicker::new().with_type(DatePickerType::DateTime);
    assert_eq!(p.picker_type(), DatePickerType::DateTime);
}

#[test]
fn test_date_picker_type_month() {
    let p = DatePicker::new().with_type(DatePickerType::Month);
    assert_eq!(p.picker_type(), DatePickerType::Month);
}

#[test]
fn test_date_picker_type_year() {
    let p = DatePicker::new().with_type(DatePickerType::Year);
    assert_eq!(p.picker_type(), DatePickerType::Year);
}

#[test]
fn test_date_picker_placeholder() {
    let p = DatePicker::new().with_placeholder("选择日期");
    assert_eq!(p.placeholder(), "选择日期");
}

#[test]
fn test_date_picker_range_mode() {
    // 范围模式：选择起止日期
    let mut p = DatePicker::new().with_type(DatePickerType::DateRange);
    assert_eq!(p.picker_type(), DatePickerType::DateRange);
    p.handle(DatePickerMessage::SelectRangeStart(SimpleDate::new(2026, 7, 1)));
    p.handle(DatePickerMessage::SelectRangeEnd(SimpleDate::new(2026, 7, 31)));
    assert_eq!(p.range_start(), Some(&SimpleDate::new(2026, 7, 1)));
    assert_eq!(p.range_end(), Some(&SimpleDate::new(2026, 7, 31)));
}

#[test]
fn test_date_picker_format_value() {
    let mut p = DatePicker::new();
    p.handle(DatePickerMessage::SelectDate(SimpleDate::new(2026, 7, 19)));
    assert_eq!(p.formatted_value(), "2026-07-19");
}

#[test]
fn test_date_picker_format_value_with_time() {
    let mut p = DatePicker::new().with_type(DatePickerType::DateTime);
    p.handle(DatePickerMessage::SelectDateTime(SimpleDate::new(2026, 7, 19), 14, 30, 0));
    assert_eq!(p.formatted_value(), "2026-07-19 14:30:00");
}
