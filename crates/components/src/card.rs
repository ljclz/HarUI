//! Card 组件 — 卡片容器
//!
//! 参考 Element Plus `<el-card>`。
//! 支持：header/footer 插槽、shadow(hover/always/never)、图片、hover 状态。

use har_ui_core::theme::Theme;
use har_ui_core::theme::style_sheets;
use iced::widget::{container, text};
use iced::{Color, Element, Length, Padding};

/// 阴影模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CardShadow {
    /// 始终显示
    #[default]
    Always,
    /// 悬停时显示
    Hover,
    /// 从不显示
    Never,
}

/// Card 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CardMessage {
    Hovered,
    Unhovered,
    UpdateBody(String),
}

/// Card 组件
#[derive(Debug, Clone)]
pub struct Card {
    body: String,
    header: Option<String>,
    footer: Option<String>,
    image: Option<String>,
    shadow: CardShadow,
    hovered: bool,
}

impl Card {
    pub fn new(body: impl Into<String>) -> Self {
        Self {
            body: body.into(),
            header: None,
            footer: None,
            image: None,
            shadow: CardShadow::Always,
            hovered: false,
        }
    }

    pub fn with_header(mut self, h: impl Into<String>) -> Self {
        self.header = Some(h.into());
        self
    }

    pub fn with_footer(mut self, f: impl Into<String>) -> Self {
        self.footer = Some(f.into());
        self
    }

    pub fn with_image(mut self, src: impl Into<String>) -> Self {
        self.image = Some(src.into());
        self
    }

    pub fn with_shadow(mut self, s: CardShadow) -> Self {
        self.shadow = s;
        self
    }

    pub fn body(&self) -> &str {
        &self.body
    }

    pub fn header(&self) -> Option<&String> {
        self.header.as_ref()
    }

    pub fn footer(&self) -> Option<&String> {
        self.footer.as_ref()
    }

    pub fn image(&self) -> Option<&String> {
        self.image.as_ref()
    }

    pub fn shadow(&self) -> CardShadow {
        self.shadow
    }

    pub fn is_hovered(&self) -> bool {
        self.hovered
    }

    /// 计算属性：是否应显示阴影
    pub fn should_show_shadow(&self) -> bool {
        match self.shadow {
            CardShadow::Always => true,
            CardShadow::Never => false,
            CardShadow::Hover => self.hovered,
        }
    }

    /// 处理消息
    pub fn handle(&mut self, msg: CardMessage) {
        match msg {
            CardMessage::Hovered => self.hovered = true,
            CardMessage::Unhovered => self.hovered = false,
            CardMessage::UpdateBody(s) => self.body = s,
        }
    }

    /// 渲染 Card 为 iced::Element
    ///
    /// # 参数
    /// - `theme`: HarUI 主题引用
    pub fn view<'a>(&'a self, theme: &'a Theme) -> Element<'a, ()> {
        let text_primary = Color::from(theme.neutral.text_primary);
        let text_regular = Color::from(theme.neutral.text_regular);
        let border_lighter = Color::from(theme.neutral.border_lighter);

        let mut col_children: Vec<Element<'a, ()>> = Vec::new();

        // header
        if let Some(h) = &self.header {
            let header_text = text(h.clone()).color(text_primary).size(16);
            let header = container(header_text)
                .width(Length::Fill)
                .padding(Padding::from([12u16, 16u16]))
                .style(move |_t| iced::widget::container::Style {
                    text_color: Some(text_primary),
                    background: None,
                    border: iced::Border {
                        color: border_lighter,
                        width: 0.0,
                        radius: iced::border::radius(0.0),
                    },
                    shadow: iced::Shadow::default(),
                });
            // header 下加分割线
            let divider = container(text(""))
                .width(Length::Fill)
                .height(Length::Fixed(1.0))
                .style(move |_t| iced::widget::container::Style {
                    text_color: None,
                    background: Some(iced::Background::Color(border_lighter)),
                    border: iced::Border::default(),
                    shadow: iced::Shadow::default(),
                });
            col_children.push(header.into());
            col_children.push(divider.into());
        }

        // body
        let body_text = text(self.body.clone()).color(text_regular).size(14);
        let body = container(body_text)
            .width(Length::Fill)
            .padding(Padding::from(16u16));
        col_children.push(body.into());

        // footer
        if let Some(f) = &self.footer {
            let divider = container(text(""))
                .width(Length::Fill)
                .height(Length::Fixed(1.0))
                .style(move |_t| iced::widget::container::Style {
                    text_color: None,
                    background: Some(iced::Background::Color(border_lighter)),
                    border: iced::Border::default(),
                    shadow: iced::Shadow::default(),
                });
            let footer_text = text(f.clone()).color(text_regular).size(14);
            let footer = container(footer_text)
                .width(Length::Fill)
                .padding(Padding::from([12u16, 16u16]));
            col_children.push(divider.into());
            col_children.push(footer.into());
        }

        let col = iced::widget::Column::with_children(col_children).spacing(0);

        // 卡片容器：圆角 + 阴影（按 shadow 模式）
        let should_show = self.should_show_shadow();
        container(col)
            .width(Length::Fill)
            .style(move |_t| {
                let base = style_sheets::container_card_style(theme);
                if should_show {
                    base
                } else {
                    iced::widget::container::Style {
                        shadow: iced::Shadow::default(),
                        ..base
                    }
                }
            })
            .into()
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_card_default_shadow() {
        let c = Card::new("body");
        assert_eq!(c.shadow(), CardShadow::Always);
    }

    #[test]
    fn test_card_should_show_shadow_logic() {
        let mut c = Card::new("body").with_shadow(CardShadow::Hover);
        assert!(!c.should_show_shadow());
        c.handle(CardMessage::Hovered);
        assert!(c.should_show_shadow());
    }

    #[test]
    fn test_card_update_body_message() {
        let mut c = Card::new("hello");
        c.handle(CardMessage::UpdateBody("world".to_string()));
        assert_eq!(c.body(), "world");
    }
}
