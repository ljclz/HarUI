//! IME 处理模块
//!
//! 实现 compositionstart/update/end 三阶段状态机，
//! 在 IME 组合输入时不触发 onChange，组合结束后才提交文本。
//!
//! 同时支持 InputMode::Digit / Price 模式对非数字字符的过滤。
//!
//! ## 状态机
//!
//! ```text
//! ┌─────────┐  CompositionStart   ┌────────────┐
//! │  Idle   │ ─────────────────► │ Composing  │
//! └─────────┘                    └────────────┘
//!      ▲                              │
//!      │      CompositionEnd          │
//!      └──────────────────────────────┘
//!              (提交文本)
//! ```
//!
//! ## R.3 IME 事件桥接（subscription）
//!
//! [`subscription`] 提供 iced 事件流到 IME 事件的桥接，输出 [`ImeBridgeEvent`]。
//!
//! ### iced 0.13.x 平台限制
//! - `iced::window::Event` 在 0.13.x **不暴露 `Ime` 变体**
//!   （源码确认：`iced_core-0.13.2/src/window/event.rs` 与
//!   `iced_winit-0.13.0/src/conversion.rs` 中 `WindowEvent::Ime` 落入 `_ => None`）
//! - 因此 `Enabled` / `Disabled` / `Preedit` 三类事件目前**无法从 iced 事件流捕获**
//! - 仅 `Commit` 变体可触发：通过 `keyboard::Event::KeyPressed { text: Some(s), .. }`
//!   中含非 ASCII 字符（典型 CJK 输入）推断 IME 提交
//! - 完整 IME 支持（含 Preedit 显示）由 `text_input` widget 在 winit 层内部处理
//!
//! ### R.3.4 真机测试说明
//! - 本模块的单元测试仅验证类型可构造与函数签名
//! - 中文输入真机测试需在 Windows / macOS / Linux 上运行 demo 程序手动验证：
//!   1. 焦点进入 Input 组件
//!   2. 切换到中文输入法（如搜狗 / 微软拼音 / macOS 自带拼音）
//!   3. 输入拼音并选词，验证 Input.value 正确更新且组合期不触发 onChange
//!   4. 验证 Digit / Price 模式下 IME 提交的非数字字符被过滤

/// IME 状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImeState {
    /// 空闲状态：未在 IME 组合输入中
    Idle,
    /// 组合状态：IME 正在输入（如中文输入法选词阶段）
    Composing,
}

/// 输入模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    /// 普通文本：任意字符
    Text,
    /// 密码：任意字符（不显示明文）
    Password,
    /// 数字：仅允许 [0-9]
    Digit,
    /// 价格：[0-9] + 最多 1 个小数点 + 最多 2 位小数
    Price,
}

/// IME 事件
#[derive(Debug, Clone, PartialEq)]
pub enum ImeEvent {
    /// IME 开始组合输入
    CompositionStart,
    /// IME 组合中文本更新
    CompositionUpdate(String),
    /// IME 组合结束，提交最终文本
    CompositionEnd(String),
    /// 直接字符输入（非 IME）
    TextInput(char),
}

/// 事件处理结果
#[derive(Debug, Clone, PartialEq)]
pub enum ProcessResult {
    /// 无状态变化或文本
    NoChange,
    /// 状态已变化（如 Idle ↔ Composing），UI 需要刷新光标/状态
    StateChanged,
    /// 文本已提交（应该触发 onChange）
    TextCommitted(String),
    /// 输入被过滤（如 Digit 模式下的字母）
    Filtered,
}

/// IME 处理器
#[derive(Debug, Clone)]
pub struct ImeProcessor {
    state: ImeState,
    mode: InputMode,
    composition_text: String,
    /// 当前输入框中的文本（用于 Digit/Price 过滤判断）
    current_value: String,
}

impl ImeProcessor {
    /// 创建默认处理器（Text 模式）
    pub fn new() -> Self {
        Self {
            state: ImeState::Idle,
            mode: InputMode::Text,
            composition_text: String::new(),
            current_value: String::new(),
        }
    }

