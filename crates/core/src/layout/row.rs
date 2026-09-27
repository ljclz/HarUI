//! Row 组件 — 水平布局容器
//!
//! 参考 Element Plus `<el-row>` 组件：
//! - gutter: 列间距（px）
//! - justify: 主轴对齐方式
//! - align: 交叉轴对齐方式

use iced::Element;

/// 主轴对齐方式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RowJustify {
    #[default]
    Start,
    End,
    Center,
    SpaceAround,
    SpaceBetween,
    SpaceEvenly,
}

/// 交叉轴对齐方式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RowAlign {
    #[default]
    Top,
    Middle,
    Bottom,
}

/// Row — 行容器
#[derive(Debug, Clone)]
pub struct Row {
    gutter: f32,
    justify: RowJustify,
    align: RowAlign,
}

impl Row {
    pub fn new() -> Self {
        Self {
            gutter: 0.0,
            justify: RowJustify::Start,
            align: RowAlign::Top,
        }
    }

    pub fn with_gutter(mut self, value: f32) -> Self {
        self.gutter = value;
        self
    }

    pub fn with_justify(mut self, value: RowJustify) -> Self {
        self.justify = value;
        self
    }

    pub fn with_align(mut self, value: RowAlign) -> Self {
        self.align = value;
        self
    }

    pub fn gutter(&self) -> f32 {
        self.gutter
    }

    pub fn justify(&self) -> RowJustify {
        self.justify
    }

    pub fn align(&self) -> RowAlign {
        self.align
    }
}

impl Default for Row {
    fn default() -> Self {
        Self::new()
    }
}

/// 构建水平布局 Element — 把 children 装入 iced Row
///
/// 简化版签名，等价于 `iced::widget::Row::with_children(children).into()`。
pub fn row<'a, Message: Clone + 'a>(children: Vec<Element<'a, Message>>) -> Element<'a, Message> {
    iced::widget::Row::with_children(children).into()
}
