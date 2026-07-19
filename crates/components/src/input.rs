//! Input 组件 — 输入框
//!
//! 参考 Element Plus `<el-input>` 组件。
//! 集成 har-ui-core 的 ImeProcessor 处理 IME 组合输入。

use har_ui_core::utils::ime::{ImeEvent, ImeProcessor, InputMode as CoreInputMode, ProcessResult};

/// 输入框模式（包装 CoreInputMode 以便公开 API）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputMode {
    #[default]
    Text,
    Password,
    Digit,
    Price,
}

impl InputMode {
    fn to_core(self) -> CoreInputMode {
        match self {
            InputMode::Text => CoreInputMode::Text,
            InputMode::Password => CoreInputMode::Password,
            InputMode::Digit => CoreInputMode::Digit,
            InputMode::Price => CoreInputMode::Price,
        }
    }
}

/// 输入框交互状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputState {
    #[default]
    Normal,
    Focused,
    Composing,
}

/// Input 消息（事件）
#[derive(Debug, Clone, PartialEq)]
pub enum InputMessage {
    /// 直接字符输入（非 IME）
    Char(char),
    /// 退格
    Backspace,
    /// 清空
    Clear,
    /// 获得焦点
    Focused,
    /// 失去焦点
    Blurred,
    /// IME 事件
    ImeEvent(ImeEvent),
}

/// Input 组件
#[derive(Debug, Clone)]
pub struct Input {
    value: String,
    placeholder: String,
    mode: InputMode,
    state: InputState,
    disabled: bool,
    clearable: bool,
    maxlength: Option<usize>,
    prefix: Option<String>,
    suffix: Option<String>,
    ime: ImeProcessor,
}

impl Input {
    pub fn new() -> Self {
        Self {
            value: String::new(),
            placeholder: String::new(),
            mode: InputMode::Text,
            state: InputState::Normal,
            disabled: false,
            clearable: false,
            maxlength: None,
            prefix: None,
            suffix: None,
            ime: ImeProcessor::new(),
        }
    }

    pub fn with_mode(mut self, mode: InputMode) -> Self {
        self.mode = mode;
        self.ime = ImeProcessor::with_mode(mode.to_core());
        self
    }

    pub fn with_placeholder(mut self, p: impl Into<String>) -> Self {
        self.placeholder = p.into();
        self
    }

    pub fn with_clearable(mut self, v: bool) -> Self {
        self.clearable = v;
        self
    }

    pub fn with_maxlength(mut self, n: usize) -> Self {
        self.maxlength = Some(n);
        self
    }

    pub fn with_prefix(mut self, p: impl Into<String>) -> Self {
        self.prefix = Some(p.into());
        self
    }

    pub fn with_suffix(mut self, s: impl Into<String>) -> Self {
        self.suffix = Some(s.into());
        self
    }

    pub fn disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn placeholder(&self) -> &str {
        &self.placeholder
    }

    pub fn mode(&self) -> InputMode {
        self.mode
    }

    pub fn state(&self) -> InputState {
        self.state
    }

    pub fn is_clearable(&self) -> bool {
        self.clearable
    }

    pub fn is_disabled(&self) -> bool {
        self.disabled
    }

    pub fn maxlength(&self) -> Option<usize> {
        self.maxlength
    }

    pub fn prefix(&self) -> Option<&str> {
        self.prefix.as_deref()
    }

    pub fn suffix(&self) -> Option<&str> {
        self.suffix.as_deref()
    }

    pub fn composition_text(&self) -> &str {
        self.ime.composition_text()
    }

    /// 处理消息
    pub fn handle(&mut self, msg: InputMessage) {
        if self.disabled {
            return;
        }

        match msg {
            InputMessage::Char(ch) => {
                if self.state == InputState::Composing {
                    // 组合中不接受直接字符
                    return;
                }
                // maxlength 检查
                if let Some(max) = self.maxlength {
                    if self.value.chars().count() >= max {
                        return;
                    }
                }
                // 委托给 ImeProcessor 处理
                self.ime.set_current_value(self.value.clone());
                let result = self.ime.handle_event(ImeEvent::TextInput(ch));
                if let ProcessResult::TextCommitted(s) = result {
                    self.value.push_str(&s);
                }
            }
            InputMessage::Backspace => {
                if self.state == InputState::Composing {
                    return;
                }
                self.value.pop();
            }
            InputMessage::Clear => {
                self.value.clear();
            }
            InputMessage::Focused => {
                if self.state != InputState::Composing {
                    self.state = InputState::Focused;
                }
            }
            InputMessage::Blurred => {
                if self.state != InputState::Composing {
                    self.state = InputState::Normal;
                }
            }
            InputMessage::ImeEvent(event) => {
                let result = self.ime.handle_event(event.clone());
                match &event {
                    ImeEvent::CompositionStart => {
                        self.state = InputState::Composing;
                    }
                    ImeEvent::CompositionEnd(_text) => {
                        self.state = InputState::Focused;
                        if let ProcessResult::TextCommitted(s) = result {
                            // maxlength 检查（按字符数）
                            if let Some(max) = self.maxlength {
                                let remaining = max.saturating_sub(self.value.chars().count());
                                let truncated: String = s.chars().take(remaining).collect();
                                self.value.push_str(&truncated);
                            } else {
                                self.value.push_str(&s);
                            }
                        }
                        // 同步 current_value
                        self.ime.set_current_value(self.value.clone());
                    }
                    _ => {}
                }
            }
        }
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_default_state_normal() {
        let inp = Input::new();
        assert_eq!(inp.state(), InputState::Normal);
        assert!(!inp.is_disabled());
        assert!(!inp.is_clearable());
        assert_eq!(inp.maxlength(), None);
    }

    #[test]
    fn test_mode_conversion() {
        assert_eq!(InputMode::Text.to_core(), CoreInputMode::Text);
        assert_eq!(InputMode::Password.to_core(), CoreInputMode::Password);
        assert_eq!(InputMode::Digit.to_core(), CoreInputMode::Digit);
        assert_eq!(InputMode::Price.to_core(), CoreInputMode::Price);
    }
}
