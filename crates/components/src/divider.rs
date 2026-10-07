//! Divider 分割线 — 参考 Element Plus `<el-divider>`。
//!
//! 支持：方向（horizontal/vertical）、内容位置（left/center/right）、文本、虚线样式。

use har_ui_core::theme::Theme;
use iced::widget::{container, text};
use iced::{Color, Element, Length};

/// 方向
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DividerDirection {
    #[default]
    Horizontal,
    Vertical,
}

/// 内容位置
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DividerContentPosition {
    Left,
    #[default]
    Center,
    Right,
}

/// Divider 组件
#[derive(Debug, Clone)]
pub struct Divider {
    direction: DividerDirection,
    content_position: DividerContentPosition,
    text: Option<String>,
    border_dashed: bool,
}

impl Default for Divider {
    fn default() -> Self {
        Self::new()
    }
}

impl Divider {
    pub fn new() -> Self {
        Self {
            direction: DividerDirection::Horizontal,
            content_position: DividerContentPosition::Center,
            text: None,
            border_dashed: false,
        }
    }

    pub fn with_direction(mut self, d: DividerDirection) -> Self {
        self.direction = d;
        self
    }

    pub fn with_content_position(mut self, p: DividerContentPosition) -> Self {
        self.content_position = p;
        self
    }

    pub fn with_text(mut self, t: impl Into<String>) -> Self {
        self.text = Some(t.into());
        self
    }

    pub fn with_border_dashed(mut self, v: bool) -> Self {
        self.border_dashed = v;
        self
    }

    pub fn direction(&self) -> DividerDirection {
        self.direction
    }

    pub fn content_position(&self) -> DividerContentPosition {
        self.content_position
    }

    pub fn text(&self) -> Option<&str> {
        self.text.as_deref()
    }

    pub fn border_dashed(&self) -> bool {
        self.border_dashed
    }

    pub fn clear_text(&mut self) {
        self.text = None;
    }

    /// 渲染 Divider 为 iced::Element
    ///
    /// # 参数
    /// - `theme`: HarUI 主题引用
    pub fn view<'a>(&'a self, theme: &'a Theme) -> Element<'a, ()> {
        let border_color = Color::from(theme.neutral.border_base);
        let text_color = Color::from(theme.neutral.text_primary);

        // 垂直方向：渲染一条垂直线
        if matches!(self.direction, DividerDirection::Vertical) {
            let line = container(text(""))
                .width(Length::Fixed(1.0))
                .height(Length::Fixed(24.0))
                .style(move |_t| iced::widget::container::Style {
                    text_color: None,
                    background: Some(iced::Background::Color(border_color)),
                    border: iced::Border::default(),
                    shadow: iced::Shadow::default(),
                    snap: false,
                });
            return line.into();
        }

        // 水平方向：可选文本 + 两侧分割线
        match &self.text {
            None => {
                let line = container(text(""))
                    .width(Length::Fill)
                    .height(Length::Fixed(1.0))
                    .style(move |_t| iced::widget::container::Style {
                        text_color: None,
                        background: Some(iced::Background::Color(border_color)),
                        border: iced::Border::default(),
                        shadow: iced::Shadow::default(),
                        snap: false,
                    });
                line.into()
            }
            Some(t) => {
                let left_flex = match self.content_position {
                    DividerContentPosition::Left => Length::Fixed(0.0),
                    _ => Length::Fill,
                };
                let right_flex = match self.content_position {
                    DividerContentPosition::Right => Length::Fixed(0.0),
                    _ => Length::Fill,
                };

                let left_line = container(text(""))
                    .width(left_flex)
                    .height(Length::Fixed(1.0))
                    .style(move |_t| iced::widget::container::Style {
                        text_color: None,
                        background: Some(iced::Background::Color(border_color)),
                        border: iced::Border::default(),
                        shadow: iced::Shadow::default(),
                        snap: false,
                    });

                let right_line = container(text(""))
                    .width(right_flex)
                    .height(Length::Fixed(1.0))
                    .style(move |_t| iced::widget::container::Style {
                        text_color: None,
                        background: Some(iced::Background::Color(border_color)),
                        border: iced::Border::default(),
                        shadow: iced::Shadow::default(),
                        snap: false,
                    });

                let label = text(t.clone()).color(text_color).size(14.0);

                iced::widget::Row::new()
                    .push(left_line)
                    .push(iced::widget::Space::new().width(Length::Fixed(8.0)))
                    .push(label)
                    .push(iced::widget::Space::new().width(Length::Fixed(8.0)))
                    .push(right_line)
                    .align_y(iced::Alignment::Center)
                    .into()
            }
        }
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_divider_default_no_text_no_dashed() {
        let d = Divider::new();
        assert!(d.text().is_none());
        assert!(!d.border_dashed());
        assert_eq!(d.direction(), DividerDirection::Horizontal);
    }
}
