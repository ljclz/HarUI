//! Tag 组件 — 标签
//!
//! 参考 Element Plus `<el-tag>`。
//! 支持：6 种 type、3 种 effect、3 种 size、closable、自定义颜色、hit 边框。

use har_ui_core::theme::Theme;
use iced::widget::{button, container, text};
use iced::{Color, Element, Padding};

/// Tag 类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TagType {
    #[default]
    Default,
    Primary,
    Success,
    Warning,
    Danger,
    Info,
}

/// Tag 视觉效果
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TagEffect {
    Dark,
    #[default]
    Light,
    Plain,
}

/// Tag 尺寸
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TagSize {
    Large,
    #[default]
    Default,
    Small,
}

/// Tag 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TagMessage {
    Close,
    UpdateText(String),
}

/// Tag 组件
#[derive(Debug, Clone)]
pub struct Tag {
    text: String,
    tag_type: TagType,
    effect: TagEffect,
    size: TagSize,
    closable: bool,
    hit: bool,
    color: Option<String>,
    closed: bool,
}

impl Tag {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            tag_type: TagType::Default,
            effect: TagEffect::Light,
            size: TagSize::Default,
            closable: false,
            hit: false,
            color: None,
            closed: false,
        }
    }

    pub fn with_type(mut self, t: TagType) -> Self {
        self.tag_type = t;
        self
    }

    pub fn with_effect(mut self, e: TagEffect) -> Self {
        self.effect = e;
        self
    }

    pub fn with_size(mut self, s: TagSize) -> Self {
        self.size = s;
        self
    }

    pub fn with_closable(mut self, v: bool) -> Self {
        self.closable = v;
        self
    }

    pub fn with_hit(mut self, v: bool) -> Self {
        self.hit = v;
        self
    }

    pub fn with_color(mut self, c: impl Into<String>) -> Self {
        self.color = Some(c.into());
        self
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn tag_type(&self) -> TagType {
        self.tag_type
    }

    pub fn effect(&self) -> TagEffect {
        self.effect
    }

    pub fn size(&self) -> TagSize {
        self.size
    }

    pub fn closable(&self) -> bool {
        self.closable
    }

    pub fn hit(&self) -> bool {
        self.hit
    }

    pub fn color(&self) -> Option<&String> {
        self.color.as_ref()
    }

    pub fn is_closed(&self) -> bool {
        self.closed
    }

    /// 重置关闭状态
    pub fn reset(&mut self) {
        self.closed = false;
    }

    /// 处理消息
    pub fn handle(&mut self, msg: TagMessage) {
        match msg {
            TagMessage::Close => {
                if self.closable {
                    self.closed = true;
                }
            }
            TagMessage::UpdateText(s) => {
                self.text = s;
            }
        }
    }

    /// 按类型选调色板
    fn palette_for_type<'a>(theme: &'a Theme, t: TagType) -> &'a har_ui_core::theme::color::ColorPalette {
        match t {
            TagType::Primary => &theme.primary,
            TagType::Success => &theme.success,
            TagType::Warning => &theme.warning,
            TagType::Danger => &theme.danger,
            TagType::Info => &theme.info,
            TagType::Default => &theme.info,
        }
    }

    /// 按 size 计算 padding 与 font_size
    fn size_props(size: TagSize) -> (Padding, f32) {
        match size {
            TagSize::Large => (Padding::from([6u16, 12u16]), 16.0),
            TagSize::Default => (Padding::from([4u16, 10u16]), 14.0),
            TagSize::Small => (Padding::from([2u16, 8u16]), 12.0),
        }
    }

    /// 渲染 Tag 为 iced::Element
    ///
    /// # 参数
    /// - `theme`: HarUI 主题引用
    /// - `on_close`: 关闭按钮回调（closable=true 时使用）
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        on_close: impl Fn() -> Message + 'a,
    ) -> Element<'a, Message> {
        // closed 状态：返回空 container
        if self.closed {
            return container(text("")).into();
        }

        let palette = Self::palette_for_type(theme, self.tag_type);
        let base_color = Color::from(palette.base);
        let (padding, font_size) = Self::size_props(self.size);

        // 按 effect 计算 bg / text / border
        let (bg_color, text_color, border_color) = match self.effect {
            TagEffect::Dark => (
                Color { a: 1.0, ..base_color },
                Color::WHITE,
                Color { a: 0.0, ..base_color },
            ),
            TagEffect::Light => (
                Color { a: 0.1, ..base_color },
                base_color,
                Color { a: 0.0, ..base_color },
            ),
            TagEffect::Plain => (
                Color { a: 0.0, ..base_color },
                base_color,
                base_color,
            ),
        };

        // 自定义 color 覆盖
        let custom = self.color.as_ref().and_then(|c| parse_hex(c));
        let (bg_color, text_color, border_color) = if let Some(c) = custom {
            (c, c, c)
        } else {
            (bg_color, text_color, border_color)
        };

        // 文本
        let label = text(self.text.clone()).color(text_color).size(font_size);

        // hit 边框：border_color 加深
        let final_border_color = if self.hit {
            Color { a: 1.0, ..base_color }
        } else {
            border_color
        };

        let mut row = iced::widget::Row::new()
            .push(label)
            .align_y(iced::Alignment::Center);

        // 关闭按钮
        if self.closable {
            let close_btn = button(text("×").color(text_color).size(font_size))
                .padding(Padding::from([0u16, 4u16]))
                .on_press(on_close())
                .style(move |_t, _status| iced::widget::button::Style {
                    background: None,
                    text_color,
                    border: iced::Border::default(),
                    shadow: iced::Shadow::default(),
                });
            row = row.push(close_btn);
        }

        container(row)
            .padding(padding)
            .style(move |_t| iced::widget::container::Style {
                text_color: Some(text_color),
                background: Some(iced::Background::Color(bg_color)),
                border: iced::Border {
                    color: final_border_color,
                    width: if self.effect == TagEffect::Plain || self.hit { 1.0 } else { 0.0 },
                    radius: iced::border::radius(4.0),
                },
                shadow: iced::Shadow::default(),
            })
            .into()
    }
}

/// 解析 hex 颜色（#RRGGBB）为 Color
fn parse_hex(hex: &str) -> Option<Color> {
    let hex = hex.trim_start_matches('#');
    if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        return Some(Color::from_rgb8(r, g, b));
    }
    None
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_tag_default_props() {
        let t = Tag::new("hello");
        assert_eq!(t.tag_type(), TagType::Default);
        assert_eq!(t.effect(), TagEffect::Light);
        assert_eq!(t.size(), TagSize::Default);
    }

    #[test]
    fn test_tag_close_respects_closable() {
        let mut t = Tag::new("x");
        t.handle(TagMessage::Close);
        assert!(!t.is_closed());

        let mut t2 = Tag::new("x").with_closable(true);
        t2.handle(TagMessage::Close);
        assert!(t2.is_closed());
    }

    #[test]
    fn test_tag_update_text() {
        let mut t = Tag::new("a");
        t.handle(TagMessage::UpdateText("b".to_string()));
        assert_eq!(t.text(), "b");
    }
}
