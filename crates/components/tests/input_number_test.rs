//! InputNumber 组件测试
//!
//! 参考 Element Plus `<el-input-number>` 组件 API。
//! 测试覆盖：
//! - min / max / step / precision
//! - +/- 按钮控制
//! - 直接输入
//! - controls_position (right两侧 / right 两侧箭头)

use har_ui_components::input_number::{ControlsPosition, InputNumber, InputNumberMessage};

#[test]
fn test_input_number_default_value_is_zero() {
    let inp = InputNumber::new();
    assert_eq!(inp.value(), 0.0);
}

#[test]
fn test_input_number_with_min_max() {
    let inp = InputNumber::new().with_min(0.0).with_max(100.0);
    assert_eq!(inp.min(), Some(0.0));
    assert_eq!(inp.max(), Some(100.0));
}

#[test]
fn test_input_number_with_step() {
    let inp = InputNumber::new().with_step(5.0);
    assert_eq!(inp.step(), 5.0);
}

#[test]
fn test_input_number_default_step_is_one() {
    let inp = InputNumber::new();
    assert_eq!(inp.step(), 1.0);
}

#[test]
fn test_input_number_increment_button() {
    let mut inp = InputNumber::new().with_step(2.0);
    inp.handle(InputNumberMessage::Increment);
    assert_eq!(inp.value(), 2.0);
    inp.handle(InputNumberMessage::Increment);
    assert_eq!(inp.value(), 4.0);
}

#[test]
fn test_input_number_decrement_button() {
    let mut inp = InputNumber::new().with_step(2.0);
    inp.handle(InputNumberMessage::Decrement);
    assert_eq!(inp.value(), -2.0);
    inp.handle(InputNumberMessage::Decrement);
    assert_eq!(inp.value(), -4.0);
}

#[test]
fn test_input_number_clamps_to_max() {
    let mut inp = InputNumber::new().with_max(10.0).with_step(3.0);
    inp.handle(InputNumberMessage::Increment); // 3
    inp.handle(InputNumberMessage::Increment); // 6
    inp.handle(InputNumberMessage::Increment); // 9
    inp.handle(InputNumberMessage::Increment); // 应 clamp 到 10
    assert_eq!(inp.value(), 10.0);
}

#[test]
fn test_input_number_clamps_to_min() {
    let mut inp = InputNumber::new().with_min(-5.0).with_step(3.0);
    inp.handle(InputNumberMessage::Decrement); // -3
    inp.handle(InputNumberMessage::Decrement); // -6 → clamp 到 -5
    assert_eq!(inp.value(), -5.0);
}

#[test]
fn test_input_number_precision_rounds() {
    let mut inp = InputNumber::new().with_step(0.1).with_precision(2);
    inp.handle(InputNumberMessage::Increment); // 0.1
    inp.handle(InputNumberMessage::Increment); // 0.2
    inp.handle(InputNumberMessage::Increment); // 0.3
    // 应保留两位小数
    let v = inp.value();
    assert!((v - 0.3).abs() < 0.001);
}

#[test]
fn test_input_number_direct_input() {
    let mut inp = InputNumber::new();
    inp.handle(InputNumberMessage::SetValue(42.5));
    assert_eq!(inp.value(), 42.5);
}

#[test]
fn test_input_number_direct_input_clamped() {
    let mut inp = InputNumber::new().with_min(0.0).with_max(100.0);
    inp.handle(InputNumberMessage::SetValue(-10.0));
    assert_eq!(inp.value(), 0.0);
    inp.handle(InputNumberMessage::SetValue(150.0));
    assert_eq!(inp.value(), 100.0);
}

#[test]
fn test_input_number_controls_position_variants() {
    let positions = [ControlsPosition::Default, ControlsPosition::Right];
    assert_eq!(positions.len(), 2);
    assert_ne!(ControlsPosition::Default, ControlsPosition::Right);
}

#[test]
fn test_input_number_disabled_blocks_increment() {
    let mut inp = InputNumber::new().disabled(true);
    inp.handle(InputNumberMessage::Increment);
    assert_eq!(inp.value(), 0.0);
}

#[test]
fn test_input_number_set_value_applies_precision() {
    let mut inp = InputNumber::new().with_precision(2);
    inp.handle(InputNumberMessage::SetValue(123.456));
    // 应四舍五入到 2 位小数
    assert!((inp.value() - 123.46).abs() < 0.001);
}

#[test]
fn test_input_number_step_can_be_fractional() {
    let mut inp = InputNumber::new()
        .with_step(0.5)
        .with_min(0.0)
        .with_max(2.0);
    inp.handle(InputNumberMessage::Increment); // 0.5
    assert_eq!(inp.value(), 0.5);
    inp.handle(InputNumberMessage::Increment); // 1.0
    assert_eq!(inp.value(), 1.0);
    inp.handle(InputNumberMessage::Increment); // 1.5
    assert_eq!(inp.value(), 1.5);
    inp.handle(InputNumberMessage::Increment); // 2.0
    assert_eq!(inp.value(), 2.0);
    inp.handle(InputNumberMessage::Increment); // clamp to 2.0
    assert_eq!(inp.value(), 2.0);
}

#[test]
fn test_input_number_builder_chains() {
    let inp = InputNumber::new()
        .with_min(0.0)
        .with_max(100.0)
        .with_step(5.0)
        .with_precision(1)
        .disabled(false);
    assert_eq!(inp.min(), Some(0.0));
    assert_eq!(inp.max(), Some(100.0));
    assert_eq!(inp.step(), 5.0);
    assert_eq!(inp.precision(), Some(1));
    assert!(!inp.is_disabled());
}
