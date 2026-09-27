//! StatusBar 状态栏组件 — POS 专用
//!
//! 显示硬件设备状态：连接、称重、扫码、打印机、网络。
//! 支持左右两侧插槽、自定义状态项。

use har_ui_components::status_bar::{StatusBar, StatusBarMessage, StatusItem, StatusLevel};

// ---------- 基础构造 ----------

#[test]
fn test_status_bar_default() {
    let s = StatusBar::new();
    assert!(s.left_items().is_empty());
    assert!(s.right_items().is_empty());
}

#[test]
fn test_status_bar_with_left_items() {
    let s = StatusBar::new()
        .with_left_item(StatusItem::new("秤", StatusLevel::Success))
        .with_left_item(StatusItem::new("扫码枪", StatusLevel::Success));
    assert_eq!(s.left_items().len(), 2);
    assert_eq!(s.left_items()[0].label(), "秤");
}

#[test]
fn test_status_bar_with_right_items() {
    let s = StatusBar::new()
        .with_right_item(StatusItem::new("网络", StatusLevel::Success))
        .with_right_item(StatusItem::new("时间", StatusLevel::Info));
    assert_eq!(s.right_items().len(), 2);
}

// ---------- StatusItem ----------

#[test]
fn test_status_item_default() {
    let item = StatusItem::new("网络", StatusLevel::Success);
    assert_eq!(item.label(), "网络");
    assert_eq!(item.level(), StatusLevel::Success);
    assert_eq!(item.detail(), None);
}

#[test]
fn test_status_item_with_detail() {
    let item = StatusItem::new("网络", StatusLevel::Success).with_detail("已连接 100Mbps");
    assert_eq!(item.detail(), Some(&"已连接 100Mbps".to_string()));
}

#[test]
fn test_status_item_levels() {
    let levels = [
        StatusLevel::Success,
        StatusLevel::Warning,
        StatusLevel::Error,
        StatusLevel::Info,
    ];
    assert_eq!(levels.len(), 4);
}

// ---------- 更新状态 ----------

#[test]
fn test_status_bar_update_left_item() {
    let mut s = StatusBar::new().with_left_item(StatusItem::new("秤", StatusLevel::Success));
    s.handle(StatusBarMessage::UpdateLeft(
        0,
        StatusLevel::Error,
        Some("离线".to_string()),
    ));
    assert_eq!(s.left_items()[0].level(), StatusLevel::Error);
    assert_eq!(s.left_items()[0].detail(), Some(&"离线".to_string()));
}

#[test]
fn test_status_bar_update_right_item() {
    let mut s = StatusBar::new().with_right_item(StatusItem::new("网络", StatusLevel::Success));
    s.handle(StatusBarMessage::UpdateRight(
        0,
        StatusLevel::Warning,
        Some("弱信号".to_string()),
    ));
    assert_eq!(s.right_items()[0].level(), StatusLevel::Warning);
}

#[test]
fn test_status_bar_update_nonexistent_index_ignored() {
    let mut s = StatusBar::new().with_left_item(StatusItem::new("秤", StatusLevel::Success));
    // 越界索引应被忽略，不 panic
    s.handle(StatusBarMessage::UpdateLeft(5, StatusLevel::Error, None));
    assert_eq!(s.left_items()[0].level(), StatusLevel::Success);
}

// ---------- 批量更新硬件状态 ----------

#[test]
fn test_status_bar_set_hardware_status() {
    let mut s = StatusBar::new();
    s.handle(StatusBarMessage::SetHardwareStatus {
        scale: StatusLevel::Success,
        scanner: StatusLevel::Warning,
        printer: StatusLevel::Error,
        network: StatusLevel::Success,
    });
    // 应自动添加 4 个状态项到左侧
    assert_eq!(s.left_items().len(), 4);
    let labels: Vec<&str> = s.left_items().iter().map(|i| i.label()).collect();
    assert!(labels.contains(&"秤"));
    assert!(labels.contains(&"扫码枪"));
    assert!(labels.contains(&"打印机"));
    assert!(labels.contains(&"网络"));
}

#[test]
fn test_status_bar_set_hardware_status_overwrites() {
    // 多次调用 SetHardwareStatus 应覆盖而非追加
    let mut s = StatusBar::new();
    s.handle(StatusBarMessage::SetHardwareStatus {
        scale: StatusLevel::Success,
        scanner: StatusLevel::Success,
        printer: StatusLevel::Success,
        network: StatusLevel::Success,
    });
    s.handle(StatusBarMessage::SetHardwareStatus {
        scale: StatusLevel::Error,
        scanner: StatusLevel::Error,
        printer: StatusLevel::Error,
        network: StatusLevel::Error,
    });
    assert_eq!(s.left_items().len(), 4);
    // 全部应为 Error
    for item in s.left_items() {
        assert_eq!(item.level(), StatusLevel::Error);
    }
}

// ---------- 自定义右侧时间项 ----------

#[test]
fn test_status_bar_clear_all() {
    let mut s = StatusBar::new()
        .with_left_item(StatusItem::new("秤", StatusLevel::Success))
        .with_right_item(StatusItem::new("时间", StatusLevel::Info));
    s.handle(StatusBarMessage::ClearAll);
    assert_eq!(s.left_items().len(), 0);
    assert_eq!(s.right_items().len(), 0);
}
