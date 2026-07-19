//! Keypad 组件 — 数字键盘（POS 专用）
//!
//! 参考 POS 收银机的数字键盘。
//! 支持：
//! - 0-9 数字输入
//! - 小数点（00 / . ）
//! - 退格、清除
//! - OK 确认
//! - 三种模式：price（价格，最多 2 位小数）、number（整数）、quantity（数量，3 位小数）
//! - max/min 边界检查
//! - 触摸优化（按钮热区 ≥ 44x44px）

use har_ui_components::keypad::{Keypad, KeypadMessage, KeypadMode, KeypadState};

// ---------- 基础构造 ----------

#[test]
fn test_keypad_default() {
    let k = Keypad::new();
    assert_eq!(k.value(), "");
    assert_eq!(k.mode(), KeypadMode::Price);
    assert_eq!(k.state(), KeypadState::Editing);
    assert_eq!(k.max(), None);
    assert_eq!(k.min(), None);
}

#[test]
fn test_keypad_with_mode() {
    let k = Keypad::new().with_mode(KeypadMode::Number);
    assert_eq!(k.mode(), KeypadMode::Number);

    let k2 = Keypad::new().with_mode(KeypadMode::Quantity);
    assert_eq!(k2.mode(), KeypadMode::Quantity);
}

#[test]
fn test_keypad_with_max_min() {
    let k = Keypad::new()
        .with_max(9999.99)
        .with_min(0.10);
    assert_eq!(k.max(), Some(9999.99));
    assert_eq!(k.min(), Some(0.10));
}

// ---------- 数字输入 ----------

#[test]
fn test_keypad_digit_input() {
    let mut k = Keypad::new();
    k.handle(KeypadMessage::Digit(1));
    k.handle(KeypadMessage::Digit(2));
    k.handle(KeypadMessage::Digit(3));
    assert_eq!(k.value(), "123");
}

#[test]
fn test_keypad_zero_first_then_digit() {
    // 前导 0 后输入数字应替换 0
    let mut k = Keypad::new();
    k.handle(KeypadMessage::Digit(0));
    k.handle(KeypadMessage::Digit(5));
    assert_eq!(k.value(), "5");
}

#[test]
fn test_keypad_multiple_zeros_ignored() {
    let mut k = Keypad::new();
    k.handle(KeypadMessage::Digit(0));
    k.handle(KeypadMessage::Digit(0));
    k.handle(KeypadMessage::Digit(0));
    assert_eq!(k.value(), "0");
}

// ---------- 小数点 ----------

#[test]
fn test_keypad_decimal_point_price_mode() {
    // price 模式：最多 2 位小数
    let mut k = Keypad::new().with_mode(KeypadMode::Price);
    k.handle(KeypadMessage::Digit(1));
    k.handle(KeypadMessage::Dot);
    k.handle(KeypadMessage::Digit(2));
    k.handle(KeypadMessage::Digit(3));
    k.handle(KeypadMessage::Digit(4)); // 应被忽略（超出 2 位）
    assert_eq!(k.value(), "1.23");
}

#[test]
fn test_keypad_decimal_point_quantity_mode() {
    // quantity 模式：最多 3 位小数
    let mut k = Keypad::new().with_mode(KeypadMode::Quantity);
    k.handle(KeypadMessage::Digit(1));
    k.handle(KeypadMessage::Dot);
    k.handle(KeypadMessage::Digit(2));
    k.handle(KeypadMessage::Digit(3));
    k.handle(KeypadMessage::Digit(4));
    k.handle(KeypadMessage::Digit(5)); // 应被忽略
    assert_eq!(k.value(), "1.234");
}

#[test]
fn test_keypad_decimal_point_number_mode_ignored() {
    // number 模式：不允许小数点
    let mut k = Keypad::new().with_mode(KeypadMode::Number);
    k.handle(KeypadMessage::Digit(1));
    k.handle(KeypadMessage::Dot); // 应被忽略
    k.handle(KeypadMessage::Digit(2));
    assert_eq!(k.value(), "12");
}

#[test]
fn test_keypad_double_dot_ignored() {
    let mut k = Keypad::new();
    k.handle(KeypadMessage::Digit(1));
    k.handle(KeypadMessage::Dot);
    k.handle(KeypadMessage::Dot); // 第二个小数点应被忽略
    k.handle(KeypadMessage::Digit(2));
    assert_eq!(k.value(), "1.2");
}

#[test]
fn test_keypad_dot_first_prepends_zero() {
    // 直接输入小数点应以 "0." 开头
    let mut k = Keypad::new();
    k.handle(KeypadMessage::Dot);
    k.handle(KeypadMessage::Digit(5));
    assert_eq!(k.value(), "0.5");
}

// ---------- 退格、清除 ----------

