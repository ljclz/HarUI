//! Keypad 组件 — 数字键盘（POS 专用）
//!
//! 参考 POS 收银机的数字键盘。
//! 支持：0-9 / 00 / . / 退格 / 清除 / OK 确认，三种模式（price/number/quantity），
//! max/min 边界检查，触摸优化（按钮热区 ≥ 44x44px）。

use har_ui_core::theme::Theme;
use iced::widget::{button, container, text};
use iced::{Color, Element, Length, Padding};

/// 键盘模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KeypadMode {
    /// 价格：最多 2 位小数
    #[default]
    Price,
    /// 整数：无小数点
    Number,
    /// 数量：最多 3 位小数
    Quantity,
}

impl KeypadMode {
    /// 该模式下允许的小数位数（None 表示不允许小数）
    pub fn max_decimals(self) -> Option<usize> {
        match self {
            KeypadMode::Price => Some(2),
            KeypadMode::Number => None,
            KeypadMode::Quantity => Some(3),
        }
    }
}

/// 键盘状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KeypadState {
    #[default]
    Editing,
    Confirmed,
    Invalid,
}

/// 键盘消息
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeypadMessage {
    Digit(u8),  // 0-9
    DoubleZero, // 00
    Dot,        // 小数点
    Backspace,
    Clear,
    Ok,
}

/// 整数部分最大位数
const MAX_INT_LEN: usize = 8;

/// 键盘组件
#[derive(Debug, Clone)]
pub struct Keypad {
    value: String,
    mode: KeypadMode,
    state: KeypadState,
    max: Option<f64>,
    min: Option<f64>,
}

impl Keypad {
    pub fn new() -> Self {
        Self {
            value: String::new(),
            mode: KeypadMode::Price,
            state: KeypadState::Editing,
            max: None,
            min: None,
        }
    }

    pub fn with_mode(mut self, m: KeypadMode) -> Self {
        self.mode = m;
        self
    }

    pub fn with_max(mut self, v: f64) -> Self {
        self.max = Some(v);
        self
    }

    pub fn with_min(mut self, v: f64) -> Self {
        self.min = Some(v);
        self
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn mode(&self) -> KeypadMode {
        self.mode
    }

    pub fn state(&self) -> KeypadState {
        self.state
    }

    pub fn max(&self) -> Option<f64> {
        self.max
    }

    pub fn min(&self) -> Option<f64> {
        self.min
    }

    /// 数值化当前值（"0." 视为 0.0）
    pub fn numeric_value(&self) -> Option<f64> {
        if self.value.is_empty() {
            return None;
        }
        // 处理 "0." 这种尾点
        let s = if self.value.ends_with('.') {
            format!("{}0", self.value)
        } else {
            self.value.clone()
        };
        s.parse::<f64>().ok()
    }

    /// 触摸热区
    pub fn button_size(&self) -> (f32, f32) {
        (60.0, 60.0)
    }

    /// 重置
    pub fn reset(&mut self) {
        self.value.clear();
        self.state = KeypadState::Editing;
    }

    /// 处理消息
    pub fn handle(&mut self, msg: KeypadMessage) {
        // Confirmed/Invalid 状态下不接受除 Clear/Ok 外的输入
        match self.state {
            KeypadState::Confirmed | KeypadState::Invalid => match msg {
                KeypadMessage::Clear => {
                    self.reset();
                    return;
                }
                _ => return,
            },
            KeypadState::Editing => {}
        }

        match msg {
            KeypadMessage::Digit(d) => {
                let ch = char::from_digit(d as u32, 10).unwrap_or('0');
                self.push_digit(ch);
            }
            KeypadMessage::DoubleZero => self.push_double_zero(),
            KeypadMessage::Dot => self.push_dot(),
            KeypadMessage::Backspace => {
                self.value.pop();
            }
            KeypadMessage::Clear => {
                self.reset();
            }
            KeypadMessage::Ok => self.confirm(),
        }
    }

    /// 推入数字字符
    fn push_digit(&mut self, ch: char) {
        // 前导 0 处理：若当前值仅为 "0" 且输入非小数点后的 0，则替换
        if self.value == "0" {
            // 替换前导 0
            self.value.pop();
            self.value.push(ch);
            return;
        }
        // 检查整数部分长度
        if let Some(dot_pos) = self.value.find('.') {
            // 已有小数点，检查小数位数
            let decimals = self.value.len() - dot_pos - 1;
            let max_dec = self.mode.max_decimals().unwrap_or(0);
            if decimals >= max_dec {
                return; // 超出小数位数限制
            }
        } else {
            // 整数部分，检查长度
            if self.value.len() >= MAX_INT_LEN {
                return;
            }
        }
        self.value.push(ch);
    }

    fn push_double_zero(&mut self) {
        if let Some(dot_pos) = self.value.find('.') {
            let decimals = self.value.len() - dot_pos - 1;
            let max_dec = self.mode.max_decimals().unwrap_or(0);
            let remaining = max_dec.saturating_sub(decimals);
            // 推入剩余位数（最多 2 位）
            for _ in 0..remaining.min(2) {
                self.value.push('0');
            }
        } else {
            // 整数部分
            if self.value == "0" {
                // "0" + "00" 仍为 "0"
                return;
            }
            // 检查整数部分长度
            let remaining = MAX_INT_LEN.saturating_sub(self.value.len());
            for _ in 0..remaining.min(2) {
                self.value.push('0');
            }
        }
    }

    fn push_dot(&mut self) {
        // Number 模式不允许小数点
        if self.mode.max_decimals().is_none() {
            return;
        }
        // 已有小数点则忽略
        if self.value.contains('.') {
            return;
        }
        // 空值或 "0" 时补 0
        if self.value.is_empty() {
            self.value.push_str("0.");
        } else {
            self.value.push('.');
        }
    }

    fn confirm(&mut self) {
        if self.value.is_empty() {
            return;
        }
        let v = match self.numeric_value() {
            Some(v) => v,
            None => {
                self.state = KeypadState::Invalid;
                return;
            }
        };
        // 边界检查
        if let Some(max) = self.max
            && v > max
        {
            self.state = KeypadState::Invalid;
            return;
        }
        if let Some(min) = self.min
            && v < min
        {
            self.state = KeypadState::Invalid;
            return;
        }
        self.state = KeypadState::Confirmed;
    }

    /// 渲染 Keypad 为 iced::Element
    ///
    /// # 参数
    /// - `theme`: HarUI 主题引用
    /// - `on_key`: 用户按键时发出消息，参数为按键标签（如 "7"、"00"、"."、"Del"、"OK"）
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        on_key: impl Fn(String) -> Message + 'a,
    ) -> Element<'a, Message> {
        let primary = Color::from(theme.primary.base);
        let danger = Color::from(theme.danger.base);
        let success = Color::from(theme.success.base);
        let text_primary = Color::from(theme.neutral.text_primary);
        let text_secondary = Color::from(theme.neutral.text_secondary);
        let border_lighter = Color::from(theme.neutral.border_lighter);

