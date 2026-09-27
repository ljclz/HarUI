//! Skeleton 骨架屏 — 参考 Element Plus `<el-skeleton>`。
//!
//! 支持：段落/标题/头像/图片 4 种样式、animated、count 重复、loading 切换。

use har_ui_core::theme::Theme;
use iced::widget::{container, text};
use iced::{Color, Element, Length};

/// 骨架项变体
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkeletonVariant {
    Paragraph,
    Title,
    Avatar,
    Image,
}

/// 骨架项
#[derive(Debug, Clone)]
pub struct SkeletonItem {
    variant: SkeletonVariant,
    width: Option<String>,
    height: Option<String>,
}

impl SkeletonItem {
    pub fn paragraph() -> Self {
        Self {
            variant: SkeletonVariant::Paragraph,
            width: None,
            height: None,
        }
    }

    pub fn title() -> Self {
        Self {
            variant: SkeletonVariant::Title,
            width: None,
            height: None,
        }
    }

    pub fn avatar() -> Self {
        Self {
            variant: SkeletonVariant::Avatar,
            width: None,
            height: None,
        }
    }

    pub fn image() -> Self {
        Self {
            variant: SkeletonVariant::Image,
            width: None,
            height: None,
        }
    }

    pub fn with_width(mut self, w: impl Into<String>) -> Self {
        self.width = Some(w.into());
        self
    }

    pub fn with_height(mut self, h: impl Into<String>) -> Self {
        self.height = Some(h.into());
        self
    }

    pub fn variant(&self) -> SkeletonVariant {
        self.variant
    }

    pub fn width(&self) -> Option<&str> {
        self.width.as_deref()
    }

    pub fn height(&self) -> Option<&str> {
        self.height.as_deref()
    }
}

/// Skeleton 组件
#[derive(Debug, Clone)]
pub struct Skeleton {
    /// 模板（用于按 count 重复）
    template: Vec<SkeletonItem>,
    /// 实际展开后的项列表
    items: Vec<SkeletonItem>,
    animated: bool,
    loading: bool,
    count: u32,
}

impl Default for Skeleton {
    fn default() -> Self {
        Self::new()
    }
}

impl Skeleton {
    pub fn new() -> Self {
        Self {
            template: Vec::new(),
            items: Vec::new(),
            animated: false,
            loading: false,
            count: 1,
        }
    }

    pub fn with_item(mut self, item: SkeletonItem) -> Self {
        self.template.push(item.clone());
        self.rebuild_items();
        self
    }

    pub fn with_template(mut self, items: Vec<SkeletonItem>) -> Self {
        self.template = items;
        self.rebuild_items();
        self
    }

    pub fn with_animated(mut self, v: bool) -> Self {
        self.animated = v;
        self
    }

    pub fn with_loading(mut self, v: bool) -> Self {
        self.loading = v;
        self
    }

    pub fn with_count(mut self, c: u32) -> Self {
        self.count = c;
        self.rebuild_items();
        self
    }

    pub fn set_loading(&mut self, v: bool) {
        self.loading = v;
    }

    pub fn items(&self) -> &[SkeletonItem] {
        &self.items
    }

    pub fn animated(&self) -> bool {
        self.animated
    }

    pub fn loading(&self) -> bool {
        self.loading
    }

    pub fn count(&self) -> u32 {
        self.count
    }

    fn rebuild_items(&mut self) {
        self.items.clear();
        if self.count == 0 {
            return;
        }
        for _ in 0..self.count {
            for item in &self.template {
                self.items.push(item.clone());
            }
        }
    }

    /// 渲染单个骨架项
    fn render_item<'a>(item: &'a SkeletonItem, theme: &'a Theme) -> Element<'a, ()> {
        let bg = Color::from(theme.neutral.border_lighter);
        let (width, height) = match item.variant {
            SkeletonVariant::Paragraph => (Length::Fill, Length::Fixed(16.0)),
            SkeletonVariant::Title => (Length::Fixed(200.0), Length::Fixed(24.0)),
            SkeletonVariant::Avatar => (Length::Fixed(40.0), Length::Fixed(40.0)),
            SkeletonVariant::Image => (Length::Fixed(120.0), Length::Fixed(120.0)),
        };
        // 自定义 width/height 覆盖（支持 "120px" / "120" 格式）
        let width = item
            .width
            .as_deref()
            .and_then(parse_px)
            .map(Length::Fixed)
            .unwrap_or(width);
        let height = item
            .height
            .as_deref()
            .and_then(parse_px)
            .map(Length::Fixed)
            .unwrap_or(height);
        let radius = match item.variant {
            SkeletonVariant::Avatar => 20.0,
            _ => 4.0,
        };
        container(text(""))
            .width(width)
            .height(height)
            .style(move |_t| iced::widget::container::Style {
                text_color: None,
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

    /// 渲染 Skeleton 为 iced::Element
    pub fn view<'a>(&'a self, theme: &'a Theme) -> Element<'a, ()> {
        // loading=false 时不显示骨架屏（占位为空）
        if !self.loading && !self.items.is_empty() {
            // 实际项目应渲染真实内容；这里 loading=false 时返回空
            return container(text("")).into();
        }
        if self.items.is_empty() {
            return container(text("")).into();
        }

        let children: Vec<Element<'a, ()>> = self
            .items
            .iter()
            .map(|item| Self::render_item(item, theme))
            .collect();

        iced::widget::Column::with_children(children)
            .spacing(12)
            .into()
    }
}

/// 解析 "120px" / "120" 为 f32
fn parse_px(s: &str) -> Option<f32> {
    let s = s.trim_end_matches("px").trim();
    s.parse::<f32>().ok()
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_skeleton_default() {
        let s = Skeleton::new();
        assert!(!s.animated());
        assert!(!s.loading());
        assert_eq!(s.count(), 1);
        assert!(s.items().is_empty());
    }
}
