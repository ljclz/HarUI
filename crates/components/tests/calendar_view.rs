//! Calendar view() 测试 — TDD RED 阶段

use har_ui_components::calendar::{Calendar, CalendarMessage, SimpleDate};
use har_ui_core::theme::Theme;

#[test]
fn test_calendar_view_default_renders() {
    let theme = Theme::element_light();
    let c = Calendar::default();
    let _element = c.view(&theme, |_| ());
}

#[test]
fn test_calendar_view_january_renders() {
    let theme = Theme::element_light();
    let c = Calendar::new(2026, 1);
    let _element = c.view(&theme, |_| ());
}

#[test]
fn test_calendar_view_february_leap_year_renders() {
    let theme = Theme::element_light();
    let c = Calendar::new(2024, 2);
    let _element = c.view(&theme, |_| ());
}

#[test]
fn test_calendar_view_february_non_leap_renders() {
    let theme = Theme::element_light();
    let c = Calendar::new(2026, 2);
    let _element = c.view(&theme, |_| ());
}

#[test]
fn test_calendar_view_with_selected_date_renders() {
    let theme = Theme::element_light();
    let mut c = Calendar::new(2026, 7);
    c.handle(CalendarMessage::SelectDate(SimpleDate::new(2026, 7, 15)));
    let _element = c.view(&theme, |_| ());
}

#[test]
fn test_calendar_view_with_today_renders() {
    let theme = Theme::element_light();
    let c = Calendar::new(2026, 7).with_today(SimpleDate::new(2026, 7, 19));
    let _element = c.view(&theme, |_| ());
}

#[test]
fn test_calendar_view_with_range_renders() {
    let theme = Theme::element_light();
    let c = Calendar::new(2026, 7)
        .with_range(SimpleDate::new(2026, 7, 10), SimpleDate::new(2026, 7, 20));
    let _element = c.view(&theme, |_| ());
}

#[test]
fn test_calendar_view_after_next_month_renders() {
    let theme = Theme::element_light();
    let mut c = Calendar::new(2026, 7);
    c.handle(CalendarMessage::NextMonth);
    let _element = c.view(&theme, |_| ());
}

#[test]
fn test_calendar_view_after_prev_month_renders() {
    let theme = Theme::element_light();
    let mut c = Calendar::new(2026, 7);
    c.handle(CalendarMessage::PrevMonth);
    let _element = c.view(&theme, |_| ());
}

#[test]
fn test_calendar_view_crossing_year_boundary_renders() {
    let theme = Theme::element_light();
    let mut c = Calendar::new(2026, 12);
    c.handle(CalendarMessage::NextMonth);
    assert_eq!(c.year(), 2027);
    assert_eq!(c.month(), 1);
    let _element = c.view(&theme, |_| ());
}

#[test]
fn test_calendar_view_today_button_renders() {
    let theme = Theme::element_light();
    let mut c = Calendar::new(2025, 1).with_today(SimpleDate::new(2026, 7, 19));
    c.handle(CalendarMessage::Today);
    assert_eq!(c.year(), 2026);
    assert_eq!(c.month(), 7);
    let _element = c.view(&theme, |_| ());
}

#[test]
fn test_calendar_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let c = Calendar::new(2026, 7)
        .with_today(SimpleDate::new(2026, 7, 19))
        .with_range(SimpleDate::new(2026, 7, 5), SimpleDate::new(2026, 7, 15));
    let _element = c.view(&theme, |_| ());
}
