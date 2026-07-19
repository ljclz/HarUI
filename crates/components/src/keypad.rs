//! Keypad 组件 — 数字键盘（POS 专用）
//!
//! 参考 POS 收银机的数字键盘。
//! 支持：0-9 / 00 / . / 退格 / 清除 / OK 确认，三种模式（price/number/quantity），
//! max/min 边界检查，触摸优化（按钮热区 ≥ 44x44px）。

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
    Digit(u8),     // 0-9
    DoubleZero,    // 00
    Dot,           // 小数点
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
        if let Some(max) = self.max {
            if v > max {
                self.state = KeypadState::Invalid;
                return;
            }
        }
        if let Some(min) = self.min {
            if v < min {
                self.state = KeypadState::Invalid;
                return;
            }
        }
        self.state = KeypadState::Confirmed;
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
