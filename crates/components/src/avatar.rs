//! Avatar 头像 — 参考 Element Plus `<el-avatar>`。
//!
//! 支持：3 种来源（icon/text/image）、4 种尺寸、2 种形状、4 种 fit、加载失败 fallback。

use har_ui_core::theme::Theme;
use iced::widget::{container, text};
use iced::{Color, Element, Length};

/// 来源类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AvatarSource {
    #[default]
    Icon,
    Text,
    Image,
}

/// 预设尺寸
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AvatarSize {
    Large,
    #[default]
    Default,
    Small,
}

/// 形状
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AvatarShape {
    #[default]
    Circle,
    Square,
}

/// 图片适应模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AvatarFit {
    Fill,
    #[default]
    Cover,
    Contain,
    None,
}

/// Avatar 组件
#[derive(Debug, Clone)]
pub struct Avatar {
    source: AvatarSource,
    size: AvatarSize,
    pixel_size: Option<u32>,
    shape: AvatarShape,
    fit: AvatarFit,
    icon: Option<String>,
    text: Option<String>,
    image_url: Option<String>,
    fallback_text: Option<String>,
    has_error: bool,
}

impl Default for Avatar {
    fn default() -> Self {
        Self::new()
    }
}

impl Avatar {
    pub fn new() -> Self {
        Self {
            source: AvatarSource::Icon,
            size: AvatarSize::Default,
            pixel_size: None,
            shape: AvatarShape::Circle,
            fit: AvatarFit::Cover,
            icon: None,
            text: None,
            image_url: None,
            fallback_text: None,
            has_error: false,
        }
    }

    pub fn with_text(mut self, t: impl Into<String>) -> Self {
        self.text = Some(t.into());
        self.source = AvatarSource::Text;
        self
    }

    pub fn with_image_url(mut self, url: impl Into<String>) -> Self {
        self.image_url = Some(url.into());
        self.source = AvatarSource::Image;
        self
    }

    pub fn with_icon(mut self, i: impl Into<String>) -> Self {
        self.icon = Some(i.into());
        self.source = AvatarSource::Icon;
        self
    }

    pub fn with_size(mut self, s: AvatarSize) -> Self {
        self.size = s;
        self
    }

    pub fn with_pixel_size(mut self, s: u32) -> Self {
        self.pixel_size = Some(s);
        self
    }

    pub fn with_shape(mut self, s: AvatarShape) -> Self {
        self.shape = s;
        self
    }

    pub fn with_fit(mut self, f: AvatarFit) -> Self {
        self.fit = f;
        self
    }

    pub fn with_fallback_text(mut self, t: impl Into<String>) -> Self {
        self.fallback_text = Some(t.into());
        self
    }

    /// 从姓名取首字符作为头像文字
    pub fn with_initials(mut self, name: impl Into<String>) -> Self {
        let n = name.into();
        let first = n.chars().next().map(|c| c.to_string()).unwrap_or_default();
        self.text = Some(first);
        self.source = AvatarSource::Text;
        self
    }

    pub fn source(&self) -> AvatarSource {
        self.source
    }

    pub fn size(&self) -> AvatarSize {
        self.size
    }

    pub fn pixel_size(&self) -> Option<u32> {
        self.pixel_size
    }

    pub fn shape(&self) -> AvatarShape {
        self.shape
    }

    pub fn fit(&self) -> AvatarFit {
        self.fit
    }

    pub fn text(&self) -> Option<&str> {
        self.text.as_deref()
    }

    pub fn image_url(&self) -> Option<&str> {
        self.image_url.as_deref()
    }

    pub fn icon(&self) -> Option<&str> {
        self.icon.as_deref()
    }

    pub fn has_error(&self) -> bool {
        self.has_error
    }

    /// 模拟图片加载失败：切换到 fallback 文本（若存在）
    pub fn handle_load_error(&mut self) {
        self.has_error = true;
        if let Some(fb) = self.fallback_text.clone() {
            self.text = Some(fb);
            self.source = AvatarSource::Text;
        }
    }

    /// 按 size 计算像素尺寸
    fn pixel_size_for(size: AvatarSize, override_size: Option<u32>) -> u32 {
        if let Some(s) = override_size {
            return s.max(16);
        }
        match size {
            AvatarSize::Large => 56,
            AvatarSize::Default => 40,
            AvatarSize::Small => 28,
        }
    }

    /// 渲染 Avatar 为 iced::Element
    pub fn view<'a>(&'a self, theme: &'a Theme) -> Element<'a, ()> {
        let px = Self::pixel_size_for(self.size, self.pixel_size) as f32;
        let radius = match self.shape {
            AvatarShape::Circle => px / 2.0,
            AvatarShape::Square => 4.0,
        };
        let bg = Color::from(theme.neutral.bg_overlay);
        let text_color = Color::from(theme.neutral.text_primary);

        // 内容：按 source 选择 icon/text/image
        // iced 0.13.1 暂无内置 image widget（需 features），简化为占位
        let content: Element<'a, ()> = match self.source {
            AvatarSource::Icon => {
                let icon_text = self.icon.clone().unwrap_or_else(|| "👤".to_string());
                text(icon_text).color(text_color).size(px * 0.5).into()
            }
            AvatarSource::Text => {
                let t = self.text.clone().unwrap_or_default();
                text(t).color(text_color).size(px * 0.45).into()
            }
            AvatarSource::Image => {
                // 简化：图片显示为占位框（实际加载需 features=["image"]）
                let url = self.image_url.clone().unwrap_or_default();
                let label = if url.is_empty() {
                    "IMG".to_string()
                } else {
                    "🖼".to_string()
                };
                text(label).color(text_color).size(px * 0.4).into()
            }
        };

        container(content)
            .width(Length::Fixed(px))
            .height(Length::Fixed(px))
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center)
            .style(move |_t| iced::widget::container::Style {
                text_color: Some(text_color),
                background: Some(iced::Background::Color(bg)),
                border: iced::Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: iced::border::radius(radius),
                },
                shadow: iced::Shadow::default(),
            })
            .into()
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_avatar_default_circle_cover() {
        let a = Avatar::new();
        assert_eq!(a.shape(), AvatarShape::Circle);
        assert_eq!(a.fit(), AvatarFit::Cover);
        assert_eq!(a.source(), AvatarSource::Icon);
    }
}
