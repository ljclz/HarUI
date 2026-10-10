//! Watermark 水印 — 参考 Element Plus `<el-watermark>`
//!
//! 状态无关的包装组件：在内容之上叠加半透明文字水印层。
//!
//! ## 实现说明
//! iced 0.14 的基础 text/容器不支持旋转与平铺，本组件渲染**居中单条**
//! 半透明水印（文本/字号/透明度可配）；平铺+旋转需 canvas 自绘 widget（后续项）。
//! 防篡改提示：watermark 配置仅是装饰层，不能阻止内容被复制。

use har_ui_core::theme::Theme;
use iced::widget::{container, text};
use iced::{Element, Length};

/// 水印配置
#[derive(Debug, Clone)]
pub struct Watermark {
    text: String,
    font_size: f32,
    opacity: f32,
}

impl Default for Watermark {
    fn default() -> Self {
        Self::new("HarUI")
    }
}

impl Watermark {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            font_size: 16.0,
            opacity: 0.15,
        }
    }

    pub fn with_font_size(mut self, size: f32) -> Self {
        self.font_size = size.clamp(8.0, 72.0);
        self
    }

    /// 水印透明度（0.0-1.0，内部钳制到 [0.02, 0.6] 防不可见/遮挡）
    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity.clamp(0.02, 0.6);
        self
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn font_size(&self) -> f32 {
        self.font_size
    }

    pub fn opacity(&self) -> f32 {
        self.opacity
    }

    /// 渲染：`content` 为业务内容（原样透传），水印层叠放于上
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        _theme: &'a Theme,
        content: Element<'a, Message>,
    ) -> Element<'a, Message> {
        use iced::widget::stack;
        let watermark = text(self.text.clone())
            .size(self.font_size)
            .color(iced::Color {
                a: self.opacity,
                ..iced::Color::BLACK
            });
        let layer = container(watermark)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill);
        stack![content, layer].into()
    }

    /// 便捷方法：按主题文字色生成水印色（深浅主题共用 text_primary，
    /// 依赖透明度区分层次）
    pub fn themed_color(theme: &Theme) -> iced::Color {
        iced::Color::from(theme.neutral.text_primary)
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let w = Watermark::default();
        assert_eq!(w.text(), "HarUI");
        assert_eq!(w.font_size(), 16.0);
        assert_eq!(w.opacity(), 0.15);
    }

    #[test]
    fn test_opacity_clamped() {
        assert_eq!(Watermark::new("x").with_opacity(1.5).opacity(), 0.6);
        assert_eq!(Watermark::new("x").with_opacity(0.0).opacity(), 0.02);
        assert_eq!(Watermark::new("x").with_opacity(0.3).opacity(), 0.3);
    }

    #[test]
    fn test_font_size_clamped() {
        assert_eq!(Watermark::new("x").with_font_size(200.0).font_size(), 72.0);
        assert_eq!(Watermark::new("x").with_font_size(1.0).font_size(), 8.0);
    }

    #[test]
    fn test_view_renders() {
        let theme = Theme::element_light();
        let w = Watermark::new("机密文件");
        let content = text("业务内容").into();
        let _element = w.view::<()>(&theme, content);
    }
}
