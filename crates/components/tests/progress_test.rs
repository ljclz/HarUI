//! Progress 进度条 — 参考 Element Plus `<el-progress>`。
//!
//! 覆盖：百分比、type（line/circle/dashboard）、status、stroke-width、color、show-text。

use har_ui_components::progress::{Progress, ProgressMessage, ProgressStatus, ProgressType};

#[test]
fn test_progress_default() {
    let p = Progress::new();
    assert_eq!(p.picker_type(), ProgressType::Line);
    assert_eq!(p.percentage(), 0);
    assert_eq!(p.status(), ProgressStatus::Default);
    assert_eq!(p.stroke_width(), 6);
    assert!(p.show_text());
}

#[test]
fn test_progress_set_percentage() {
    let mut p = Progress::new();
    p.handle(ProgressMessage::SetPercentage(50));
    assert_eq!(p.percentage(), 50);
}

#[test]
fn test_progress_percentage_clamped() {
    let mut p = Progress::new();
    p.handle(ProgressMessage::SetPercentage(150));
    assert_eq!(p.percentage(), 100);
    p.handle(ProgressMessage::SetPercentage(-10));
    assert_eq!(p.percentage(), 0);
}

#[test]
fn test_progress_auto_status_success_at_100() {
    let mut p = Progress::new();
    p.handle(ProgressMessage::SetPercentage(100));
    assert_eq!(p.status(), ProgressStatus::Success);
}

#[test]
fn test_progress_manual_status() {
    let p = Progress::new().with_status(ProgressStatus::Exception);
    assert_eq!(p.status(), ProgressStatus::Exception);
}

#[test]
fn test_progress_type_circle() {
    let p = Progress::new().with_type(ProgressType::Circle);
    assert_eq!(p.picker_type(), ProgressType::Circle);
}

#[test]
fn test_progress_type_dashboard() {
    let p = Progress::new().with_type(ProgressType::Dashboard);
    assert_eq!(p.picker_type(), ProgressType::Dashboard);
}

#[test]
fn test_progress_stroke_width() {
    let p = Progress::new().with_stroke_width(10);
    assert_eq!(p.stroke_width(), 10);
}

#[test]
fn test_progress_show_text_toggle() {
    let p = Progress::new().with_show_text(false);
    assert!(!p.show_text());
}

#[test]
fn test_progress_increment() {
    let mut p = Progress::new();
    p.handle(ProgressMessage::Increment(10));
    assert_eq!(p.percentage(), 10);
    p.handle(ProgressMessage::Increment(20));
    assert_eq!(p.percentage(), 30);
    // 不能超过 100
    p.handle(ProgressMessage::Increment(100));
    assert_eq!(p.percentage(), 100);
}

#[test]
fn test_progress_color_override() {
    let p = Progress::new().with_color("#67c23a");
    assert_eq!(p.color(), Some("#67c23a"));
}

#[test]
fn test_progress_format_text_line() {
    let mut p = Progress::new();
    p.handle(ProgressMessage::SetPercentage(42));
    assert_eq!(p.formatted_text(), "42%");
}

#[test]
fn test_progress_reset() {
    let mut p = Progress::new();
    p.handle(ProgressMessage::SetPercentage(80));
    p.handle(ProgressMessage::Reset);
    assert_eq!(p.percentage(), 0);
    assert_eq!(p.status(), ProgressStatus::Default);
}
