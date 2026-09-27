//! ColorPicker 颜色选择 — 参考 Element Plus `<el-color-picker>`。
//!
//! 支持：color、alpha、show_alpha、disabled、format、predefine、panel、clear。

use har_ui_core::theme::Theme;
use iced::widget::{container, text};
use iced::{Color, Element, Length, Padding};

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

    /// 渲染 ColorPicker 为 iced::Element（纯展示）
    ///
    /// 渲染当前颜色块 + 下拉箭头；若 `panel_visible` 且存在 predefine 则展开色板网格。
    ///
    /// # 参数
    /// - `theme`: HarUI 主题引用
    pub fn view<'a>(&'a self, theme: &'a Theme) -> Element<'a, ()> {
        let text_primary = Color::from(theme.neutral.text_primary);
        let text_regular = Color::from(theme.neutral.text_regular);
        let text_placeholder = Color::from(theme.neutral.text_placeholder);
        let text_disabled = Color::from(theme.neutral.text_disabled);
        let border_lighter = Color::from(theme.neutral.border_lighter);
        let bg_overlay = Color::from(theme.neutral.bg_overlay);

        let has_color = !self.color.is_empty();
        let current_color = if has_color {
            parse_hex_color(&self.color, self.alpha)
        } else {
            Color::from_rgb8(238, 238, 238)
        };

        let swatch_border_color = if self.disabled {
            text_disabled
        } else {
            border_lighter
        };
        let swatch = container(text(""))
            .width(Length::Fixed(28.0))
            .height(Length::Fixed(28.0))
            .style(move |_t| iced::widget::container::Style {
                text_color: None,
                background: Some(iced::Background::Color(current_color)),
                border: iced::Border {
                    color: swatch_border_color,
                    width: 1.0,
                    radius: iced::border::radius(4.0),
                },
                shadow: iced::Shadow::default(),
            });

        let arrow_color = if self.disabled {
            text_disabled
        } else {
            text_regular
        };
        let trigger = iced::widget::Row::new()
            .push(swatch)
            .push(iced::widget::Space::with_width(Length::Fixed(6.0)))
            .push(text("▼").color(arrow_color).size(12.0))
            .align_y(iced::Alignment::Center);

        let trigger_text_color = if self.disabled {
            text_disabled
        } else {
            text_primary
        };
        let trigger_wrap = container(trigger)
            .width(Length::Fill)
            .padding(Padding::from([4u16, 8u16]))
            .style(move |_t| iced::widget::container::Style {
                text_color: Some(trigger_text_color),
                background: Some(iced::Background::Color(bg_overlay)),
                border: iced::Border {
                    color: border_lighter,
                    width: 1.0,
                    radius: iced::border::radius(4.0),
                },
                shadow: iced::Shadow::default(),
            });

        let mut outer_children: Vec<Element<'a, ()>> = Vec::new();
        outer_children.push(trigger_wrap.into());

        // 面板：预定义色板网格
        if self.panel_visible && !self.predefine.is_empty() {
            let mut grid_children: Vec<Element<'a, ()>> = Vec::new();
            for c in &self.predefine {
                let parsed = parse_hex_color(c, 255);
                let cell = container(text(""))
                    .width(Length::Fixed(24.0))
                    .height(Length::Fixed(24.0))
                    .style(move |_t| iced::widget::container::Style {
                        text_color: None,
                        background: Some(iced::Background::Color(parsed)),
                        border: iced::Border {
                            color: border_lighter,
                            width: 1.0,
                            radius: iced::border::radius(3.0),
                        },
                        shadow: iced::Shadow::default(),
                    });
                grid_children.push(cell.into());
            }
            let grid = iced::widget::Row::with_children(grid_children)
                .spacing(4)
                .align_y(iced::Alignment::Center);
            let panel = container(grid)
                .width(Length::Fill)
                .padding(Padding::from(8u16))
                .style(move |_t| iced::widget::container::Style {
                    text_color: Some(text_primary),
                    background: Some(iced::Background::Color(bg_overlay)),
                    border: iced::Border {
                        color: border_lighter,
                        width: 1.0,
                        radius: iced::border::radius(4.0),
                    },
                    shadow: iced::Shadow::default(),
                });
            outer_children.push(panel.into());
        }

        let _ = text_placeholder;

        container(iced::widget::Column::with_children(outer_children).spacing(4))
            .width(Length::Fill)
            .into()
    }
}

/// 解析 hex 颜色字符串为 iced::Color
/// 支持 #RGB / #RRGGBB / #RRGGBBAA；无效或空字符串返回 (0,0,0) 黑色
/// alpha 参数为 u8（0-255），内部转为 f32（0.0-1.0）
fn parse_hex_color(s: &str, alpha: u8) -> Color {
    let alpha_f = alpha as f32 / 255.0;
    if s.is_empty() {
        return Color::from_rgba8(0, 0, 0, alpha_f);
    }
    let s = s.strip_prefix('#').unwrap_or(s);
    let (r, g, b, a) = match s.len() {
        3 => {
            let r = u8::from_str_radix(&s[0..1].repeat(2), 16).unwrap_or(0);
            let g = u8::from_str_radix(&s[1..2].repeat(2), 16).unwrap_or(0);
            let b = u8::from_str_radix(&s[2..3].repeat(2), 16).unwrap_or(0);
            (r, g, b, alpha_f)
        }
        6 => {
            let r = u8::from_str_radix(&s[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&s[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&s[4..6], 16).unwrap_or(0);
            (r, g, b, alpha_f)
        }
        8 => {
            let r = u8::from_str_radix(&s[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&s[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&s[4..6], 16).unwrap_or(0);
            let a_byte = u8::from_str_radix(&s[6..8], 16).unwrap_or(alpha);
            (r, g, b, a_byte as f32 / 255.0)
        }
        _ => return Color::from_rgba8(0, 0, 0, alpha_f),
    };
    Color::from_rgba8(r, g, b, a)
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
