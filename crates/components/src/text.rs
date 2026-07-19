//! Text 文本 — 参考 Element Plus `<el-text>`。
//!
//! 支持：6 种 type、3 种 size、truncated、tag、copyable、max_lines。

use har_ui_core::theme::Theme;
use iced::widget::{container, text};
use iced::{Color, Element, Length};

/// 类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextType {
    #[default]
    Default,
    Primary,
    Success,
    Warning,
    Danger,
    Info,
}

/// 尺寸
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextSize {
    Large,
    #[default]
    Default,
    Small,
}

/// 标签
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextTag {
    #[default]
    Span,
    P,
    Div,
    H1,
    H2,
    H3,
}

/// Text 组件
#[derive(Debug, Clone)]
pub struct Text {
    text_type: TextType,
    size: TextSize,
    content: String,
    truncated: bool,
    tag: TextTag,
    copyable: bool,
    max_lines: Option<u32>,
}

impl Default for Text {
    fn default() -> Self {
        Self::new()
    }
}

impl Text {
    pub fn new() -> Self {
        Self {
            text_type: TextType::Default,
            size: TextSize::Default,
            content: String::new(),
            truncated: false,
            tag: TextTag::Span,
            copyable: false,
            max_lines: None,
        }
    }

    pub fn with_type(mut self, t: TextType) -> Self {
        self.text_type = t;
        self
    }

    pub fn with_size(mut self, s: TextSize) -> Self {
        self.size = s;
        self
    }

    pub fn with_content(mut self, c: impl Into<String>) -> Self {
        self.content = c.into();
        self
    }

    pub fn with_truncated(mut self, v: bool) -> Self {
        self.truncated = v;
        self
    }

    pub fn with_tag(mut self, t: TextTag) -> Self {
        self.tag = t;
        self
    }

    pub fn with_copyable(mut self, v: bool) -> Self {
        self.copyable = v;
        self
    }

    pub fn with_max_lines(mut self, n: u32) -> Self {
        self.max_lines = Some(n);
        self
    }

    pub fn text_type(&self) -> TextType {
        self.text_type
    }

    pub fn size(&self) -> TextSize {
        self.size
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn truncated(&self) -> bool {
        self.truncated
    }

    pub fn tag(&self) -> TextTag {
        self.tag
    }

    pub fn copyable(&self) -> bool {
        self.copyable
    }

    pub fn max_lines(&self) -> Option<u32> {
        self.max_lines
    }

    /// 渲染 Text 为 iced::Element
    ///
    /// # 参数
    /// - `theme`: HarUI 主题引用
    pub fn view<'a>(&'a self, theme: &'a Theme) -> Element<'a, ()> {
        let color = match self.text_type {
            TextType::Default => Color::from(theme.neutral.text_regular),
            TextType::Primary => Color::from(theme.primary.base),
            TextType::Success => Color::from(theme.success.base),
            TextType::Warning => Color::from(theme.warning.base),
            TextType::Danger => Color::from(theme.danger.base),
            TextType::Info => Color::from(theme.info.base),
        };
        let font_size = match self.size {
            TextSize::Large => 18.0,
            TextSize::Default => 14.0,
            TextSize::Small => 12.0,
        };

        let label = match self.tag {
            TextTag::H1 => "H1: ",
            TextTag::H2 => "H2: ",
            TextTag::H3 => "H3: ",
            _ => "",
        };

        let display_text = if label.is_empty() {
            self.content.clone()
        } else {
            format!("{}{}", label, self.content)
        };

        let make_text = || text(display_text.clone()).color(color).size(font_size);

        let wrapper: iced::widget::Container<'_, ()> = if self.truncated {
            container(
                iced::widget::Row::new()
                    .push(make_text())
                    .push(text("…").color(Color::from(theme.neutral.text_placeholder))),
            )
            .width(Length::Shrink)
        } else {
            container(make_text()).width(Length::Shrink)
        };

        if self.copyable {
            let copyable_row = iced::widget::Row::new()
                .push(wrapper)
                .push(iced::widget::Space::with_width(Length::Fixed(4.0)))
                .push(
                    text("📋")
                        .color(Color::from(theme.neutral.text_secondary))
                        .size(12.0),
                )
                .align_y(iced::Alignment::Center);
            return copyable_row.into();
        }
        wrapper.into()
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_text_default_empty_content() {
        let t = Text::new();
        assert_eq!(t.content(), "");
        assert_eq!(t.text_type(), TextType::Default);
        assert_eq!(t.size(), TextSize::Default);
        assert_eq!(t.tag(), TextTag::Span);
    }
}
