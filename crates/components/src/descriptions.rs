//! Descriptions 描述列表 — 参考 Element Plus `<el-descriptions>`。
//!
//! 支持：标题、列数、边框、方向、项 span、布局行计算。

use har_ui_core::theme::Theme;
use iced::widget::{container, text};
use iced::{Color, Element, Length, Padding};

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

    /// 渲染单个 label-value 单元格（带边框样式）
    fn render_cell<'a>(
        label: &'a str,
        value: &'a str,
        span: u32,
        col: u32,
        theme: &'a Theme,
        border: bool,
    ) -> Element<'a, ()> {
        let label_color = Color::from(theme.neutral.text_secondary);
        let value_color = Color::from(theme.neutral.text_primary);
        let border_color = Color::from(theme.neutral.border_lighter);

        let label_text = text(label.to_string()).color(label_color).size(14.0);
        let value_text = text(value.to_string()).color(value_color).size(14.0);

        // span 转 FillPortion（确保 col > 0）
        let portion = (span.max(1) as u16).max(1);
        let _ = col;

        let inner = iced::widget::Row::new()
            .push(label_text)
            .push(iced::widget::Space::new().width(Length::Fixed(8.0)))
            .push(value_text)
            .padding(Padding::from([8u16, 12u16]));

        let cell = container(inner)
            .width(Length::FillPortion(portion))
            .style(move |_t| {
                if border {
                    iced::widget::container::Style {
                        text_color: None,
                        background: None,
                        border: iced::Border {
                            color: border_color,
                            width: 1.0,
                            radius: iced::border::radius(0.0),
                        },
                        shadow: iced::Shadow::default(),
                        snap: false,
                    }
                } else {
                    iced::widget::container::Style::default()
                }
            });
        cell.into()
    }

    /// 渲染 Descriptions 为 iced::Element
    pub fn view<'a>(&'a self, theme: &'a Theme) -> Element<'a, ()> {
        let title_color = Color::from(theme.neutral.text_primary);
        let border_color = Color::from(theme.neutral.border_lighter);
        let _ = border_color;

        let mut outer_children: Vec<Element<'a, ()>> = Vec::new();

        // title
        if let Some(t) = &self.title {
            let title_text = text(t.clone()).color(title_color).size(16.0);
            let title_row = container(title_text)
                .width(Length::Fill)
                .padding(Padding::from([12u16, 0u16]));
            outer_children.push(title_row.into());
        }

        if self.items.is_empty() {
            // 空状态
            let empty = container(text("")).width(Length::Fill);
            outer_children.push(empty.into());
        } else {
            // 按布局行渲染
            let rows = self.layout_rows();
            let col = self.column.max(1);
            let border = self.border;

            let mut rows_children: Vec<Element<'a, ()>> = Vec::new();
            for row in &rows {
                let mut row_children: Vec<Element<'a, ()>> = Vec::new();
                let used: u32 = row.iter().map(|i| i.span.max(1)).sum();
                for item in row {
                    row_children.push(Self::render_cell(
                        item.label(),
                        item.value(),
                        item.span(),
                        col,
                        theme,
                        border,
                    ));
                }
                // 补齐空格使整行填满 col
                if used < col {
                    let remainder = col - used;
                    if remainder > 0 {
                        row_children.push(
                            container(text(""))
                                .width(Length::FillPortion(remainder as u16))
                                .into(),
                        );
                    }
                }
                let row_el = iced::widget::Row::with_children(row_children).spacing(0);
                rows_children.push(row_el.into());
            }
            let rows_col = iced::widget::Column::with_children(rows_children).spacing(0);
            outer_children.push(rows_col.into());
        }

        // extra slot
        if self.has_extra {
            outer_children.push(
                container(
                    text("[extra slot]")
                        .color(Color::from(theme.neutral.text_placeholder))
                        .size(12.0),
                )
                .width(Length::Fill)
                .padding(Padding::from([8u16, 0u16]))
                .into(),
            );
        }

        let outer = iced::widget::Column::with_children(outer_children).spacing(0);
        container(outer)
            .width(Length::Fill)
            .padding(Padding::from(0u16))
            .into()
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
