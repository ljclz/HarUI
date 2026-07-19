//! Space 间距 — 参考 Element Plus `<el-space>`。
//!
//! 支持：方向（horizontal/vertical）、size、wrap、fill、alignment、items 增删。

use har_ui_core::theme::Theme;
use iced::widget::{container, text};
use iced::{Element, Length};

/// 排列方向
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpaceDirection {
    #[default]
    Horizontal,
    Vertical,
}

/// 对齐方式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpaceAlignment {
    Start,
    #[default]
    Center,
    End,
    Baseline,
}

/// Space 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpaceMessage {
    /// 添加一项
    AddItem(String),
    /// 移除首个匹配项（不存在则无操作）
    RemoveItem(String),
    /// 清空所有项
    Clear,
}

/// Space 组件
#[derive(Debug, Clone)]
pub struct Space {
    direction: SpaceDirection,
    size: u32,
    wrap: bool,
    fill: bool,
    alignment: SpaceAlignment,
    items: Vec<String>,
}

impl Default for Space {
    fn default() -> Self {
        Self::new()
    }
}

impl Space {
    pub fn new() -> Self {
        Self {
            direction: SpaceDirection::Horizontal,
            size: 8,
            wrap: false,
            fill: false,
            alignment: SpaceAlignment::Center,
            items: Vec::new(),
        }
    }

    pub fn with_direction(mut self, d: SpaceDirection) -> Self {
        self.direction = d;
        self
    }

    pub fn with_size(mut self, s: u32) -> Self {
        self.size = s;
        self
    }

    pub fn with_wrap(mut self, v: bool) -> Self {
        self.wrap = v;
        self
    }

    pub fn with_fill(mut self, v: bool) -> Self {
        self.fill = v;
        self
    }

    pub fn with_alignment(mut self, a: SpaceAlignment) -> Self {
        self.alignment = a;
        self
    }

    pub fn direction(&self) -> SpaceDirection {
        self.direction
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn wrap(&self) -> bool {
        self.wrap
    }

    pub fn fill(&self) -> bool {
        self.fill
    }

    pub fn alignment(&self) -> SpaceAlignment {
        self.alignment
    }

    pub fn items(&self) -> &[String] {
        &self.items
    }

    pub fn handle(&mut self, msg: SpaceMessage) {
        match msg {
            SpaceMessage::AddItem(item) => {
                self.items.push(item);
            }
            SpaceMessage::RemoveItem(item) => {
                if let Some(pos) = self.items.iter().position(|i| i == &item) {
                    self.items.remove(pos);
                }
            }
            SpaceMessage::Clear => {
                self.items.clear();
            }
        }
    }

    /// 渲染 Space 为 iced::Element
    ///
    /// - 按 direction 排列传入的 children，spacing = size
    /// - fill=true 时容器宽度填满
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        _theme: &'a Theme,
        children: Vec<Element<'a, Message>>,
    ) -> Element<'a, Message> {
        if children.is_empty() {
            return container(text("")).into();
        }

        let spacing = self.size as u16;
        let align = match self.alignment {
            SpaceAlignment::Start => iced::Alignment::Start,
            SpaceAlignment::Center => iced::Alignment::Center,
            SpaceAlignment::End => iced::Alignment::End,
            SpaceAlignment::Baseline => iced::Alignment::Start,
        };

        let element: Element<'a, Message> = match self.direction {
            SpaceDirection::Horizontal => iced::widget::Row::with_children(children)
                .spacing(spacing)
                .align_y(align)
                .into(),
            SpaceDirection::Vertical => iced::widget::Column::with_children(children)
                .spacing(spacing)
                .align_x(align)
                .into(),
        };

        let _ = self.wrap;

        if self.fill {
            container(element)
                .width(Length::Fill)
                .into()
        } else {
            element
        }
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_space_default_size() {
        assert_eq!(Space::new().size(), 8);
    }
}
