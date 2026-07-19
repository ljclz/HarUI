//! Badge 组件 — 徽章
//!
//! 参考 Element Plus `<el-badge>`。
//! 支持：数字/小红点/文本，max 溢出，is-dot，位置 top-right/bottom-left 等。

use har_ui_components::badge::{Badge, BadgeMessage, BadgePosition, BadgeValue};

// ---------- 基础构造 ----------

#[test]
fn test_badge_default() {
    let b = Badge::new(BadgeValue::Number(5));
    assert_eq!(b.value(), &BadgeValue::Number(5));
    assert_eq!(b.max(), 99);
    assert!(!b.is_dot());
    assert_eq!(b.position(), BadgePosition::TopRight);
    assert!(!b.hidden());
}

#[test]
fn test_badge_with_max() {
    let b = Badge::new(BadgeValue::Number(5)).with_max(10);
    assert_eq!(b.max(), 10);
}

#[test]
fn test_badge_with_is_dot() {
    let b = Badge::new(BadgeValue::Number(0)).with_is_dot(true);
    assert!(b.is_dot());
}

#[test]
fn test_badge_with_position() {
    let b = Badge::new(BadgeValue::Number(5)).with_position(BadgePosition::BottomLeft);
    assert_eq!(b.position(), BadgePosition::BottomLeft);
}

// ---------- 显示文本 ----------

#[test]
fn test_badge_display_number_below_max() {
    let b = Badge::new(BadgeValue::Number(5)).with_max(99);
    assert_eq!(b.display_text(), "5");
}

#[test]
fn test_badge_display_number_above_max() {
    // 超过 max 显示 max+
    let b = Badge::new(BadgeValue::Number(150)).with_max(99);
    assert_eq!(b.display_text(), "99+");
}

#[test]
fn test_badge_display_text_value() {
    let b = Badge::new(BadgeValue::Text("New".to_string()));
    assert_eq!(b.display_text(), "New");
}

#[test]
fn test_badge_display_dot() {
    // is_dot=true 时显示为小红点
    let b = Badge::new(BadgeValue::Number(0)).with_is_dot(true);
    assert_eq!(b.display_text(), "");
}

// ---------- hidden ----------

#[test]
fn test_badge_hidden_when_number_zero() {
    // Number(0) 默认隐藏
    let b = Badge::new(BadgeValue::Number(0));
    assert!(b.hidden());
}

#[test]
fn test_badge_visible_when_number_nonzero() {
    let b = Badge::new(BadgeValue::Number(1));
    assert!(!b.hidden());
}

#[test]
fn test_badge_dot_visible_when_is_dot() {
    // is_dot=true 时即使 Number(0) 也显示
    let b = Badge::new(BadgeValue::Number(0)).with_is_dot(true);
    assert!(!b.hidden());
}

#[test]
fn test_badge_text_always_visible() {
    // Text 类型始终可见
    let b = Badge::new(BadgeValue::Text("Hello".to_string()));
    assert!(!b.hidden());
}

// ---------- 更新值 ----------

#[test]
fn test_badge_update_value() {
    let mut b = Badge::new(BadgeValue::Number(1));
    b.handle(BadgeMessage::Update(BadgeValue::Number(10)));
    assert_eq!(b.value(), &BadgeValue::Number(10));
}

#[test]
fn test_badge_update_to_zero_hides() {
    let mut b = Badge::new(BadgeValue::Number(5));
    b.handle(BadgeMessage::Update(BadgeValue::Number(0)));
    assert!(b.hidden());
}

#[test]
fn test_badge_set_hidden() {
    let mut b = Badge::new(BadgeValue::Number(5));
    b.handle(BadgeMessage::SetHidden(true));
    assert!(b.hidden());
    b.handle(BadgeMessage::SetHidden(false));
    assert!(!b.hidden());
}

// ---------- Position ----------

#[test]
fn test_badge_positions() {
    let positions = [
        BadgePosition::TopRight,
        BadgePosition::TopLeft,
        BadgePosition::BottomRight,
        BadgePosition::BottomLeft,
    ];
    assert_eq!(positions.len(), 4);
}

// ---------- BadgeValue equality ----------

#[test]
fn test_badge_value_equality() {
    assert_eq!(BadgeValue::Number(5), BadgeValue::Number(5));
    assert_ne!(BadgeValue::Number(5), BadgeValue::Number(6));
    assert_eq!(
        BadgeValue::Text("new".to_string()),
        BadgeValue::Text("new".to_string())
    );
}
