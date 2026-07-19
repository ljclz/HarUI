//! Descriptions 描述列表 — 参考 Element Plus `<el-descriptions>`。
//!
//! 支持：标题、列数、边框、方向、项 span、布局行计算。

/// 方向
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DescriptionsDirection {
    #[default]
    Horizontal,
    Vertical,
}

/// 描述项
#[derive(Debug, Clone)]
pub struct DescriptionsItem {
    label: String,
    value: String,
    span: u32,
}

impl DescriptionsItem {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            span: 1,
        }
    }

    pub fn with_span(mut self, s: u32) -> Self {
        self.span = s.max(1);
        self
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn span(&self) -> u32 {
        self.span
    }
}

/// Descriptions 组件
#[derive(Debug, Clone)]
pub struct Descriptions {
    title: Option<String>,
    column: u32,
    border: bool,
    direction: DescriptionsDirection,
    items: Vec<DescriptionsItem>,
    has_extra: bool,
}

impl Default for Descriptions {
    fn default() -> Self {
        Self::new()
    }
}

impl Descriptions {
    pub fn new() -> Self {
        Self {
            title: None,
            column: 3,
            border: true,
            direction: DescriptionsDirection::Horizontal,
            items: Vec::new(),
            has_extra: false,
        }
    }

    pub fn with_title(mut self, t: impl Into<String>) -> Self {
        self.title = Some(t.into());
        self
    }

    pub fn with_column(mut self, c: u32) -> Self {
        self.column = c.max(1);
        self
    }

    pub fn with_border(mut self, b: bool) -> Self {
        self.border = b;
        self
    }

    pub fn with_direction(mut self, d: DescriptionsDirection) -> Self {
        self.direction = d;
        self
    }

    pub fn with_item(mut self, item: DescriptionsItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn with_has_extra(mut self, v: bool) -> Self {
        self.has_extra = v;
        self
    }

    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    pub fn column(&self) -> u32 {
        self.column
    }

    pub fn border(&self) -> bool {
        self.border
    }

    pub fn direction(&self) -> DescriptionsDirection {
        self.direction
    }

    pub fn items(&self) -> &[DescriptionsItem] {
        &self.items
    }

    pub fn has_extra(&self) -> bool {
        self.has_extra
    }

    /// 按列数和 span 计算布局行
    pub fn layout_rows(&self) -> Vec<Vec<&DescriptionsItem>> {
        let mut rows: Vec<Vec<&DescriptionsItem>> = Vec::new();
        let mut current_row: Vec<&DescriptionsItem> = Vec::new();
        let mut current_span: u32 = 0;
        let col = self.column.max(1);

        for item in &self.items {
            let span = item.span.max(1);
            if current_span + span > col && !current_row.is_empty() {
                rows.push(std::mem::take(&mut current_row));
                current_span = 0;
            }
            current_span += span;
            current_row.push(item);
            if current_span >= col {
                rows.push(std::mem::take(&mut current_row));
                current_span = 0;
            }
        }
        if !current_row.is_empty() {
            rows.push(current_row);
        }
        rows
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_descriptions_default_column_3() {
        let d = Descriptions::new();
        assert_eq!(d.column(), 3);
        assert!(d.border());
        assert_eq!(d.direction(), DescriptionsDirection::Horizontal);
    }
}
