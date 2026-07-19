//! ColorPicker 颜色选择 — 参考 Element Plus `<el-color-picker>`。
//!
//! 支持：color、alpha、show_alpha、disabled、format、predefine、panel、clear。

/// 颜色格式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorFormat {
    #[default]
    Hex,
    Rgb,
    Hsl,
    Hsv,
}

/// ColorPicker 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ColorPickerMessage {
    /// 设置颜色（无效字符串将被忽略）
    SetColor(String),
    /// 设置 alpha（0-255，自动钳制）
    SetAlpha(i32),
    /// 切换面板可见
    TogglePanel,
    /// 清空（颜色置空、alpha 复位 255）
    Clear,
    /// 选择预定义颜色（越界则无操作）
    SelectPredefine(usize),
}

/// ColorPicker 组件
#[derive(Debug, Clone)]
pub struct ColorPicker {
    color: String,
    alpha: u8,
    show_alpha: bool,
    disabled: bool,
    format: ColorFormat,
    predefine: Vec<String>,
    panel_visible: bool,
}

impl Default for ColorPicker {
    fn default() -> Self {
        Self::new()
    }
}

impl ColorPicker {
    pub fn new() -> Self {
        Self {
            color: String::new(),
            alpha: 255,
            show_alpha: false,
            disabled: false,
            format: ColorFormat::Hex,
            predefine: Vec::new(),
            panel_visible: false,
        }
    }

    pub fn with_show_alpha(mut self, v: bool) -> Self {
        self.show_alpha = v;
        self
    }

    pub fn with_disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }

    pub fn with_format(mut self, f: ColorFormat) -> Self {
        self.format = f;
        self
    }

    pub fn with_predefine(mut self, p: Vec<String>) -> Self {
        self.predefine = p;
        self
    }

    pub fn color(&self) -> &str {
        &self.color
    }

    pub fn alpha(&self) -> u8 {
        self.alpha
    }

    pub fn show_alpha(&self) -> bool {
        self.show_alpha
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn format(&self) -> ColorFormat {
        self.format
    }

    pub fn predefine(&self) -> &[String] {
        &self.predefine
    }

    pub fn panel_visible(&self) -> bool {
        self.panel_visible
    }

    pub fn handle(&mut self, msg: ColorPickerMessage) {
        match msg {
            ColorPickerMessage::SetColor(c) => {
                if self.disabled {
                    return;
                }
                if is_valid_hex_color(&c) {
                    self.color = c;
                }
                // 无效则忽略
            }
            ColorPickerMessage::SetAlpha(a) => {
                if self.disabled {
                    return;
                }
                self.alpha = a.clamp(0, 255) as u8;
            }
            ColorPickerMessage::TogglePanel => {
                if self.disabled {
                    return;
                }
                self.panel_visible = !self.panel_visible;
            }
            ColorPickerMessage::Clear => {
                self.color.clear();
                self.alpha = 255;
            }
            ColorPickerMessage::SelectPredefine(idx) => {
                if self.disabled {
                    return;
                }
                if let Some(c) = self.predefine.get(idx) {
                    self.color = c.clone();
                }
            }
        }
    }
}

/// 验证 hex 颜色字符串
/// 支持 #RGB / #RRGGBB / #RRGGBBAA
fn is_valid_hex_color(s: &str) -> bool {
    let s = s.strip_prefix('#').unwrap_or(s);
    let len = s.len();
    if !matches!(len, 3 | 6 | 8) {
        return false;
    }
    s.chars().all(|c| c.is_ascii_hexdigit())
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_valid_hex_3() {
        assert!(is_valid_hex_color("#abc"));
        assert!(is_valid_hex_color("abc"));
    }

    #[test]
    fn test_valid_hex_6() {
        assert!(is_valid_hex_color("#aabbcc"));
    }

    #[test]
    fn test_valid_hex_8() {
        assert!(is_valid_hex_color("#aabbcc80"));
    }

    #[test]
    fn test_invalid_hex_wrong_len() {
        assert!(!is_valid_hex_color("#abcd"));
        assert!(!is_valid_hex_color("#abcdefg"));
    }

    #[test]
    fn test_invalid_hex_non_hex() {
        assert!(!is_valid_hex_color("xyz"));
    }
}