#[test]
fn test_keypad_backspace() {
    let mut k = Keypad::new();
    k.handle(KeypadMessage::Digit(1));
    k.handle(KeypadMessage::Digit(2));
    k.handle(KeypadMessage::Digit(3));
    k.handle(KeypadMessage::Backspace);
    assert_eq!(k.value(), "12");
}

#[test]
fn test_keypad_backspace_empty() {
    let mut k = Keypad::new();
    k.handle(KeypadMessage::Backspace);
    assert_eq!(k.value(), "");
}

#[test]
fn test_keypad_clear() {
    let mut k = Keypad::new();
    k.handle(KeypadMessage::Digit(1));
    k.handle(KeypadMessage::Digit(2));
    k.handle(KeypadMessage::Clear);
    assert_eq!(k.value(), "");
}

// ---------- OK 确认 ----------

#[test]
fn test_keypad_ok_confirm_empty_value_keeps_editing() {
    // 空值时 OK 不能确认
    let mut k = Keypad::new();
    k.handle(KeypadMessage::Ok);
    assert_eq!(k.state(), KeypadState::Editing);
}

#[test]
fn test_keypad_ok_confirm_valid_value() {
    let mut k = Keypad::new().with_max(100.0).with_min(1.0);
    k.handle(KeypadMessage::Digit(5));
    k.handle(KeypadMessage::Digit(0));
    k.handle(KeypadMessage::Ok);
    assert_eq!(k.state(), KeypadState::Confirmed);
    assert!((k.numeric_value().unwrap() - 50.0).abs() < f64::EPSILON);
}

#[test]
fn test_keypad_ok_confirm_exceeds_max() {
    // 超出 max 应进入 Invalid 状态
    let mut k = Keypad::new().with_max(100.0);
    k.handle(KeypadMessage::Digit(1));
    k.handle(KeypadMessage::Digit(0));
    k.handle(KeypadMessage::Digit(0));
    k.handle(KeypadMessage::Digit(0)); // 1000
    k.handle(KeypadMessage::Ok);
    assert_eq!(k.state(), KeypadState::Invalid);
}

#[test]
fn test_keypad_ok_confirm_below_min() {
    // 低于 min 应进入 Invalid 状态
    let mut k = Keypad::new().with_min(10.0);
    k.handle(KeypadMessage::Digit(5));
    k.handle(KeypadMessage::Ok);
    assert_eq!(k.state(), KeypadState::Invalid);
}

#[test]
fn test_keypad_reset_after_confirm() {
    let mut k = Keypad::new();
    k.handle(KeypadMessage::Digit(5));
    k.handle(KeypadMessage::Ok);
    assert_eq!(k.state(), KeypadState::Confirmed);
    k.reset();
    assert_eq!(k.value(), "");
    assert_eq!(k.state(), KeypadState::Editing);
}

// ---------- 双零键（00） ----------

#[test]
fn test_keypad_double_zero() {
    let mut k = Keypad::new();
    k.handle(KeypadMessage::Digit(1));
    k.handle(KeypadMessage::DoubleZero);
    assert_eq!(k.value(), "100");
}

#[test]
fn test_keypad_double_zero_after_dot() {
    // 小数点后输入 00 应只保留两位
    let mut k = Keypad::new().with_mode(KeypadMode::Price);
    k.handle(KeypadMessage::Digit(1));
    k.handle(KeypadMessage::Dot);
    k.handle(KeypadMessage::DoubleZero);
    assert_eq!(k.value(), "1.00");
}

// ---------- numeric_value ----------

#[test]
fn test_keypad_numeric_value_empty() {
    let k = Keypad::new();
    assert_eq!(k.numeric_value(), None);
}

#[test]
fn test_keypad_numeric_value_dot_only() {
    let mut k = Keypad::new();
    k.handle(KeypadMessage::Dot);
    // 只有 "." 应视为 0
    assert!(k.numeric_value().is_some());
    assert!((k.numeric_value().unwrap() - 0.0).abs() < f64::EPSILON);
}

// ---------- 热区 ----------

#[test]
fn test_keypad_button_hot_zone_at_least_44px() {
    // 触摸热区 ≥ 44x44px
    let k = Keypad::new();
    let (w, h) = k.button_size();
    assert!(w >= 44.0, "button width {} < 44", w);
    assert!(h >= 44.0, "button height {} < 44", h);
}

// ---------- 极长输入保护 ----------

#[test]
fn test_keypad_max_length_protection() {
    // 整数部分最多 8 位
    let mut k = Keypad::new();
    for _ in 0..20 {
        k.handle(KeypadMessage::Digit(9));
    }
    // 整数部分截断到 8 位
    assert_eq!(k.value().len() <= 8, true);
}
