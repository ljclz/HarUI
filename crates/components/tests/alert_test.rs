//! Alert 警告 — 参考 Element Plus `<el-alert>`。
//!
//! 覆盖：4 种 type、title/description、closable、center、show-icon、effect（light/dark）。

use har_ui_components::alert::{Alert, AlertMessage, AlertType, AlertEffect};

#[test]
fn test_alert_default() {
    let a = Alert::new();
    assert_eq!(a.alert_type(), AlertType::Info);
    assert_eq!(a.title(), "");
    assert_eq!(a.description(), None);
    assert!(a.closable());
    assert!(!a.center());
    assert!(a.show_icon());
    assert_eq!(a.effect(), AlertEffect::Light);
    assert!(a.visible());
}

#[test]
fn test_alert_with_title() {
    let a = Alert::new().with_title("警告提示");
    assert_eq!(a.title(), "警告提示");
}

#[test]
fn test_alert_with_description() {
    let a = Alert::new().with_description("详细描述文字");
    assert_eq!(a.description(), Some("详细描述文字"));
}

#[test]
fn test_alert_types() {
    let a = Alert::new().with_type(AlertType::Success);
    assert_eq!(a.alert_type(), AlertType::Success);
    let a = Alert::new().with_type(AlertType::Warning);
    assert_eq!(a.alert_type(), AlertType::Warning);
    let a = Alert::new().with_type(AlertType::Error);
    assert_eq!(a.alert_type(), AlertType::Error);
}

#[test]
fn test_alert_close() {
    let mut a = Alert::new().with_closable(true);
    assert!(a.visible());
    a.handle(AlertMessage::Close);
    assert!(!a.visible());
}

#[test]
fn test_alert_not_closable() {
    let mut a = Alert::new().with_closable(false);
    a.handle(AlertMessage::Close);
    // 不可关闭 → 关闭事件无效
    assert!(a.visible());
}

#[test]
fn test_alert_show_icon_toggle() {
    let a = Alert::new().with_show_icon(false);
    assert!(!a.show_icon());
}

#[test]
fn test_alert_center_mode() {
    let a = Alert::new().with_center(true);
    assert!(a.center());
}

#[test]
fn test_alert_effect_dark() {
    let a = Alert::new().with_effect(AlertEffect::Dark);
    assert_eq!(a.effect(), AlertEffect::Dark);
}

#[test]
fn test_alert_reopen_after_close() {
    let mut a = Alert::new();
    a.handle(AlertMessage::Close);
    assert!(!a.visible());
    a.handle(AlertMessage::Open);
    assert!(a.visible());
}