    /// 创建指定模式的处理器
    pub fn with_mode(mode: InputMode) -> Self {
        Self {
            state: ImeState::Idle,
            mode,
            composition_text: String::new(),
            current_value: String::new(),
        }
    }

    /// 当前 IME 状态
    pub fn state(&self) -> ImeState {
        self.state
    }

    /// 当前组合中的文本
    pub fn composition_text(&self) -> &str {
        &self.composition_text
    }

    /// 当前输入模式
    pub fn mode(&self) -> InputMode {
        self.mode
    }

    /// 设置当前输入框文本（供过滤判断使用）
    pub fn set_current_value(&mut self, value: String) {
        self.current_value = value;
    }

    /// 处理 IME 事件
    pub fn handle_event(&mut self, event: ImeEvent) -> ProcessResult {
        match event {
            ImeEvent::CompositionStart => {
                if self.state == ImeState::Idle {
                    self.state = ImeState::Composing;
                    self.composition_text.clear();
                    ProcessResult::StateChanged
                } else {
                    ProcessResult::NoChange
                }
            }
            ImeEvent::CompositionUpdate(text) => {
                if self.state == ImeState::Composing {
                    self.composition_text = text;
                    ProcessResult::StateChanged
                } else {
                    ProcessResult::NoChange
                }
            }
            ImeEvent::CompositionEnd(text) => {
                if self.state == ImeState::Composing {
                    self.state = ImeState::Idle;
                    self.composition_text.clear();
                    // 根据模式过滤提交的文本
                    let filtered = self.filter_text(&text);
                    if filtered.is_empty() && !text.is_empty() {
                        ProcessResult::Filtered
                    } else {
                        ProcessResult::TextCommitted(filtered)
                    }
                } else {
                    ProcessResult::NoChange
                }
            }
            ImeEvent::TextInput(ch) => {
                if self.state == ImeState::Composing {
                    // 组合状态下不应有直接字符输入，忽略
                    return ProcessResult::NoChange;
                }
                // 根据模式过滤单个字符
                if self.is_char_allowed(ch) {
                    // 自动维护 current_value 以正确判断小数位数/已有小数点
                    self.current_value.push(ch);
                    ProcessResult::TextCommitted(ch.to_string())
                } else {
                    ProcessResult::Filtered
                }
            }
        }
    }

    /// 重置状态
    pub fn reset(&mut self) {
        self.state = ImeState::Idle;
        self.composition_text.clear();
    }

    /// 判断字符是否被当前模式允许
    fn is_char_allowed(&self, ch: char) -> bool {
        match self.mode {
            InputMode::Text | InputMode::Password => true,
            InputMode::Digit => {
                if ch.is_ascii_digit() {
                    return true;
                }
                // Digit 模式允许一次小数点
                if ch == '.' {
                    return !self.current_value.contains('.');
                }
                false
            }
            InputMode::Price => {
                if ch.is_ascii_digit() {
                    // 已有小数点时，限制小数位数 ≤ 2
                    if let Some(dot_pos) = self.current_value.find('.') {
                        let decimal_count = self.current_value[dot_pos + 1..].chars().count();
                        return decimal_count < 2;
                    }
                    return true;
                }
                // Price 模式允许一次小数点
                if ch == '.' {
                    return !self.current_value.contains('.');
                }
                false
            }
        }
    }

    /// 过滤组合提交的文本
    fn filter_text(&self, text: &str) -> String {
        match self.mode {
            InputMode::Text | InputMode::Password => text.to_string(),
            InputMode::Digit => text.chars().filter(|c| c.is_ascii_digit()).collect(),
            InputMode::Price => {
                // 保留数字和最多一个小数点 + 最多 2 位小数
                let mut result = String::new();
                let mut has_dot = self.current_value.contains('.');
                let mut decimal_count = if has_dot {
                    self.current_value
                        .split('.')
                        .nth(1)
                        .map(|s| s.chars().count())
                        .unwrap_or(0)
                } else {
                    0
                };

                for ch in text.chars() {
                    if ch.is_ascii_digit() {
                        if has_dot && decimal_count >= 2 {
                            continue;
                        }
                        result.push(ch);
                        if has_dot {
                            decimal_count += 1;
                        }
                    } else if ch == '.' && !has_dot {
                        result.push(ch);
                        has_dot = true;
                    }
                }
                result
            }
        }
    }
}

