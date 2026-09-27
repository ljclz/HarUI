//! Container 容器组件
//!
//! 参考 Element Plus `<el-container>`：
//! - width: 流式 / 固定宽度
//! - centered: 是否居中显示
//! - padding: 内边距

use iced::Element;

/// 容器宽度策略
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ContainerWidth {
    /// 流式 — 100% 宽度
    Fluid,
    /// 固定宽度（px）
    Fixed(f32),
}

/// Container — 容器
#[derive(Debug, Clone)]
pub struct Container {
    width: ContainerWidth,
    centered: bool,
    padding: f32,
}

impl Container {
    pub fn new() -> Self {
        Self {
            width: ContainerWidth::Fluid,
            centered: false,
            padding: 0.0,
        }
    }

    pub fn with_width(mut self, value: ContainerWidth) -> Self {
        self.width = value;
        self
    }

    pub fn with_centered(mut self, value: bool) -> Self {
        self.centered = value;
        self
    }

    pub fn with_padding(mut self, value: f32) -> Self {
        self.padding = value;
        self
    }

    pub fn width(&self) -> ContainerWidth {
        self.width
    }

    pub fn is_centered(&self) -> bool {
        self.centered
    }

    pub fn padding(&self) -> f32 {
        self.padding
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}

/// 构建容器 Element — 把 content 装入 iced Container
///
/// 简化版签名，等价于 `iced::widget::container(content).into()`。
pub fn container<'a, Message: Clone + 'a>(content: Element<'a, Message>) -> Element<'a, Message> {
    iced::widget::container(content).into()
}
