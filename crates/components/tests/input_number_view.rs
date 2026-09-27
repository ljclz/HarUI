//! R.2.P0.3 InputNumber view() 测试 — TDD RED 阶段
//!
//! 验证 InputNumber view() 正确组合 text_input + +/- 按钮。
//! 测试中所有消息统一使用 String 类型（实际应用中用 app 的 Message enum）。

use har_ui_components::input_number::{ControlsPosition, InputNumber, InputNumberMessage};
use har_ui_core::theme::Theme;

// ============== R.2.P0.3.a format_value：按 precision 格式化 ==============

#[test]
fn test_input_number_format_value_no_precision() {
    let inp = InputNumber::new();
    let s = inp.format_value();
    assert_eq!(s, "0");
}

#[test]
fn test_input_number_format_value_with_precision_2() {
    let inp = InputNumber::new().with_precision(2);
    let s = inp.format_value();
    assert_eq!(s, "0.00");
}

#[test]
fn test_input_number_format_value_negative_with_precision() {
    let mut inp = InputNumber::new().with_precision(2);
    inp.handle(InputNumberMessage::SetValue(-123.456));
    let s = inp.format_value();
    assert_eq!(s, "-123.46");
}

#[test]
fn test_input_number_format_value_integer_with_precision() {
    let mut inp = InputNumber::new().with_precision(3);
    inp.handle(InputNumberMessage::SetValue(5.0));
    let s = inp.format_value();
    assert_eq!(s, "5.000");
}

#[test]
fn test_input_number_format_value_integer_no_precision() {
    let mut inp = InputNumber::new();
    inp.handle(InputNumberMessage::SetValue(42.0));
    assert_eq!(inp.format_value(), "42");
}

#[test]
fn test_input_number_format_value_fraction_no_precision() {
    let mut inp = InputNumber::new();
    inp.handle(InputNumberMessage::SetValue(123.45));
    assert_eq!(inp.format_value(), "123.45");
}

// ============== R.2.P0.3.b can_increment / can_decrement（边界） ==============

#[test]
fn test_input_number_can_increment_no_max() {
    let inp = InputNumber::new();
    assert!(inp.can_increment());
}

#[test]
fn test_input_number_can_increment_at_max() {
    let mut inp = InputNumber::new().with_max(10.0);
    inp.handle(InputNumberMessage::SetValue(10.0));
    assert!(!inp.can_increment());
}

#[test]
fn test_input_number_can_decrement_no_min() {
    let inp = InputNumber::new();
    assert!(inp.can_decrement());
}

#[test]
fn test_input_number_can_decrement_at_min() {
    let mut inp = InputNumber::new().with_min(0.0);
    inp.handle(InputNumberMessage::SetValue(0.0));
    assert!(!inp.can_decrement());
}

#[test]
fn test_input_number_disabled_blocks_both() {
    let inp = InputNumber::new().disabled(true);
    assert!(!inp.can_increment());
    assert!(!inp.can_decrement());
}

// ============== R.2.P0.3.c view() 渲染不 panic ==============

#[test]
fn test_input_number_view_default_renders() {
    let theme = Theme::element_light();
    let inp = InputNumber::new();
    let _element = inp.view(&theme, |s| s, "inc".to_string(), "dec".to_string());
}

#[test]
fn test_input_number_view_with_precision_renders() {
    let theme = Theme::element_light();
    let inp = InputNumber::new().with_precision(2).with_step(0.1);
    let _element = inp.view(&theme, |s| s, "inc".to_string(), "dec".to_string());
}

#[test]
fn test_input_number_view_with_min_max_renders() {
    let theme = Theme::element_light();
    let inp = InputNumber::new().with_min(0.0).with_max(100.0);
    let _element = inp.view(&theme, |s| s, "inc".to_string(), "dec".to_string());
}

#[test]
fn test_input_number_view_disabled_renders() {
    let theme = Theme::element_light();
    let inp = InputNumber::new().disabled(true);
    let _element = inp.view(&theme, |s| s, "inc".to_string(), "dec".to_string());
}

#[test]
fn test_input_number_view_at_max_renders() {
    let theme = Theme::element_light();
    let mut inp = InputNumber::new().with_max(10.0);
    inp.handle(InputNumberMessage::SetValue(10.0));
    let _element = inp.view(&theme, |s| s, "inc".to_string(), "dec".to_string());
}

#[test]
fn test_input_number_view_at_min_renders() {
    let theme = Theme::element_light();
    let mut inp = InputNumber::new().with_min(0.0);
    inp.handle(InputNumberMessage::SetValue(0.0));
    let _element = inp.view(&theme, |s| s, "inc".to_string(), "dec".to_string());
}

#[test]
fn test_input_number_view_controls_position_right_renders() {
    let theme = Theme::element_light();
    let inp = InputNumber::new().with_controls_position(ControlsPosition::Right);
    let _element = inp.view(&theme, |s| s, "inc".to_string(), "dec".to_string());
}

#[test]
fn test_input_number_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let inp = InputNumber::new();
    let _element = inp.view(&theme, |s| s, "inc".to_string(), "dec".to_string());
}

#[test]
fn test_input_number_view_with_custom_message_type() {
    let theme = Theme::element_light();
    let inp = InputNumber::new();
    #[derive(Clone, Debug)]
    #[allow(dead_code)]
    enum AppMsg {
        Inc,
        Dec,
        Input(String),
    }
    let _element = inp.view(&theme, AppMsg::Input, AppMsg::Inc, AppMsg::Dec);
}
