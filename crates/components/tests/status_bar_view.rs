//! StatusBar view() 测试 — TDD RED 阶段

use har_ui_components::status_bar::{StatusBar, StatusBarMessage, StatusItem, StatusLevel};
use har_ui_core::theme::Theme;

#[test]
fn test_status_bar_view_default_renders() {
    let theme = Theme::element_light();
    let s = StatusBar::new();
    let _element = s.view(&theme);
}

#[test]
fn test_status_bar_view_with_left_items_renders() {
    let theme = Theme::element_light();
    let s = StatusBar::new()
        .with_left_item(StatusItem::new("秤", StatusLevel::Success))
        .with_left_item(StatusItem::new("扫码枪", StatusLevel::Success));
    let _element = s.view(&theme);
}

#[test]
fn test_status_bar_view_with_right_items_renders() {
    let theme = Theme::element_light();
    let s = StatusBar::new()
        .with_right_item(StatusItem::new("用户", StatusLevel::Info))
        .with_right_item(StatusItem::new("时间", StatusLevel::Info));
    let _element = s.view(&theme);
}

#[test]
fn test_status_bar_view_with_both_sides_renders() {
    let theme = Theme::element_light();
    let s = StatusBar::new()
        .with_left_item(StatusItem::new("秤", StatusLevel::Success))
        .with_right_item(StatusItem::new("用户", StatusLevel::Info));
    let _element = s.view(&theme);
}

#[test]
fn test_status_bar_view_with_detail_renders() {
    let theme = Theme::element_light();
    let s = StatusBar::new().with_left_item(
        StatusItem::new("网络", StatusLevel::Success).with_detail("100Mbps"),
    );
    let _element = s.view(&theme);
}

#[test]
fn test_status_bar_view_hardware_status_renders() {
    let theme = Theme::element_light();
    let mut s = StatusBar::new();
    s.handle(StatusBarMessage::SetHardwareStatus {
        scale: StatusLevel::Success,
        scanner: StatusLevel::Warning,
        printer: StatusLevel::Error,
        network: StatusLevel::Success,
    });
    let _element = s.view(&theme);
}

#[test]
fn test_status_bar_view_all_levels_renders() {
    let theme = Theme::element_light();
    let s = StatusBar::new()
        .with_left_item(StatusItem::new("info", StatusLevel::Info))
        .with_left_item(StatusItem::new("ok", StatusLevel::Success))
        .with_left_item(StatusItem::new("warn", StatusLevel::Warning))
        .with_left_item(StatusItem::new("err", StatusLevel::Error));
    let _element = s.view(&theme);
}

#[test]
fn test_status_bar_view_after_clear_all_renders() {
    let theme = Theme::element_light();
    let mut s = StatusBar::new()
        .with_left_item(StatusItem::new("A", StatusLevel::Info))
        .with_right_item(StatusItem::new("B", StatusLevel::Info));
    s.handle(StatusBarMessage::ClearAll);
    let _element = s.view(&theme);
}

#[test]
fn test_status_bar_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let mut s = StatusBar::new()
        .with_left_item(StatusItem::new("秤", StatusLevel::Success))
        .with_right_item(StatusItem::new("时间", StatusLevel::Info));
    s.handle(StatusBarMessage::SetHardwareStatus {
        scale: StatusLevel::Error,
        scanner: StatusLevel::Error,
        printer: StatusLevel::Warning,
        network: StatusLevel::Success,
    });
    let _element = s.view(&theme);
}
