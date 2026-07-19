//! Table 组件 — 表格
//!
//! 参考 Element Plus `<el-table>` 组件。
//! 支持：列定义、斑马纹、边框、排序、行点击、空数据展示、固定列。
//!
//! ## 泛型设计
//! `Table<R>` 中的 R 表示行数据类型，必须实现 `Identifiable` trait 用于
//! row 唯一标识。具体的排序逻辑由调用方在 `TableMessage::SortBy` 处理时回调。

use std::collections::BTreeMap;

/// 行数据 trait — 必须能返回唯一标识
pub trait Identifiable {
    fn id(&self) -> String;
}

/// 排序方向
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SortOrder {
    #[default]
    None,
    Ascending,
    Descending,
}

/// 固定列方向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FixedSide {
    Left,
    Right,
}

/// 列定义
#[derive(Debug, Clone)]
pub struct TableColumn {
    pub prop: String,
    pub label: String,
    pub width: Option<f32>,
    pub sortable: bool,
    pub fixed: Option<FixedSide>,
}

impl TableColumn {
    pub fn new(prop: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            prop: prop.into(),
            label: label.into(),
            width: None,
            sortable: false,
            fixed: None,
        }
    }

    pub fn with_width(mut self, w: f32) -> Self {
        self.width = Some(w);
        self
    }

    pub fn sortable(mut self, v: bool) -> Self {
        self.sortable = v;
        self
    }

    pub fn with_fixed(mut self, side: FixedSide) -> Self {
        self.fixed = Some(side);
        self
    }
}

/// Table Props — 配置
#[derive(Debug, Clone)]
pub struct TableProps {
    pub stripe: bool,
    pub border: bool,
    pub empty_text: String,
}

impl Default for TableProps {
    fn default() -> Self {
        Self {
            stripe: false,
            border: false,
            empty_text: "No Data".to_string(),
        }
    }
}

impl TableProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_stripe(mut self, v: bool) -> Self {
        self.stripe = v;
        self
    }

    pub fn with_border(mut self, v: bool) -> Self {
        self.border = v;
        self
    }

    pub fn with_empty_text(mut self, s: impl Into<String>) -> Self {
        self.empty_text = s.into();
        self
    }
}

/// Table 消息
#[derive(Debug, Clone, PartialEq)]
pub enum TableMessage {
    /// 按某列排序
    SortBy(String, SortOrder),
    /// 点击行
    RowClicked(usize),
    /// 清空选中
    ClearSelection,
}

/// 行数据 — 简化实现，仅用 BTreeMap 存储字段
pub type TableRow = BTreeMap<String, String>;

/// Table 组件
#[derive(Debug, Clone)]
pub struct Table<R: Clone> {
    columns: Vec<TableColumn>,
    rows: Vec<R>,
    original_rows: Vec<R>,
    props: TableProps,
    sort_prop: Option<String>,
    sort_order: SortOrder,
    selected_row_index: Option<usize>,
    /// 字段提取器 — 由调用方提供，用于排序时按字段比较
    field_extractor: Option<fn(&R, &str) -> String>,
}

impl<R: Clone> Table<R> {
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            rows: Vec::new(),
            original_rows: Vec::new(),
            props: TableProps::default(),
            sort_prop: None,
            sort_order: SortOrder::None,
            selected_row_index: None,
            field_extractor: None,
        }
    }

    pub fn with_columns(mut self, cols: Vec<TableColumn>) -> Self {
        self.columns = cols;
        self
    }

    pub fn with_rows(mut self, rows: Vec<R>) -> Self {
        self.original_rows = rows.clone();
        self.rows = rows;
        self
    }

    pub fn with_props(mut self, props: TableProps) -> Self {
        self.props = props;
        self
    }

    pub fn with_field_extractor(mut self, f: fn(&R, &str) -> String) -> Self {
        self.field_extractor = Some(f);
        self
    }

    pub fn columns(&self) -> &[TableColumn] {
        &self.columns
    }

    pub fn rows(&self) -> &[R] {
        &self.rows
    }

    pub fn props(&self) -> &TableProps {
        &self.props
    }

    pub fn sort_prop(&self) -> Option<&String> {
        self.sort_prop.as_ref()
    }

    pub fn sort_order(&self) -> SortOrder {
        self.sort_order
    }

    pub fn selected_row_index(&self) -> Option<usize> {
        self.selected_row_index
    }

    /// 处理消息
    pub fn handle(&mut self, msg: TableMessage) {
        match msg {
            TableMessage::SortBy(prop, order) => {
                self.sort_prop = Some(prop.clone());
                self.sort_order = order;
                self.apply_sort(&prop, order);
            }
            TableMessage::RowClicked(idx) => {
                if idx < self.rows.len() {
                    self.selected_row_index = Some(idx);
                }
            }
            TableMessage::ClearSelection => {
                self.selected_row_index = None;
            }
        }
    }

    /// 应用排序
    fn apply_sort(&mut self, prop: &str, order: SortOrder) {
        if order == SortOrder::None {
            // 恢复原始顺序
            self.rows = self.original_rows.clone();
            return;
        }
        let extractor = match self.field_extractor {
            Some(f) => f,
            None => return,
        };
        let mut indexed: Vec<(usize, R)> = self.rows.iter().cloned().enumerate().collect();
        indexed.sort_by(|a, b| {
            let va = extractor(&a.1, prop);
            let vb = extractor(&b.1, prop);
            // 先尝试数字比较
            match (va.parse::<f64>(), vb.parse::<f64>()) {
                (Ok(na), Ok(nb)) => {
                    if na < nb {
                        std::cmp::Ordering::Less
                    } else if na > nb {
                        std::cmp::Ordering::Greater
                    } else {
                        std::cmp::Ordering::Equal
                    }
                }
                _ => va.cmp(&vb),
            }
        });
        if order == SortOrder::Descending {
            indexed.reverse();
        }
        self.rows = indexed.into_iter().map(|(_, r)| r).collect();
    }
}

impl<R: Clone> Default for Table<R> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_table_default_state() {
        let table: Table<String> = Table::new();
        assert_eq!(table.sort_order(), SortOrder::None);
        assert_eq!(table.sort_prop(), None);
        assert_eq!(table.selected_row_index(), None);
        assert!(table.columns().is_empty());
        assert!(table.rows().is_empty());
    }

    #[test]
    fn test_table_column_builder() {
        let col = TableColumn::new("price", "Price")
            .with_width(120.0)
            .sortable(true)
            .with_fixed(FixedSide::Left);
        assert_eq!(col.prop, "price");
        assert_eq!(col.label, "Price");
        assert_eq!(col.width, Some(120.0));
        assert!(col.sortable);
        assert_eq!(col.fixed, Some(FixedSide::Left));
    }

    #[test]
    fn test_table_props_builder() {
        let p = TableProps::new()
            .with_stripe(true)
            .with_border(true)
            .with_empty_text("暂无数据");
        assert!(p.stripe);
        assert!(p.border);
        assert_eq!(p.empty_text, "暂无数据");
    }
}
