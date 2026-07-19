//! Input 组件测试
//!
//! 参考 Element Plus `<el-input>` 组件 API。
//! 测试覆盖：
//! - 4 种 input mode: Text / Password / Digit / Price
//! - IME composition 三阶段 (compositionstart/update/end)
//! - clearable / prefix / suffix / maxlength
//! - 状态变化和事件派发

use har_ui_components::input::{
    Input, InputMessage, InputMode, InputState,
};
use har_ui_core::utils::ime::ImeEvent;

#[test]
fn test_input_default_mode_is_text() {
    let inp = Input::new();
    assert_eq!(inp.mode(), InputMode::Text);
}

#[test]
fn test_input_default_value_is_empty() {
    let inp = Input::new();
    assert_eq!(inp.value(), "");
}

#[test]
fn test_input_default_placeholder() {
    let inp = Input::new().with_placeholder("Please input");
    assert_eq!(inp.placeholder(), "Please input");
}

#[test]
fn test_input_text_mode_accepts_any_char() {
    let mut inp = Input::new().with_mode(InputMode::Text);
    inp.handle(InputMessage::Char('a'));
    inp.handle(InputMessage::Char('b'));
    inp.handle(InputMessage::Char('c'));
    assert_eq!(inp.value(), "abc");
}

#[test]
fn test_input_password_mode_accepts_any_char() {
    let mut inp = Input::new().with_mode(InputMode::Password);
    inp.handle(InputMessage::Char('a'));
    inp.handle(InputMessage::Char('@'));
    inp.handle(InputMessage::Char('1'));
    assert_eq!(inp.value(), "a@1");
}

#[test]
fn test_input_digit_mode_filters_non_digits() {
    let mut inp = Input::new().with_mode(InputMode::Digit);
    inp.handle(InputMessage::Char('5'));
    inp.handle(InputMessage::Char('x')); // 应被过滤
    inp.handle(InputMessage::Char('7'));
    assert_eq!(inp.value(), "57");
}

#[test]
fn test_input_price_mode_allows_two_decimal_places() {
    let mut inp = Input::new().with_mode(InputMode::Price);
    inp.handle(InputMessage::Char('1'));
    inp.handle(InputMessage::Char('2'));
    inp.handle(InputMessage::Char('.'));
    inp.handle(InputMessage::Char('3'));
    inp.handle(InputMessage::Char('4'));
    inp.handle(InputMessage::Char('5')); // 第三位小数应被过滤
    assert_eq!(inp.value(), "12.34");
}

#[test]
fn test_input_price_mode_allows_only_one_dot() {
    let mut inp = Input::new().with_mode(InputMode::Price);
    inp.handle(InputMessage::Char('1'));
    inp.handle(InputMessage::Char('.'));
    inp.handle(InputMessage::Char('.'));
    inp.handle(InputMessage::Char('2'));
    assert_eq!(inp.value(), "1.2");
}

#[test]
fn test_input_ime_composition_does_not_commit_text() {
    let mut inp = Input::new().with_mode(InputMode::Text);
    inp.handle(InputMessage::ImeEvent(ImeEvent::CompositionStart));
    inp.handle(InputMessage::ImeEvent(ImeEvent::CompositionUpdate("中".to_string())));
    inp.handle(InputMessage::ImeEvent(ImeEvent::CompositionUpdate("你好".to_string())));
    // 组合中：value 不应包含组合文本
    assert_eq!(inp.value(), "");
    assert_eq!(inp.composition_text(), "你好");
    assert_eq!(inp.state(), InputState::Composing);
}

#[test]
fn test_input_ime_composition_end_commits_text() {
    let mut inp = Input::new().with_mode(InputMode::Text);
    inp.handle(InputMessage::ImeEvent(ImeEvent::CompositionStart));
    inp.handle(InputMessage::ImeEvent(ImeEvent::CompositionUpdate("你好".to_string())));
    inp.handle(InputMessage::ImeEvent(ImeEvent::CompositionEnd("你好".to_string())));
    // 组合结束：value 应包含提交文本
    assert_eq!(inp.value(), "你好");
    assert_eq!(inp.composition_text(), "");
    // 组合结束后输入框仍保持焦点（不切回 Normal）
    assert_eq!(inp.state(), InputState::Focused);
}

#[test]
fn test_input_ime_composition_in_digit_mode_filters_non_digits() {
    let mut inp = Input::new().with_mode(InputMode::Digit);
    inp.handle(InputMessage::ImeEvent(ImeEvent::CompositionStart));
    inp.handle(InputMessage::ImeEvent(ImeEvent::CompositionUpdate("12a3".to_string())));
    inp.handle(InputMessage::ImeEvent(ImeEvent::CompositionEnd("12a3".to_string())));
    // Digit 模式下应仅保留数字
    assert_eq!(inp.value(), "123");
}

#[test]
fn test_input_clearable_clears_value() {
    let mut inp = Input::new().with_clearable(true);
    inp.handle(InputMessage::Char('a'));
    inp.handle(InputMessage::Char('b'));
    assert_eq!(inp.value(), "ab");
    // 派发 Clear 事件
    inp.handle(InputMessage::Clear);
    assert_eq!(inp.value(), "");
}

#[test]
fn test_input_maxlength_truncates() {
    let mut inp = Input::new().with_maxlength(5);
    for c in "abcdefg".chars() {
        inp.handle(InputMessage::Char(c));
    }
    assert_eq!(inp.value(), "abcde");
}

#[test]
fn test_input_prefix_and_suffix() {
    let inp = Input::new()
        .with_prefix("￥")
        .with_suffix("RMB");
    assert_eq!(inp.prefix(), Some("￥"));
    assert_eq!(inp.suffix(), Some("RMB"));
}

#[test]
fn test_input_state_transitions_focus() {
    let mut inp = Input::new();
    assert_eq!(inp.state(), InputState::Normal);
    inp.handle(InputMessage::Focused);
    assert_eq!(inp.state(), InputState::Focused);
    inp.handle(InputMessage::Blurred);
    assert_eq!(inp.state(), InputState::Normal);
}

#[test]
fn test_input_disabled_blocks_input() {
    let mut inp = Input::new().disabled(true);
    inp.handle(InputMessage::Char('a'));
    assert_eq!(inp.value(), "");
}

#[test]
fn test_input_backspace_removes_last_char() {
    let mut inp = Input::new();
    inp.handle(InputMessage::Char('h'));
    inp.handle(InputMessage::Char('i'));
    inp.handle(InputMessage::Backspace);
    assert_eq!(inp.value(), "h");
}

#[test]
fn test_input_builder_chains() {
    let inp = Input::new()
        .with_mode(InputMode::Price)
        .with_placeholder("0.00")
        .with_clearable(true)
        .with_maxlength(10)
        .with_prefix("$")
        .disabled(false);
    assert_eq!(inp.mode(), InputMode::Price);
    assert_eq!(inp.placeholder(), "0.00");
    assert!(inp.is_clearable());
    assert_eq!(inp.maxlength(), Some(10));
    assert_eq!(inp.prefix(), Some("$"));
    assert!(!inp.is_disabled());
}
