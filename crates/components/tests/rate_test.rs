//! Rate 评分 — 参考 Element Plus `<el-rate>`。
//!
//! 覆盖：max、value、disabled、allow_half、increase/decrease、clear、show_text。

use har_ui_components::rate::{Rate, RateMessage};

#[test]
fn test_rate_default() {
    let r = Rate::new();
    assert_eq!(r.max(), 5);
    assert_eq!(r.value(), 0.0);
    assert!(!r.disabled());
    assert!(!r.allow_half());
    assert!(!r.show_text());
    assert!(!r.clearable());
}

#[test]
fn test_rate_with_max() {
    let r = Rate::new().with_max(10);
    assert_eq!(r.max(), 10);
}

#[test]
fn test_rate_set_value() {
    let mut r = Rate::new();
    r.handle(RateMessage::SetValue(3.0));
    assert_eq!(r.value(), 3.0);
}

#[test]
fn test_rate_clamp_to_max() {
    let mut r = Rate::new().with_max(5);
    r.handle(RateMessage::SetValue(10.0));
    assert_eq!(r.value(), 5.0);
}

#[test]
fn test_rate_clamp_to_zero() {
    let mut r = Rate::new();
    r.handle(RateMessage::SetValue(-2.0));
    assert_eq!(r.value(), 0.0);
}

#[test]
fn test_rate_disabled_noop() {
    let mut r = Rate::new().with_disabled(true);
    r.handle(RateMessage::SetValue(3.0));
    assert_eq!(r.value(), 0.0);
}

#[test]
fn test_rate_without_allow_half_clamps_to_int() {
    let mut r = Rate::new();
    r.handle(RateMessage::SetValue(3.5));
    // 不允许半星时，3.5 应截断为 3
    assert_eq!(r.value(), 3.0);
}

#[test]
fn test_rate_with_allow_half_keeps_half() {
    let mut r = Rate::new().with_allow_half(true);
    r.handle(RateMessage::SetValue(3.5));
    assert_eq!(r.value(), 3.5);
}

#[test]
fn test_rate_allow_half_clamp_odd_value() {
    let mut r = Rate::new().with_allow_half(true);
    // 3.7 应回落到 3.5
    r.handle(RateMessage::SetValue(3.7));
    assert_eq!(r.value(), 3.5);
}

#[test]
fn test_rate_increase() {
    let mut r = Rate::new();
    r.handle(RateMessage::SetValue(3.0));
    r.handle(RateMessage::Increase);
    assert_eq!(r.value(), 4.0);
}

#[test]
fn test_rate_increase_clamp_at_max() {
    let mut r = Rate::new().with_max(5);
    r.handle(RateMessage::SetValue(5.0));
    r.handle(RateMessage::Increase);
    assert_eq!(r.value(), 5.0);
}

#[test]
fn test_rate_decrease() {
    let mut r = Rate::new();
    r.handle(RateMessage::SetValue(3.0));
    r.handle(RateMessage::Decrease);
    assert_eq!(r.value(), 2.0);
}

#[test]
fn test_rate_decrease_clamp_at_zero() {
    let mut r = Rate::new();
    r.handle(RateMessage::SetValue(0.0));
    r.handle(RateMessage::Decrease);
    assert_eq!(r.value(), 0.0);
}

#[test]
fn test_rate_clear() {
    let mut r = Rate::new();
    r.handle(RateMessage::SetValue(3.0));
    r.handle(RateMessage::Clear);
    assert_eq!(r.value(), 0.0);
}

#[test]
fn test_rate_with_show_text() {
    let r = Rate::new().with_show_text(true);
    assert!(r.show_text());
}

#[test]
fn test_rate_with_clearable() {
    let r = Rate::new().with_clearable(true);
    assert!(r.clearable());
}
