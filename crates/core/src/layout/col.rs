//! Col 组件 — 列容器
//!
//! 参考 Element Plus `<el-col>` 组件：
//! - span: 0-24，占父容器宽度的比例（24 表示 100%）
//! - offset: 0-24，左侧偏移列数
//! - push/pull: 0-24，CSS relative 左右偏移
//! - xs/sm/md/lg/xl/xxl: 响应式断点配置

use iced::Element;

/// 列跨度（0-24）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColSpan(u8);

impl ColSpan {
    pub fn new(value: u8) -> Self {
        Self(value.min(24))
    }

    pub fn value(&self) -> u8 {
        self.0
    }

    /// 24 列 — 占满
    #[allow(non_upper_case_globals)]
    pub const Full: Self = Self(24);
    /// 12 列 — 1/2
    #[allow(non_upper_case_globals)]
    pub const Half: Self = Self(12);
    /// 8 列 — 1/3
    #[allow(non_upper_case_globals)]
    pub const Third: Self = Self(8);
    /// 6 列 — 1/4
    #[allow(non_upper_case_globals)]
    pub const Quarter: Self = Self(6);
    /// 0 列 — 不占位
    #[allow(non_upper_case_globals)]
    pub const None: Self = Self(0);
}

impl Default for ColSpan {
    fn default() -> Self {
        Self::Full
    }
}

/// 列偏移（0-24）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColOffset(u8);

impl ColOffset {
    pub fn new(value: u8) -> Self {
        Self(value.min(24))
    }

    pub fn value(&self) -> u8 {
        self.0
    }

    #[allow(non_upper_case_globals)]
    pub const None: Self = Self(0);
}

impl Default for ColOffset {
    fn default() -> Self {
        Self::None
    }
}

/// 响应式断点跨度配置
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponsiveSpan {
    /// 仅 span
    Span(u8),
    /// span + offset
    SpanOffset { span: u8, offset: u8 },
}

/// Col — 列容器
#[derive(Debug, Clone)]
pub struct Col {
    span: ColSpan,
    offset: ColOffset,
    push: u8,
    pull: u8,
    xs: Option<ResponsiveSpan>,
    sm: Option<ResponsiveSpan>,
    md: Option<ResponsiveSpan>,
    lg: Option<ResponsiveSpan>,
    xl: Option<ResponsiveSpan>,
    xxl: Option<ResponsiveSpan>,
}

impl Col {
    pub fn new() -> Self {
        Self {
            span: ColSpan::Full,
            offset: ColOffset::None,
            push: 0,
            pull: 0,
            xs: None,
            sm: None,
            md: None,
            lg: None,
            xl: None,
            xxl: None,
        }
    }

    pub fn with_span(mut self, value: ColSpan) -> Self {
        self.span = value;
        self
    }

    pub fn with_offset(mut self, value: ColOffset) -> Self {
        self.offset = value;
        self
    }

    pub fn with_push(mut self, value: u8) -> Self {
        self.push = value.min(24);
        self
    }

    pub fn with_pull(mut self, value: u8) -> Self {
        self.pull = value.min(24);
        self
    }

    pub fn with_xs(mut self, value: ResponsiveSpan) -> Self {
        self.xs = Some(value);
        self
    }

    pub fn with_sm(mut self, value: ResponsiveSpan) -> Self {
        self.sm = Some(value);
        self
    }

    pub fn with_md(mut self, value: ResponsiveSpan) -> Self {
        self.md = Some(value);
        self
    }

    pub fn with_lg(mut self, value: ResponsiveSpan) -> Self {
        self.lg = Some(value);
        self
    }

    pub fn with_xl(mut self, value: ResponsiveSpan) -> Self {
        self.xl = Some(value);
        self
    }

    pub fn with_xxl(mut self, value: ResponsiveSpan) -> Self {
        self.xxl = Some(value);
        self
    }

    pub fn span(&self) -> ColSpan {
        self.span
    }

    pub fn offset(&self) -> ColOffset {
        self.offset
    }

    pub fn push(&self) -> u8 {
        self.push
    }

    pub fn pull(&self) -> u8 {
        self.pull
    }

    pub fn xs(&self) -> Option<&ResponsiveSpan> {
        self.xs.as_ref()
    }

    pub fn sm(&self) -> Option<&ResponsiveSpan> {
        self.sm.as_ref()
    }

    pub fn md(&self) -> Option<&ResponsiveSpan> {
        self.md.as_ref()
    }

    pub fn lg(&self) -> Option<&ResponsiveSpan> {
        self.lg.as_ref()
    }

    pub fn xl(&self) -> Option<&ResponsiveSpan> {
        self.xl.as_ref()
    }

    pub fn xxl(&self) -> Option<&ResponsiveSpan> {
        self.xxl.as_ref()
    }

    /// 计算列宽度百分比（基于 24 列网格）
    pub fn width_percentage(&self) -> f32 {
        (self.span.value() as f32 / 24.0) * 100.0
    }
}

impl Default for Col {
    fn default() -> Self {
        Self::new()
    }
}

/// 构建垂直布局 Element — 把 children 装入 iced Column
///
/// 简化版签名，等价于 `iced::widget::Column::with_children(children).into()`。
pub fn col<'a, Message: Clone + 'a>(children: Vec<Element<'a, Message>>) -> Element<'a, Message> {
    iced::widget::Column::with_children(children).into()
}