impl Default for ImeProcessor {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// R.3.1 IME 事件桥接（iced Subscription）
// ============================================================================

use iced::advanced::widget::Id as WidgetId;
use iced::event::{self, Event, Status};
use iced::keyboard;
use iced::window::Id as WindowId;
use iced::Subscription;

/// IME 桥接事件（R.3.1）
///
/// 通过 iced 事件流捕获的 IME 相关事件，携带目标 widget 的 [`WidgetId`]。
///
/// ## iced 0.13.x 实际触发情况
/// - `Commit`：当 `keyboard::Event::KeyPressed` 的 `text` 字段含非 ASCII 字符时触发
///   （CJK 输入法提交文本的典型特征）
/// - `Enabled` / `Disabled` / `Preedit`：iced 0.13.x 不暴露 `window::Event::Ime`，
///   这三个变体保留为 API 占位，等待 iced 后续版本支持
#[derive(Debug, Clone, PartialEq)]
pub enum ImeBridgeEvent {
    /// IME 启用（占位，iced 0.13.x 暂不触发）
    Enabled { id: WidgetId },
    /// IME 禁用（占位，iced 0.13.x 暂不触发）
    Disabled { id: WidgetId },
    /// 预编辑文本（占位，iced 0.13.x 暂不触发）
    Preedit {
        id: WidgetId,
        text: String,
        cursor: Option<(usize, usize)>,
    },
    /// 提交文本（通过键盘事件 text 字段含非 ASCII 字符推断）
    Commit { id: WidgetId, text: String },
}

/// 默认 IME 桥接 widget Id（哨兵值）
///
/// iced 0.13.x 事件流不携带目标 widget 信息，使用此哨兵作为占位。
/// 调用方可根据自身应用上下文映射到具体的 Input widget Id。
pub const DEFAULT_IME_ID: &str = "har-ui-ime";

/// IME 事件订阅（R.3.1）
///
/// 返回 [`Subscription<ImeBridgeEvent>`]，接入 iced 事件流并过滤 IME 相关事件。
///
/// ## 实现细节
/// 通过 [`iced::event::listen_with`] 订阅所有 iced 事件，过滤
/// `keyboard::Event::KeyPressed { text: Some(s), .. }` 且 `s` 含非 ASCII 字符的事件，
/// 转换为 [`ImeBridgeEvent::Commit`]。
///
/// ## 限制
/// iced 0.13.x 在 winit 转换层未暴露 `WindowEvent::Ime`，因此无法捕获
/// `Enabled` / `Disabled` / `Preedit` 事件。完整 IME 支持由 `text_input` widget
/// 在 winit 层内部处理，无需应用层订阅。
pub fn subscription() -> Subscription<ImeBridgeEvent> {
    event::listen_with(filter_ime_event)
}

/// `listen_with` 的过滤函数（函数指针，非闭包，符合 iced 0.13.x 签名约束）
fn filter_ime_event(
    event: Event,
    _status: Status,
    _window: WindowId,
) -> Option<ImeBridgeEvent> {
    match event {
        Event::Keyboard(keyboard::Event::KeyPressed {
            text: Some(text), ..
        }) if text.chars().any(|c| !c.is_ascii()) => Some(ImeBridgeEvent::Commit {
            id: WidgetId::new(DEFAULT_IME_ID),
            text: text.to_string(),
        }),
        _ => None,
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_default_mode_is_text() {
        let proc = ImeProcessor::new();
        assert_eq!(proc.mode(), InputMode::Text);
    }

    #[test]
    fn test_with_mode_sets_mode() {
        let proc = ImeProcessor::with_mode(InputMode::Digit);
        assert_eq!(proc.mode(), InputMode::Digit);
    }
}
