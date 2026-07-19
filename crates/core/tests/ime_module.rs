//! IME 处理模块测试
//!
//! 验证 compositionstart/update/end 三阶段状态机，
//! 以及 Digit/Price 模式下对非数字字符的过滤行为。

use har_ui_core::utils::ime::{ImeState, ImeEvent, InputMode, ImeProcessor, ProcessResult};

#[test]
fn test_default_state_is_idle() {
    let proc = ImeProcessor::new();
    assert_eq!(proc.state(), ImeState::Idle);
    assert_eq!(proc.composition_text(), "");
}

#[test]
fn test_composition_start_transitions_to_composing() {
    let mut proc = ImeProcessor::new();
    let result = proc.handle_event(ImeEvent::CompositionStart);
    assert_eq!(proc.state(), ImeState::Composing);
    assert!(matches!(result, ProcessResult::StateChanged));
}

#[test]
fn test_composition_update_updates_text() {
    let mut proc = ImeProcessor::new();
    proc.handle_event(ImeEvent::CompositionStart);
    proc.handle_event(ImeEvent::CompositionUpdate("你好".to_string()));
    assert_eq!(proc.composition_text(), "你好");
    assert_eq!(proc.state(), ImeState::Composing);
}

#[test]
fn test_composition_end_returns_to_idle_and_commits() {
    let mut proc = ImeProcessor::new();
    proc.handle_event(ImeEvent::CompositionStart);
    proc.handle_event(ImeEvent::CompositionUpdate("你好".to_string()));
    let result = proc.handle_event(ImeEvent::CompositionEnd("你好".to_string()));
    assert_eq!(proc.state(), ImeState::Idle);
    assert_eq!(proc.composition_text(), "");
    assert!(matches!(result, ProcessResult::TextCommitted(ref s) if s == "你好"));
}

#[test]
fn test_on_change_not_fired_during_composition() {
    let mut proc = ImeProcessor::new();
    proc.handle_event(ImeEvent::CompositionStart);
    let result = proc.handle_event(ImeEvent::CompositionUpdate("中".to_string()));
    assert!(matches!(result, ProcessResult::StateChanged));
}

#[test]
fn test_idle_input_triggers_text_committed() {
    let mut proc = ImeProcessor::new();
    let result = proc.handle_event(ImeEvent::TextInput('a'));
    assert!(matches!(result, ProcessResult::TextCommitted(ref s) if s == "a"));
    assert_eq!(proc.state(), ImeState::Idle);
}

#[test]
fn test_digit_mode_filters_non_digit_chars() {
    let mut proc = ImeProcessor::with_mode(InputMode::Digit);
    let result = proc.handle_event(ImeEvent::TextInput('a'));
    assert!(matches!(result, ProcessResult::Filtered));
    let result = proc.handle_event(ImeEvent::TextInput('5'));
    assert!(matches!(result, ProcessResult::TextCommitted(ref s) if s == "5"));
}

#[test]
fn test_digit_mode_allows_decimal_point_once() {
    let mut proc = ImeProcessor::with_mode(InputMode::Digit);
    proc.handle_event(ImeEvent::TextInput('3'));
    let result = proc.handle_event(ImeEvent::TextInput('.'));
    assert!(matches!(result, ProcessResult::TextCommitted(ref s) if s == "."));
    let result = proc.handle_event(ImeEvent::TextInput('.'));
    assert!(matches!(result, ProcessResult::Filtered));
}

#[test]
fn test_price_mode_limits_two_decimal_places() {
    let mut proc = ImeProcessor::with_mode(InputMode::Price);
    proc.handle_event(ImeEvent::TextInput('1'));
    proc.handle_event(ImeEvent::TextInput('2'));
    proc.handle_event(ImeEvent::TextInput('.'));
    proc.handle_event(ImeEvent::TextInput('3'));
    proc.handle_event(ImeEvent::TextInput('4'));
    let result = proc.handle_event(ImeEvent::TextInput('5'));
    assert!(matches!(result, ProcessResult::Filtered));
}

#[test]
fn test_text_mode_allows_any_char() {
    let mut proc = ImeProcessor::with_mode(InputMode::Text);
    let result = proc.handle_event(ImeEvent::TextInput('x'));
    assert!(matches!(result, ProcessResult::TextCommitted(ref s) if s == "x"));
}

#[test]
fn test_password_mode_allows_any_char() {
    let mut proc = ImeProcessor::with_mode(InputMode::Password);
    let result = proc.handle_event(ImeEvent::TextInput('@'));
    assert!(matches!(result, ProcessResult::TextCommitted(ref s) if s == "@"));
}

#[test]
fn test_composition_end_in_digit_mode_filters_non_digits() {
    let mut proc = ImeProcessor::with_mode(InputMode::Digit);
    proc.handle_event(ImeEvent::CompositionStart);
    proc.handle_event(ImeEvent::CompositionUpdate("abc".to_string()));
    let result = proc.handle_event(ImeEvent::CompositionEnd("abc".to_string()));
    assert_eq!(proc.state(), ImeState::Idle);
    assert!(matches!(result, ProcessResult::Filtered));
}

#[test]
fn test_composition_end_in_digit_mode_keeps_digits() {
    let mut proc = ImeProcessor::with_mode(InputMode::Digit);
    proc.handle_event(ImeEvent::CompositionStart);
    proc.handle_event(ImeEvent::CompositionUpdate("12".to_string()));
    let result = proc.handle_event(ImeEvent::CompositionEnd("12".to_string()));
    assert!(matches!(result, ProcessResult::TextCommitted(ref s) if s == "12"));
}

#[test]
fn test_reset_clears_state() {
    let mut proc = ImeProcessor::new();
    proc.handle_event(ImeEvent::CompositionStart);
    proc.handle_event(ImeEvent::CompositionUpdate("中".to_string()));
    proc.reset();
    assert_eq!(proc.state(), ImeState::Idle);
    assert_eq!(proc.composition_text(), "");
}

#[test]
fn test_set_current_value_for_digit_filtering() {
    let mut proc = ImeProcessor::with_mode(InputMode::Digit);
    proc.set_current_value("123".to_string());
    let result = proc.handle_event(ImeEvent::TextInput('.'));
    assert!(matches!(result, ProcessResult::TextCommitted(_)));
    let result = proc.handle_event(ImeEvent::TextInput('.'));
    assert!(matches!(result, ProcessResult::Filtered));
}

#[test]
fn test_price_mode_allows_integer_input() {
    let mut proc = ImeProcessor::with_mode(InputMode::Price);
    let r1 = proc.handle_event(ImeEvent::TextInput('1'));
    let r2 = proc.handle_event(ImeEvent::TextInput('2'));
    let r3 = proc.handle_event(ImeEvent::TextInput('3'));
    assert!(matches!(r1, ProcessResult::TextCommitted(_)));
    assert!(matches!(r2, ProcessResult::TextCommitted(_)));
    assert!(matches!(r3, ProcessResult::TextCommitted(_)));
}
