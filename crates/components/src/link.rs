//! Link 文字链接 — 参考 Element Plus `<el-link>`。
//!
//! 支持：6 种 type、underline、disabled、href、icon、点击事件。

use har_ui_core::theme::Theme;
use iced::widget::{button, container, text};
use iced::{Color, Element, Length, Padding};

/// 类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LinkType {
    #[default]
    Default,
    Primary,
    Success,
    Warning,
    Danger,
    Info,
}

/// Link 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkMessage {
    Click,
    ResetClick,
}

/// Link 组件
#[derive(Debug, Clone)]
pub struct Link {
    link_type: LinkType,
    text: String,
    href: Option<String>,
    underline: bool,
    disabled: bool,
    icon: Option<String>,
    clicked: bool,
}

impl Default for Link {
    fn default() -> Self {
        Self::new()
    }
}

impl Link {
    pub fn new() -> Self {
        Self {
            link_type: LinkType::Default,
            text: String::new(),
            href: None,
            underline: true,
            disabled: false,
            icon: None,
            clicked: false,
        }
    }

    pub fn with_type(mut self, t: LinkType) -> Self {
        self.link_type = t;
        self
    }

    pub fn with_text(mut self, t: impl Into<String>) -> Self {
        self.text = t.into();
        self
    }

    pub fn with_href(mut self, h: impl Into<String>) -> Self {
        self.href = Some(h.into());
        self
    }

    pub fn with_underline(mut self, v: bool) -> Self {
        self.underline = v;
        self
    }

    pub fn with_disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }

    pub fn with_icon(mut self, i: impl Into<String>) -> Self {
        self.icon = Some(i.into());
        self
    }

    pub fn link_type(&self) -> LinkType {
        self.link_type
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn href(&self) -> Option<&str> {
        self.href.as_deref()
    }

    pub fn underline(&self) -> bool {
        self.underline
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn icon(&self) -> Option<&str> {
        self.icon.as_deref()
    }

    pub fn clicked(&self) -> bool {
        self.clicked
    }

    pub fn handle(&mut self, msg: LinkMessage) {
        match msg {
            LinkMessage::Click => {
                if !self.disabled {
                    self.clicked = true;
                }
            }
            LinkMessage::ResetClick => {
                self.clicked = false;
            }
        }
    }

    /// 渲染 Link 为 iced::Element
    ///
    /// # 参数
    /// - `theme`: HarUI 主题引用
    /// - `on_click`: 点击链接时发出消息（disabled 时不绑定）
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        on_click: Message,
    ) -> Element<'a, Message> {
        let accent = match self.link_type {
            LinkType::Default => Color::from(theme.neutral.text_primary),
            LinkType::Primary => Color::from(theme.primary.base),
            LinkType::Success => Color::from(theme.success.base),
            LinkType::Warning => Color::from(theme.warning.base),
            LinkType::Danger => Color::from(theme.danger.base),
            LinkType::Info => Color::from(theme.info.base),
        };
        let text_color = if self.disabled {
            Color::from(theme.neutral.text_disabled)
        } else {
            accent
        };

        let mut row_children: Vec<Element<'a, Message>> = Vec::new();
        if let Some(icon) = &self.icon {
            row_children.push(text(icon.clone()).color(text_color).size(14.0).into());
            row_children.push(iced::widget::Space::new().width(Length::Fixed(4.0)).into());
        }
        let link_text = text(self.text.clone()).color(text_color).size(14.0);
        row_children.push(link_text.into());

        let content = iced::widget::Row::with_children(row_children)
            .spacing(0)
            .align_y(iced::Alignment::Center);

        let mut btn =
            button(content)
                .padding(Padding::from([2u16, 4u16]))
                .style(move |_t, _status| iced::widget::button::Style {
                    background: None,
                    text_color,
                    border: iced::Border::default(),
                    shadow: iced::Shadow::default(),
                    snap: false,
                });
        if !self.disabled {
            btn = btn.on_press(on_click);
        }

        let inner: Element<'a, Message> = btn.into();

        // underline 装饰：通过下边框模拟
        if self.underline && !self.disabled {
            container(inner)
                .style(move |_t| iced::widget::container::Style {
                    text_color: None,
                    background: None,
                    border: iced::Border {
                        color: text_color,
                        width: 0.0,
                        radius: iced::border::radius(0.0),
                    },
                    shadow: iced::Shadow::default(),
                    snap: false,
                })
                .into()
        } else {
            container(inner).into()
        }
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_link_default_underline_true() {
        let l = Link::new();
        assert!(l.underline());
        assert_eq!(l.link_type(), LinkType::Default);
    }
}
