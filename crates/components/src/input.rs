//! Input 组件 — 输入框
//!
//! 参考 Element Plus `<el-input>` 组件。
//! 集成 har-ui-core 的 ImeProcessor 处理 IME 组合输入。

use har_ui_core::theme::Theme;
use har_ui_core::theme::style_sheets;
use har_ui_core::utils::ime::{ImeEvent, ImeProcessor, InputMode as CoreInputMode, ProcessResult};
use iced::Element;
use iced::widget::{text, text_input};

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

    /// 是否为密码模式（secure 显示）
    pub fn is_secure(&self) -> bool {
        matches!(self.mode, InputMode::Password)
    }

    /// 计算完整的 text_input::Style
    ///
    /// 委托给 `style_sheets::input_style`，按 status 返回对应样式。
    pub fn compute_style(&self, theme: &Theme, status: text_input::Status) -> text_input::Style {
        style_sheets::input_style(theme, status)
    }

    /// 渲染输入框为 iced::Element
    ///
    /// # 参数
    /// - `theme`: HarUI 主题引用
    /// - `on_input`: 文本变化时发出的消息构造器（接收新字符串）
    ///
    /// # 行为
    /// - Password 模式：`secure(true)` 隐藏字符
    /// - `disabled` 时不附加 on_input，iced 自动设置 Status::Disabled
    /// - 有 prefix/suffix 时用 row 包裹 text_input
    /// - style 闭包按 status 动态计算样式
    ///
    /// # R.3.3 IME 支持
    /// iced 0.13.x 的 `text_input` widget 在 winit 层**已内置 IME 支持**：
    /// - 平台输入法（中文 / 日文 / 韩文等）的 Preedit / Commit 由 winit 直接交付给
    ///   `text_input` widget 内部处理，无需应用层订阅 IME 事件流
    /// - 应用层 [`har_ui_core::utils::ime::subscription`] 仅作为事件桥接占位，
    ///   当前 iced 0.13.x 不暴露 `window::Event::Ime`，无法在应用层捕获完整 IME 事件
    /// - 本组件通过 [`ImeProcessor`] 维护 `Composing` 状态，组合期间显示
    ///   `composition_text` 而非 `value`，组合结束（`CompositionEnd`）后才提交文本
    /// - 真机中文输入测试参见 `crates/core/src/utils/ime.rs` 顶部文档
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        on_input: impl Fn(String) -> Message + 'a,
    ) -> Element<'a, Message> {
        // 组合中显示 composition_text，否则显示 value
        let display_value = if self.state == InputState::Composing {
            self.ime.composition_text()
        } else {
            &self.value
        };

        let mut ti = text_input(&self.placeholder, display_value).secure(self.is_secure());

        if !self.disabled {
            ti = ti.on_input(on_input);
        }

        ti = ti.style(move |_t, status| style_sheets::input_style(theme, status));

        // prefix/suffix 用 row 包裹
        let has_prefix = self.prefix.is_some();
        let has_suffix = self.suffix.is_some();
        if !has_prefix && !has_suffix {
            return ti.into();
        }

        let mut children: Vec<Element<'a, Message>> = Vec::with_capacity(3);
        if let Some(p) = &self.prefix {
            children.push(text(p).into());
        }
        children.push(ti.into());
        if let Some(s) = &self.suffix {
            children.push(text(s).into());
        }

        // row 接受可变参数，用 row! 宏更简洁；这里用数组方式避免宏的复杂泛型
        let r = iced::widget::Row::with_children(children)
            .align_y(iced::Alignment::Center)
            .spacing(4);
        Element::from(r)
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
                if let Some(max) = self.maxlength
                    && self.value.chars().count() >= max
                {
                    return;
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
