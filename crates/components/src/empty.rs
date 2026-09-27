//! Empty 空状态 — 参考 Element Plus `<el-empty>`。
//!
//! 支持：默认空状态图片、自定义 image/description、尺寸、附加内容。

use har_ui_core::theme::Theme;
use iced::widget::{container, text};
use iced::{Color, Element, Length, Padding};

/// 空状态图片类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EmptyImage {
    #[default]
    Default,
    Error,
    Network,
    Custom,
}

/// 空状态尺寸
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EmptySize {
    #[default]
    Normal,
    Small,
    Large,
}

/// Empty 组件
#[derive(Debug, Clone)]
pub struct Empty {
    description: String,
    image: EmptyImage,
    image_url: Option<String>,
    size: EmptySize,
    has_extra: bool,
}

impl Default for Empty {
    fn default() -> Self {
        Self::new()
    }
}

impl Empty {
    pub fn new() -> Self {
        Self {
            description: "暂无数据".to_string(),
            image: EmptyImage::Default,
            image_url: None,
            size: EmptySize::Normal,
            has_extra: false,
        }
    }

    pub fn with_description(mut self, d: impl Into<String>) -> Self {
        self.description = d.into();
        self
    }

    pub fn with_image(mut self, img: EmptyImage) -> Self {
        self.image = img;
        self
    }

    pub fn with_image_url(mut self, url: impl Into<String>) -> Self {
        self.image = EmptyImage::Custom;
        self.image_url = Some(url.into());
        self
    }

    pub fn with_size(mut self, s: EmptySize) -> Self {
        self.size = s;
        self
    }

    pub fn with_has_extra(mut self, v: bool) -> Self {
        self.has_extra = v;
        self
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn image(&self) -> EmptyImage {
        self.image
    }

    pub fn image_url(&self) -> Option<&str> {
        self.image_url.as_deref()
    }

    pub fn size(&self) -> EmptySize {
        self.size
    }

    pub fn has_extra(&self) -> bool {
        self.has_extra
    }

    /// 按 size 计算 icon 尺寸
    fn icon_size(size: EmptySize) -> f32 {
        match size {
            EmptySize::Small => 40.0,
            EmptySize::Normal => 64.0,
            EmptySize::Large => 100.0,
        }
    }

    /// 按 image 类型选 emoji
    fn icon_emoji(image: EmptyImage) -> &'static str {
        match image {
            EmptyImage::Default => "📭",
            EmptyImage::Error => "⚠️",
            EmptyImage::Network => "📡",
            EmptyImage::Custom => "🖼",
        }
    }

    /// 渲染 Empty 为 iced::Element
    pub fn view<'a>(&'a self, theme: &'a Theme) -> Element<'a, ()> {
        let icon_size = Self::icon_size(self.size);
        let icon_str = if self.image == EmptyImage::Custom && self.image_url.is_some() {
            "🖼"
        } else {
            Self::icon_emoji(self.image)
        };
        let icon_text = text(icon_str).size(icon_size);
        let desc_text = text(self.description.clone())
            .color(Color::from(theme.neutral.text_secondary))
            .size(if self.size == EmptySize::Small {
                12.0
            } else {
                14.0
            });

        let mut col = iced::widget::Column::new()
            .push(icon_text)
            .push(iced::widget::Space::with_height(Length::Fixed(8.0)))
            .push(desc_text)
            .spacing(0)
            .align_x(iced::alignment::Horizontal::Center);

        if self.has_extra {
            col = col.push(iced::widget::Space::with_height(Length::Fixed(12.0)));
            col = col.push(
                text("[extra slot]")
                    .color(Color::from(theme.neutral.text_placeholder))
                    .size(12.0),
            );
        }

        container(col)
            .width(Length::Fill)
            .padding(Padding::from([24u16, 16u16]))
            .align_x(iced::alignment::Horizontal::Center)
            .into()
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_empty_default_values() {
        let e = Empty::new();
        assert_eq!(e.description(), "暂无数据");
        assert_eq!(e.image(), EmptyImage::Default);
        assert_eq!(e.size(), EmptySize::Normal);
        assert!(!e.has_extra());
        assert_eq!(e.image_url(), None);
    }
}
