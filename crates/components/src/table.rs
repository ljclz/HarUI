//! Table 组件 — 表格
//!
//! 参考 Element Plus `<el-table>` 组件。
//! 支持：列定义、斑马纹、边框、排序、行点击、空数据展示、固定列、虚拟滚动。
//!
//! ## 泛型设计
//! `Table<R>` 中的 R 表示行数据类型，必须实现 `Identifiable` trait 用于
//! row 唯一标识。具体的排序逻辑由调用方在 `TableMessage::SortBy` 处理时回调。
//!
//! ## 虚拟滚动
//! 当行数 > threshold（默认 100）时启用虚拟滚动：只渲染可见区域的行，
//! 滚动时通过 `Scroll(offset)` 消息更新 offset，`visible_rows()` 返回
//! 当前视窗内的行索引区间。500 行布局 < 0.5s（M2 性能预算）。

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
    /// 虚拟滚动：滚动到 offset（像素）
    Scroll(f32),
    /// 虚拟滚动：设置视窗高度（像素）
    SetViewportHeight(f32),
}

/// 虚拟滚动配置
///
/// 当行数超过阈值时启用，仅渲染可见区域的行。
/// 视窗逻辑：visible_start = floor(scroll_offset / row_height)
///          visible_count = ceil(viewport_height / row_height) + 1（缓冲）
#[derive(Debug, Clone)]
pub struct VirtualScroll {
    /// 单行高度（像素）
    pub row_height: f32,
    /// 视窗高度（像素）
    pub viewport_height: f32,
    /// 当前滚动偏移（像素）
    pub scroll_offset: f32,
    /// 启用阈值：行数超过此值才启用虚拟滚动
    pub threshold: usize,
}

impl VirtualScroll {
    pub fn new(row_height: f32, viewport_height: f32) -> Self {
        Self {
            row_height: row_height.max(1.0),
            viewport_height: viewport_height.max(1.0),
            scroll_offset: 0.0,
            threshold: 100,
        }
    }

    pub fn with_threshold(mut self, t: usize) -> Self {
        self.threshold = t;
        self
    }

    /// 是否启用虚拟滚动
    pub fn should_enable(&self, total_rows: usize) -> bool {
        total_rows > self.threshold
    }

    /// 计算可见行索引区间 [start, end)
    pub fn visible_range(&self, total_rows: usize) -> (usize, usize) {
        if total_rows == 0 || self.row_height <= 0.0 {
            return (0, 0);
        }
        let start = ((self.scroll_offset / self.row_height).floor() as usize)
            .saturating_sub(0);
        let visible_count = ((self.viewport_height / self.row_height).ceil() as usize) + 1;
        let end = (start.saturating_add(visible_count)).min(total_rows);
        (start.min(total_rows), end)
    }

    /// 总滚动高度
    pub fn total_height(&self, total_rows: usize) -> f32 {
        total_rows as f32 * self.row_height
    }

    /// 钳制 scroll_offset 到 [0, max_offset]
    pub fn clamp_offset(&mut self, total_rows: usize) {
        let max = (total_rows as f32 * self.row_height - self.viewport_height).max(0.0);
        if self.scroll_offset < 0.0 {
            self.scroll_offset = 0.0;
        }
        if self.scroll_offset > max {
            self.scroll_offset = max;
        }
    }
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
    /// 虚拟滚动配置（None 表示不启用）
    virtual_scroll: Option<VirtualScroll>,
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
            virtual_scroll: None,
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

