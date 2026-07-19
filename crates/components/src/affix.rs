//! Affix 固钉 — 参考 Element Plus `<el-affix>`。
//!
//! 支持：固定位置（top/bottom）、偏移、滚动事件触发、目标容器、状态变化事件。

use har_ui_core::theme::Theme;
use iced::widget::{container, text};
use iced::{Color, Element, Length, Padding};

/// 固定位置
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AffixPosition {
    #[default]
    Top,
    Bottom,
}

/// Affix 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AffixMessage {
    /// 滚动事件（scroll_y 为页面/容器垂直滚动位置）
    Scroll { scroll_y: u32 },
}

/// Affix 组件
#[derive(Debug, Clone)]
pub struct Affix {
    position: AffixPosition,
    offset: u32,
    zindex: i32,
    target: Option<String>,
    fixed: bool,
    change_fired: bool,
}

impl Default for Affix {
    fn default() -> Self {
        Self::new()
    }
}

impl Affix {
    pub fn new() -> Self {
        Self {
            position: AffixPosition::Top,
            offset: 0,
            zindex: 100,
            target: None,
            fixed: false,
            change_fired: false,
        }
    }

    pub fn with_position(mut self, p: AffixPosition) -> Self {
        self.position = p;
        self
    }

    pub fn with_offset(mut self, o: u32) -> Self {
        self.offset = o;
        self
    }

    pub fn with_zindex(mut self, z: i32) -> Self {
        self.zindex = z;
        self
    }

    pub fn with_target(mut self, t: impl Into<String>) -> Self {
        self.target = Some(t.into());
        self
    }

    pub fn position(&self) -> AffixPosition {
        self.position
    }

    pub fn offset(&self) -> u32 {
        self.offset
    }

    pub fn zindex(&self) -> i32 {
        self.zindex
    }

    pub fn target(&self) -> Option<&str> {
        self.target.as_deref()
    }

    pub fn is_fixed(&self) -> bool {
        self.fixed
    }

    pub fn change_fired(&self) -> bool {
        self.change_fired
    }

    pub fn clear_change_flag(&mut self) {
        self.change_fired = false;
    }

    pub fn handle(&mut self, msg: AffixMessage) {
        match msg {
            AffixMessage::Scroll { scroll_y } => {
                // top 模式：scroll_y > offset 时固定（严格大于）
                let new_fixed = match self.position {
                    AffixPosition::Top => scroll_y > self.offset,
                    AffixPosition::Bottom => {
                        // bottom 模式：scroll_y < offset 时固定（这里简化逻辑）
                        // 实际实现需要 viewport 高度，这里仅做语义化处理
                        scroll_y < self.offset
                    }
                };
                if new_fixed != self.fixed {
                    self.fixed = new_fixed;
                    self.change_fired = true;
                }
            }
        }
    }

    /// 渲染 Affix 为 iced::Element
    ///
    /// 当 `is_fixed()` 为 true 时，容器加边框（模拟固定态视觉反馈），
    /// 否则以普通容器包裹传入的 content。
    ///
    /// # 参数
    /// - `theme`: HarUI 主题引用
    /// - `content`: 被包裹的内容
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        content: Element<'a, Message>,
    ) -> Element<'a, Message> {
        let primary = Color::from(theme.primary.base);
        let border_light = Color::from(theme.neutral.border_light);
        let text_secondary = Color::from(theme.neutral.text_secondary);

        let (border_color, border_width, label) = if self.fixed {
            (primary, 2.0, "[affixed] ")
        } else {
            (border_light, 0.0, "")
        };

        let mut row_children: Vec<Element<'a, Message>> = Vec::new();
        if !label.is_empty() {
            row_children.push(
                text(label.to_string())
                    .color(text_secondary)
                    .size(12.0)
                    .into(),
            );
        }
        row_children.push(content);

        let inner = iced::widget::Row::with_children(row_children)
            .spacing(0)
            .align_y(iced::Alignment::Center);

        let wrap = container(inner)
            .width(Length::Fill)
            .padding(Padding::from([8u16, 12u16]))
            .style(move |_t| iced::widget::container::Style {
                text_color: None,
                background: None,
                border: iced::Border {
                    color: border_color,
                    width: border_width,
                    radius: iced::border::radius(4.0),
                },
                shadow: iced::Shadow::default(),
            });

        wrap.into()
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_affix_default_zindex_100() {
        let a = Affix::new();
        assert_eq!(a.zindex(), 100);
        assert_eq!(a.offset(), 0);
        assert_eq!(a.position(), AffixPosition::Top);
    }
}
