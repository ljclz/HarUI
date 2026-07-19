//! R.2.P1.18 DatePicker view() 测试

use har_ui_components::date_picker::{DatePicker, DatePickerMessage, DatePickerType, SimpleDate};
use har_ui_core::theme::Theme;

#[test]
fn test_date_picker_view_default_renders() {
    let theme = Theme::element_light();
    let dp = DatePicker::new();
    let _element = dp.view(&theme, |_| ());
}

#[test]
fn test_date_picker_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let dp = DatePicker::new();
    let _element = dp.view(&theme, |_| ());
}

#[test]
fn test_date_picker_view_disabled_renders() {
    let theme = Theme::element_light();
    let dp = DatePicker::new().with_disabled(true);
    let _element = dp.view(&theme, |_| ());
}

#[test]
fn test_date_picker_view_with_value_renders() {
    let theme = Theme::element_light();
    let mut dp = DatePicker::new();
    dp.handle(DatePickerMessage::SelectDate(SimpleDate::new(2026, 7, 15)));
    let _element = dp.view(&theme, |_| ());
}

#[test]
fn test_date_picker_view_open_panel_renders() {
    let theme = Theme::element_light();
    let mut dp = DatePicker::new();
    dp.handle(DatePickerMessage::Open);
    let _element = dp.view(&theme, |_| ());
}

#[test]
fn test_date_picker_view_open_with_selected_renders() {
    let theme = Theme::element_light();
    let mut dp = DatePicker::new();
    dp.handle(DatePickerMessage::SelectDate(SimpleDate::new(2026, 7, 15)));
    dp.handle(DatePickerMessage::Open);
    let _element = dp.view(&theme, |_| ());
}

#[test]
fn test_date_picker_view_daterange_with_range_renders() {
    let theme = Theme::element_light();
    let mut dp = DatePicker::new()
        .with_type(DatePickerType::DateRange)
        .with_today(SimpleDate::new(2026, 7, 19));
    dp.handle(DatePickerMessage::SelectRangeStart(SimpleDate::new(2026, 7, 10)));
    dp.handle(DatePickerMessage::SelectRangeEnd(SimpleDate::new(2026, 7, 20)));
    dp.handle(DatePickerMessage::Open);
    let _element = dp.view(&theme, |_| ());
}

#[test]
fn test_date_picker_view_datetime_type_renders() {
    let theme = Theme::element_light();
    let dp = DatePicker::new().with_type(DatePickerType::DateTime);
    let _element = dp.view(&theme, |_| ());
}

#[test]
fn test_date_picker_view_year_type_renders() {
    let theme = Theme::element_light();
    let dp = DatePicker::new().with_type(DatePickerType::Year);
    let _element = dp.view(&theme, |_| ());
}

#[test]
fn test_date_picker_view_month_type_renders() {
    let theme = Theme::element_light();
    let dp = DatePicker::new().with_type(DatePickerType::Month);
    let _element = dp.view(&theme, |_| ());
}

#[test]
fn test_date_picker_view_clearable_renders() {
    let theme = Theme::element_light();
    let dp = DatePicker::new().with_clearable(true);
    let _element = dp.view(&theme, |_| ());
}

#[test]
fn test_date_picker_view_custom_placeholder_renders() {
    let theme = Theme::element_light();
    let dp = DatePicker::new().with_placeholder("Pick a date");
    let _element = dp.view(&theme, |_| ());
}

#[test]
fn test_date_picker_view_open_dark_theme_renders() {
    let theme = Theme::element_dark();
    let mut dp = DatePicker::new();
    dp.handle(DatePickerMessage::Open);
    let _element = dp.view(&theme, |_| ());
}

#[test]
fn test_date_picker_view_with_custom_message_type() {
    let theme = Theme::element_light();
    let dp = DatePicker::new();
    #[derive(Clone, Debug)]
    #[allow(dead_code)]
    enum AppMsg {
        Pick(String),
    }
    let _element = dp.view(&theme, |s| AppMsg::Pick(s));
}