        let make_key_btn = move |label: &str, color: Color| -> Element<'a, Message> {
            let msg = on_key(label.to_string());
            button(text(label.to_string()).color(color).size(18.0))
                .width(Length::Fill)
                .padding(Padding::from([12u16, 0u16]))
                .on_press(msg)
                .style(move |_t, _status| iced::widget::button::Style {
                    background: None,
                    text_color: color,
                    border: iced::Border {
                        color: border_lighter,
                        width: 1.0,
                        radius: iced::border::radius(4.0),
                    },
                    shadow: iced::Shadow::default(),
                })
                .into()
        };

        let row1 = iced::widget::Row::new()
            .push(make_key_btn("7", text_primary))
            .push(make_key_btn("8", text_primary))
            .push(make_key_btn("9", text_primary))
            .spacing(4);

        let row2 = iced::widget::Row::new()
            .push(make_key_btn("4", text_primary))
            .push(make_key_btn("5", text_primary))
            .push(make_key_btn("6", text_primary))
            .spacing(4);

        let row3 = iced::widget::Row::new()
            .push(make_key_btn("1", text_primary))
            .push(make_key_btn("2", text_primary))
            .push(make_key_btn("3", text_primary))
            .spacing(4);

        let allow_dot = self.mode.max_decimals().is_some();
        let mut row4 = iced::widget::Row::new()
            .push(make_key_btn("0", text_primary))
            .push(make_key_btn("00", text_primary));
        if allow_dot {
            row4 = row4.push(make_key_btn(".", text_primary));
        } else {
            let placeholder: Element<'a, Message> = container(text("")).width(Length::Fill).into();
            row4 = row4.push(placeholder);
        }
        row4 = row4.spacing(4);

        let row5 = iced::widget::Row::new()
            .push(make_key_btn("Del", danger))
            .push(make_key_btn("Clr", text_secondary))
            .push(make_key_btn("OK", success))
            .spacing(4);

        let mut col_children: Vec<Element<'a, Message>> = Vec::new();
        if !self.value.is_empty() {
            col_children.push(
                container(text(self.value.clone()).color(primary).size(20.0))
                    .width(Length::Fill)
                    .padding(Padding::from([8u16, 4u16]))
                    .style(move |_t| iced::widget::container::Style {
                        text_color: None,
                        background: None,
                        border: iced::Border {
                            color: border_lighter,
                            width: 1.0,
                            radius: iced::border::radius(4.0),
                        },
                        shadow: iced::Shadow::default(),
                    })
                    .into(),
            );
        }
        col_children.push(row1.into());
        col_children.push(row2.into());
        col_children.push(row3.into());
        col_children.push(row4.into());
        col_children.push(row5.into());

        iced::widget::Column::with_children(col_children)
            .spacing(4)
            .into()
    }
}

impl Default for Keypad {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_keypad_mode_decimals() {
        assert_eq!(KeypadMode::Price.max_decimals(), Some(2));
        assert_eq!(KeypadMode::Number.max_decimals(), None);
        assert_eq!(KeypadMode::Quantity.max_decimals(), Some(3));
    }

    #[test]
    fn test_keypad_default_state_fields() {
        let k = Keypad::new();
        assert_eq!(k.value(), "");
        assert_eq!(k.mode(), KeypadMode::Price);
        assert_eq!(k.state(), KeypadState::Editing);
        assert_eq!(k.max(), None);
        assert_eq!(k.min(), None);
    }

    #[test]
    fn test_keypad_numeric_value_parsing() {
        let mut k = Keypad::new();
        k.handle(KeypadMessage::Digit(1));
        k.handle(KeypadMessage::Dot);
        k.handle(KeypadMessage::Digit(2));
        assert!((k.numeric_value().unwrap() - 1.2).abs() < f64::EPSILON);
    }
}
