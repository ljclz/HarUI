//! Badge 组件 — 徽章
//!
//! 参考 Element Plus `<el-badge>`。
//! 支持：数字/小红点/文本、max 溢出（显示 max+）、is-dot、4 种位置、zero 自动隐藏。

use har_ui_core::theme::Theme;
use iced::widget::{container, text};
use iced::{Color, Element, Length};

/// Badge 值类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BadgeValue {
    Number(u32),
    Text(String),
}

/// Badge 位置
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BadgePosition {
    #[default]
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
}

/// Badge 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BadgeMessage {
    Update(BadgeValue),
    SetHidden(bool),
}

/// Badge 组件
#[derive(Debug, Clone)]
pub struct Badge {
    value: BadgeValue,
    max: u32,
    is_dot: bool,
    position: BadgePosition,
    hidden_override: Option<bool>,
}

impl Badge {
    pub fn new(value: BadgeValue) -> Self {
        Self {
            value,
            max: 99,
            is_dot: false,
            position: BadgePosition::TopRight,
            hidden_override: None,
        }
    }

    pub fn with_max(mut self, m: u32) -> Self {
        self.max = m;
        self
    }

    pub fn with_is_dot(mut self, v: bool) -> Self {
        self.is_dot = v;
        self
    }

    pub fn with_position(mut self, p: BadgePosition) -> Self {
        self.position = p;
        self
    }

    pub fn value(&self) -> &BadgeValue {
        &self.value
    }

    pub fn max(&self) -> u32 {
        self.max
    }

    pub fn is_dot(&self) -> bool {
        self.is_dot
    }

    pub fn position(&self) -> BadgePosition {
        self.position
    }

    /// 是否隐藏
    pub fn hidden(&self) -> bool {
        if let Some(h) = self.hidden_override {
            return h;
        }
        // Number(0) 默认隐藏；is_dot 或 Text 类型不隐藏
        match &self.value {
            BadgeValue::Number(0) => !self.is_dot,
            _ => false,
        }
    }

    /// 显示文本
    pub fn display_text(&self) -> String {
        if self.is_dot {
            return String::new();
        }
        match &self.value {
            BadgeValue::Number(n) => {
                if *n > self.max {
                    format!("{}+", self.max)
                } else {
                    n.to_string()
                }
            }
            BadgeValue::Text(s) => s.clone(),
        }
    }

    /// 处理消息
    pub fn handle(&mut self, msg: BadgeMessage) {
        match msg {
            BadgeMessage::Update(v) => {
                self.value = v;
                self.hidden_override = None;
            }
            BadgeMessage::SetHidden(h) => {
                self.hidden_override = Some(h);
            }
        }
    }

    /// 渲染 Badge 为 iced::Element（仅渲染徽标本身，不包含子元素）
    ///
    /// # 参数
    /// - `theme`: HarUI 主题引用
    pub fn view<'a>(&'a self, theme: &'a Theme) -> Element<'a, ()> {
        // hidden 状态：返回空 container
        if self.hidden() {
            return container(text("")).into();
        }

        let danger = Color::from(theme.danger.base);
        let text_color = Color::WHITE;

        if self.is_dot {
            // 小红点
            return container(text(""))
                .width(Length::Fixed(8.0))
                .height(Length::Fixed(8.0))
                .style(move |_t| iced::widget::container::Style {
                    text_color: None,
                    background: Some(iced::Background::Color(danger)),
                    border: iced::Border {
                        color: Color::WHITE,
                        width: 1.0,
                        radius: iced::border::radius(4.0),
                    },
                    shadow: iced::Shadow::default(),
                    snap: false,
                })
                .into();
        }

        // 数字/文本
        let display = self.display_text();
        let is_text = matches!(self.value, BadgeValue::Text(_));
        let width = if is_text {
            Length::Shrink
        } else {
            Length::Fixed(match display.len() {
                1 => 18.0,
                2 => 22.0,
                _ => 28.0,
            })
        };

        let badge_text = text(display).color(text_color).size(12);
        container(badge_text)
            .width(width)
            .height(Length::Fixed(18.0))
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center)
            .style(move |_t| iced::widget::container::Style {
                text_color: Some(text_color),
                background: Some(iced::Background::Color(danger)),
                border: iced::Border {
                    color: Color::WHITE,
                    width: 1.0,
                    radius: iced::border::radius(10.0),
                },
                shadow: iced::Shadow::default(),
                snap: false,
            })
            .into()
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_badge_default_max() {
        let b = Badge::new(BadgeValue::Number(5));
        assert_eq!(b.max(), 99);
    }

    #[test]
    fn test_badge_display_max_overflow() {
        let b = Badge::new(BadgeValue::Number(200)).with_max(99);
        assert_eq!(b.display_text(), "99+");
    }

    #[test]
    fn test_badge_hidden_logic_for_zero() {
        let b = Badge::new(BadgeValue::Number(0));
        assert!(b.hidden());
        let b2 = b.with_is_dot(true);
        assert!(!b2.hidden());
    }
}
