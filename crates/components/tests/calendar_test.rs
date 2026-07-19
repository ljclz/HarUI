//! Calendar 日历 — 参考 Element Plus `<el-calendar>`。
//!
//! 覆盖：月面板/上下月切换/今天/选定日期/范围/月面板网格计算。

use har_ui_components::calendar::{Calendar, CalendarMessage, SimpleDate};

#[test]
fn test_calendar_default_this_month() {
    let c = Calendar::new(2026, 7);
    assert_eq!(c.year(), 2026);
    assert_eq!(c.month(), 7);
}

#[test]
fn test_calendar_next_month() {
    let mut c = Calendar::new(2026, 1);
    c.handle(CalendarMessage::NextMonth);
    assert_eq!(c.year(), 2026);
    assert_eq!(c.month(), 2);
}

#[test]
fn test_calendar_next_month_cross_year() {
    let mut c = Calendar::new(2026, 12);
    c.handle(CalendarMessage::NextMonth);
    assert_eq!(c.year(), 2027);
    assert_eq!(c.month(), 1);
}

#[test]
fn test_calendar_prev_month_cross_year() {
    let mut c = Calendar::new(2026, 1);
    c.handle(CalendarMessage::PrevMonth);
    assert_eq!(c.year(), 2025);
    assert_eq!(c.month(), 12);
}

#[test]
fn test_calendar_today_jump() {
    let mut c = Calendar::new(2025, 1);
    c.handle(CalendarMessage::Today);
    // 跳到今天（2026-07-19）
    assert_eq!(c.year(), 2026);
    assert_eq!(c.month(), 7);
}

#[test]
fn test_calendar_select_date() {
    let mut c = Calendar::new(2026, 7);
    c.handle(CalendarMessage::SelectDate(SimpleDate::new(2026, 7, 15)));
    assert_eq!(c.selected(), Some(&SimpleDate::new(2026, 7, 15)));
}

#[test]
fn test_calendar_month_grid_42_cells() {
    // 月面板固定 6 行 × 7 列 = 42 格（含上下月填充）
    let c = Calendar::new(2026, 7);
    let grid = c.month_grid();
    assert_eq!(grid.len(), 42);
}

#[test]
fn test_calendar_month_grid_first_cell_correct() {
    // 2026-07-01 是星期三，第一天格应该是 2026-06-29（周一）
    let c = Calendar::new(2026, 7);
    let grid = c.month_grid();
    assert_eq!(grid[0], SimpleDate::new(2026, 6, 29));
}

#[test]
fn test_calendar_month_grid_includes_next_month() {
    let c = Calendar::new(2026, 7);
    let grid = c.month_grid();
    // 7 月 31 天 + 起始 6 月 29/30 日 + 8 月初若干天 = 42
    let has_august = grid.iter().any(|d| d.year == 2026 && d.month == 8);
    assert!(has_august);
}

#[test]
fn test_calendar_is_today() {
    let c = Calendar::new(2026, 7);
    let today = SimpleDate::new(2026, 7, 19);
    assert!(c.is_today(&today));
    let not_today = SimpleDate::new(2026, 7, 18);
    assert!(!c.is_today(&not_today));
}

#[test]
fn test_calendar_is_current_month() {
    let c = Calendar::new(2026, 7);
    assert!(c.is_current_month(&SimpleDate::new(2026, 7, 15)));
    assert!(!c.is_current_month(&SimpleDate::new(2026, 6, 30)));
}

#[test]
fn test_calendar_range_check() {
    let c = Calendar::new(2026, 7)
        .with_range(SimpleDate::new(2026, 7, 10), SimpleDate::new(2026, 7, 20));
    assert!(c.in_range(&SimpleDate::new(2026, 7, 15)));
    assert!(!c.in_range(&SimpleDate::new(2026, 7, 25)));
}