    /// 启用虚拟滚动
    pub fn with_virtual_scroll(mut self, vs: VirtualScroll) -> Self {
        self.virtual_scroll = Some(vs);
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

    pub fn virtual_scroll(&self) -> Option<&VirtualScroll> {
        self.virtual_scroll.as_ref()
    }

    /// 虚拟滚动是否生效（配置存在且行数超过阈值）
    pub fn is_virtual_scroll_active(&self) -> bool {
        match &self.virtual_scroll {
            Some(vs) => vs.should_enable(self.rows.len()),
            None => false,
        }
    }

    /// 返回当前可见行的索引区间 [start, end)
    /// 未启用虚拟滚动时返回 (0, rows.len())
    pub fn visible_range(&self) -> (usize, usize) {
        match &self.virtual_scroll {
            Some(vs) if vs.should_enable(self.rows.len()) => vs.visible_range(self.rows.len()),
            _ => (0, self.rows.len()),
        }
    }

    /// 返回当前可见行的切片
    pub fn visible_rows(&self) -> &[R] {
        let (start, end) = self.visible_range();
        if start >= self.rows.len() {
            return &[];
        }
        let end = end.min(self.rows.len());
        &self.rows[start..end]
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
            TableMessage::Scroll(offset) => {
                if let Some(vs) = self.virtual_scroll.as_mut() {
                    vs.scroll_offset = offset.max(0.0);
                    vs.clamp_offset(self.rows.len());
                }
            }
            TableMessage::SetViewportHeight(h) => {
                if let Some(vs) = self.virtual_scroll.as_mut() {
                    vs.viewport_height = h.max(1.0);
                    vs.clamp_offset(self.rows.len());
                }
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

    // ============ 虚拟滚动测试 ============

    #[test]
    fn test_virtual_scroll_visible_range_basic() {
        // 1000 行，行高 30，视窗 300 → 可见 10+1=11 行
        let vs = VirtualScroll::new(30.0, 300.0);
        // offset=0 → start=0, end=11
        let (s, e) = vs.visible_range(1000);
        assert_eq!(s, 0);
        assert_eq!(e, 11);
    }

    #[test]
    fn test_virtual_scroll_visible_range_scrolled() {
        let mut vs = VirtualScroll::new(30.0, 300.0);
        vs.scroll_offset = 150.0; // 滚动 5 行
        let (s, e) = vs.visible_range(1000);
        assert_eq!(s, 5);
        assert_eq!(e, 16); // 5 + 11
    }

    #[test]
    fn test_virtual_scroll_clamp_offset() {
        let mut vs = VirtualScroll::new(30.0, 300.0);
        // 1000 行 × 30 = 30000, 视窗 300, max_offset = 29700
        vs.scroll_offset = 99999.0;
        vs.clamp_offset(1000);
        assert_eq!(vs.scroll_offset, 29700.0);
        // 负值钳为 0
        vs.scroll_offset = -100.0;
        vs.clamp_offset(1000);
        assert_eq!(vs.scroll_offset, 0.0);
    }

    #[test]
    fn test_virtual_scroll_threshold() {
        let vs = VirtualScroll::new(30.0, 300.0).with_threshold(50);
        assert!(!vs.should_enable(50)); // 等于阈值不启用
        assert!(vs.should_enable(51));  // 超过阈值启用
    }

    #[test]
    fn test_virtual_scroll_empty_rows() {
        let vs = VirtualScroll::new(30.0, 300.0);
        let (s, e) = vs.visible_range(0);
        assert_eq!(s, 0);
        assert_eq!(e, 0);
    }

    #[test]
    fn test_table_without_virtual_scroll_returns_all_rows() {
        let table: Table<String> = Table::new()
            .with_rows(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
        // 未配置虚拟滚动 → visible_range 返回 (0, 3)
        let (s, e) = table.visible_range();
        assert_eq!(s, 0);
        assert_eq!(e, 3);
        assert_eq!(table.visible_rows().len(), 3);
        assert!(!table.is_virtual_scroll_active());
    }

    #[test]
    fn test_table_with_virtual_scroll_under_threshold() {
        // 行数 50，阈值 100 → 不启用虚拟滚动
        let rows: Vec<String> = (0..50).map(|i| format!("row{}", i)).collect();
        let table: Table<String> = Table::new()
            .with_rows(rows)
            .with_virtual_scroll(VirtualScroll::new(30.0, 300.0));
        assert!(!table.is_virtual_scroll_active());
        let (s, e) = table.visible_range();
        assert_eq!(s, 0);
        assert_eq!(e, 50);
    }

    #[test]
    fn test_table_with_virtual_scroll_over_threshold() {
        // 行数 500，阈值 100 → 启用虚拟滚动
        let rows: Vec<String> = (0..500).map(|i| format!("row{}", i)).collect();
        let mut table: Table<String> = Table::new()
            .with_rows(rows)
            .with_virtual_scroll(VirtualScroll::new(30.0, 300.0));
        assert!(table.is_virtual_scroll_active());

        // 初始 offset=0 → 可见 11 行
        let (s, e) = table.visible_range();
        assert_eq!(s, 0);
        assert_eq!(e, 11);
        assert_eq!(table.visible_rows().len(), 11);
        assert_eq!(table.visible_rows()[0], "row0");

        // 滚动到 offset=150 → start=5
        table.handle(TableMessage::Scroll(150.0));
        let (s, e) = table.visible_range();
        assert_eq!(s, 5);
        assert_eq!(e, 16);
        assert_eq!(table.visible_rows()[0], "row5");
    }

    #[test]
    fn test_table_virtual_scroll_500_rows_layout_under_500ms() {
        // M2 性能预算：500 行布局 < 0.5s
        // 验证：500 行 × 30px = 15000px 总高度，虚拟滚动只渲染 11 行
        let rows: Vec<String> = (0..500).map(|i| format!("row{}", i)).collect();
        let start = std::time::Instant::now();

        let mut table: Table<String> = Table::new()
            .with_rows(rows)
            .with_virtual_scroll(VirtualScroll::new(30.0, 300.0));

        // 模拟滚动 100 次视窗更新
        for i in 0..100 {
            table.handle(TableMessage::Scroll(i as f32 * 30.0));
            let _ = table.visible_rows();
        }

        let elapsed = start.elapsed();
        assert!(
            elapsed.as_millis() < 500,
            "500 rows layout took {:?}, expected < 500ms",
            elapsed
        );
    }

    #[test]
    fn test_table_virtual_scroll_set_viewport_height() {
        let rows: Vec<String> = (0..500).map(|i| format!("row{}", i)).collect();
        let mut table: Table<String> = Table::new()
            .with_rows(rows)
            .with_virtual_scroll(VirtualScroll::new(30.0, 300.0));

        // 视窗 300 → 11 行
        assert_eq!(table.visible_rows().len(), 11);

        // 视窗 600 → 21 行
        table.handle(TableMessage::SetViewportHeight(600.0));
        assert_eq!(table.visible_rows().len(), 21);
    }

    #[test]
    fn test_table_virtual_scroll_offset_beyond_max() {
        // 用 200 行（超过 threshold=100）才能激活虚拟滚动
        let rows: Vec<String> = (0..200).map(|i| format!("row{}", i)).collect();
        let mut table: Table<String> = Table::new()
            .with_rows(rows)
            .with_virtual_scroll(VirtualScroll::new(30.0, 300.0));

        // 滚动超过最大 offset（200×30-300=5700）
        table.handle(TableMessage::Scroll(99999.0));
        let vs = table.virtual_scroll().unwrap();
        assert_eq!(vs.scroll_offset, 5700.0);

        // 最后一页：start=190, end=200
        let (s, e) = table.visible_range();
        assert_eq!(s, 190);
        assert_eq!(e, 200);
    }
}
