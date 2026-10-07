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
use std::rc::Rc;

use har_ui_core::theme::Theme;
use iced::widget::{container, mouse_area, scrollable, text};
use iced::{Color, Element, Length, Padding};

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
    /// 列宽拖拽边界 (min, max) 像素；Some 时表头出现拖拽条（view_msg 模式）
    pub resizable: Option<(f32, f32)>,
}

impl TableColumn {
    pub fn new(prop: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            prop: prop.into(),
            label: label.into(),
            width: None,
            sortable: false,
            fixed: None,
            resizable: None,
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

    /// 设置列宽拖拽边界（min, max）像素，min 下限 20；设置后该列表头出现拖拽条
    pub fn with_resize_bounds(mut self, min: f32, max: f32) -> Self {
        let lo = min.max(20.0);
        self.resizable = Some((lo, max.max(lo)));
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
    /// 横向滚动：滚动到 offset（像素）— 冻结列布局下由 view_msg 的横向 scrollable 接线
    ScrollX(f32),
    /// 程序化列宽调整：列索引 + 像素增量（正=变宽），钳制到拖拽边界（无边界时 [40, +∞)）
    ResizeColumn(usize, f32),
    /// 列宽拖拽开始：列索引（表头拖拽条 on_press；锚点在首次 on_move 时建立）
    ResizeStart(usize),
    /// 列宽拖拽移动：光标 x（表头拖拽条 on_move；仅在拖拽会话中生效）
    ResizeMove(f32),
    /// 列宽拖拽结束
    ResizeEnd,
}

/// 虚拟滚动配置
///
/// 实现已上收至 core 行为层 [`har_ui_core::behavior::virtual_list::VirtualList`]（ADR-009），
/// 本别名保持原路径 / 原名 / 原方法，数学逐行等价。
/// 当行数超过阈值时启用，仅渲染可见区域的行。
pub type VirtualScroll = har_ui_core::behavior::virtual_list::VirtualList;

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
    /// 横向滚动偏移（像素，冻结列布局下有意义）
    scroll_x: f32,
    /// 横向视窗宽度（Some 时参与 scroll_x 钳制计算）
    h_viewport_width: Option<f32>,
    /// 运行时列宽覆盖（拖拽/程序化 resize 结果），与 columns 等长，None 表示未覆盖
    column_widths: Vec<Option<f32>>,
    /// 进行中的拖拽会话：(列索引, 上次光标 x — 首次 on_move 时锚定)
    resize_state: Option<(usize, Option<f32>)>,
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
            scroll_x: 0.0,
            h_viewport_width: None,
            column_widths: Vec::new(),
            resize_state: None,
        }
    }

    pub fn with_columns(mut self, cols: Vec<TableColumn>) -> Self {
        self.column_widths = vec![None; cols.len()];
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

    /// 设置横向视窗宽度（像素）。启用冻结列布局的 scroll_x 钳制计算；
    /// 无 fixed 列时也可单独用于开启中段横向滚动（ADR-008）
    pub fn with_horizontal_viewport(mut self, w: f32) -> Self {
        self.h_viewport_width = Some(w.max(1.0));
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

    /// 横向滚动偏移（像素）
    pub fn scroll_x(&self) -> f32 {
        self.scroll_x
    }

    /// 横向视窗宽度配置
    pub fn h_viewport_width(&self) -> Option<f32> {
        self.h_viewport_width
    }

    /// 运行时列宽（拖拽/程序化覆盖优先于列默认宽），越界返回 None
    pub fn resolved_width(&self, idx: usize) -> Option<f32> {
        if idx >= self.columns.len() {
            return None;
        }
        self.column_widths
            .get(idx)
            .copied()
            .flatten()
            .or(self.columns[idx].width)
    }

    /// 是否启用冻结列布局：任一列带 fixed，或显式设置了横向视窗（ADR-008）
    pub fn is_frozen_layout(&self) -> bool {
        self.columns.iter().any(|c| c.fixed.is_some()) || self.h_viewport_width.is_some()
    }

    /// 列按冻结方向三段划分：(左冻结, 中段可滚, 右冻结)，各段内保持原相对顺序
    pub fn fixed_partition(&self) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
        let mut left = Vec::new();
        let mut mid = Vec::new();
        let mut right = Vec::new();
        for (i, col) in self.columns.iter().enumerate() {
            match col.fixed {
                Some(FixedSide::Left) => left.push(i),
                Some(FixedSide::Right) => right.push(i),
                None => mid.push(i),
            }
        }
        (left, mid, right)
    }

    /// 冻结列总宽（左 + 右）。未显式宽度的冻结列以 100px 兜底（无法自动测宽，ADR-008）
    pub fn frozen_total_width(&self) -> f32 {
        let (left, _, right) = self.fixed_partition();
        left.iter()
            .chain(right.iter())
            .map(|&i| self.resolved_width(i).unwrap_or(100.0))
            .sum()
    }

    /// 中段内容总宽：仅统计显式宽度列（Fill 列不产生横向滚动需求）
    pub fn mid_content_width(&self) -> f32 {
        let (_, mid, _) = self.fixed_partition();
        mid.iter().filter_map(|&i| self.resolved_width(i)).sum()
    }

    /// 中段可用宽度 = 横向视窗 - 冻结总宽（未设置视窗时为 0，即不做上限钳制）
    pub fn mid_available_width(&self) -> f32 {
        match self.h_viewport_width {
            Some(w) => (w - self.frozen_total_width()).max(0.0),
            None => 0.0,
        }
    }

    /// 钳制横向滚动偏移到 [0, max]
    pub fn clamp_scroll_x(&mut self) {
        let max = (self.mid_content_width() - self.mid_available_width()).max(0.0);
        if self.scroll_x < 0.0 {
            self.scroll_x = 0.0;
        }
        if self.scroll_x > max {
            self.scroll_x = max;
        }
    }

    /// 列宽调整核心：在当前运行时宽度上叠加增量并钳制。
    /// 有 resizable 边界按边界；无边界时程序化调整兜底 [40, +∞)（ADR-008）
    ///
    /// 调整后同步重新钳制 scroll_x：列宽收缩可能使当前横向偏移越界
    pub fn resize_column(&mut self, idx: usize, delta: f32) {
        if idx >= self.columns.len() {
            return;
        }
        let base = self.resolved_width(idx).unwrap_or(100.0);
        let (min, max) = self.columns[idx].resizable.unwrap_or((40.0, f32::MAX));
        let new_w = (base + delta).clamp(min, max);
        if self.column_widths.len() != self.columns.len() {
            self.column_widths = vec![None; self.columns.len()];
        }
        self.column_widths[idx] = Some(new_w);
        self.clamp_scroll_x();
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
            TableMessage::ScrollX(offset) => {
                self.scroll_x = offset.max(0.0);
                self.clamp_scroll_x();
            }
            TableMessage::ResizeColumn(idx, delta) => {
                self.resize_column(idx, delta);
            }
            TableMessage::ResizeStart(idx) => {
                // 仅可拖拽列（resizable 非空）允许开启拖拽会话
                if idx < self.columns.len() && self.columns[idx].resizable.is_some() {
                    self.resize_state = Some((idx, None));
                }
            }
            TableMessage::ResizeMove(x) => {
                if let Some((idx, last_x)) = self.resize_state {
                    match last_x {
                        // 首次 on_move：锚定起点，不产生宽度变化
                        None => self.resize_state = Some((idx, Some(x))),
                        Some(last) => {
                            self.resize_column(idx, x - last);
                            self.resize_state = Some((idx, Some(x)));
                        }
                    }
                }
            }
            TableMessage::ResizeEnd => {
                self.resize_state = None;
            }
        }
    }

    /// 渲染表格为 iced::Element（渲染模式：不接线任何交互事件）
    ///
    /// # 参数
    /// - `theme`: HarUI 主题引用
    /// - `cell_renderer`: 单元格文本提取器（行数据 + 字段名 → 字符串）
    /// - `on_row_click`: 行点击回调（保留参数；渲染模式下行点击不接线，历史行为）
    ///
    /// # 行为
    /// - 表头：列标签横向排列
    /// - 数据行：通过 cell_renderer 提取每个单元格文本
    /// - 斑马纹：奇数行使用浅色背景
    /// - 边框：可选 border_lighter 分隔线
    /// - 空数据：显示 empty_text
    /// - 虚拟滚动：仅渲染 visible_rows()；垂直滚动由应用层发 `TableMessage::Scroll` 驱动
    /// - 冻结列：任一列带 `fixed` 或设置 `with_horizontal_viewport` 时切换为三段式布局
    /// - 需要接线横向滚动 / 列宽拖拽 / 行点击时，使用 [`Table::view_msg`]
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        cell_renderer: impl Fn(&R, &str) -> String + 'a,
        on_row_click: impl Fn(usize) -> Message + 'a,
    ) -> Element<'a, Message> {
        let _ = on_row_click; // 保留既有语义：渲染模式行点击不接线（原 L643 注释）
        self.render(theme, cell_renderer, None)
    }

    /// 交互模式渲染：通过 `on_msg` 统一接收全部 `TableMessage`
    ///
    /// 相比 [`Table::view`] 额外接线：
    /// - 行点击 → `TableMessage::RowClicked`
    /// - 冻结布局中段横向滚动 → `TableMessage::ScrollX`
    /// - 可拖拽列（`with_resize_bounds`）表头拖拽条 → `ResizeStart/ResizeMove/ResizeEnd`
    pub fn view_msg<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        cell_renderer: impl Fn(&R, &str) -> String + 'a,
        on_msg: impl Fn(TableMessage) -> Message + 'a,
    ) -> Element<'a, Message> {
        let msg: Rc<dyn Fn(TableMessage) -> Message + 'a> = Rc::new(on_msg);
        self.render(theme, cell_renderer, Some(msg))
    }

    /// 渲染核心：按 `is_frozen_layout()` 分派到冻结三段式或既有布局
    fn render<'a, Message: Clone + 'a, CR>(
        &'a self,
        theme: &'a Theme,
        cell_renderer: CR,
        table_msg: Option<Rc<dyn Fn(TableMessage) -> Message + 'a>>,
    ) -> Element<'a, Message>
    where
        CR: Fn(&R, &str) -> String + 'a,
    {
        if self.is_frozen_layout() {
            self.render_frozen(theme, &cell_renderer, table_msg.as_ref())
        } else {
            self.render_legacy(theme, &cell_renderer, table_msg)
        }
    }

    /// 既有布局（非冻结）：表头行 + 垂直 scrollable 表体。行为与 v1.0.0 完全一致，
    /// 仅在 view_msg 模式下为数据行补接线 RowClicked。
    fn render_legacy<'a, Message: Clone + 'a, CR>(
        &'a self,
        theme: &'a Theme,
        cell_renderer: &CR,
        table_msg: Option<Rc<dyn Fn(TableMessage) -> Message + 'a>>,
    ) -> Element<'a, Message>
    where
        CR: Fn(&R, &str) -> String + 'a,
    {
        let border_color = Color::from(theme.neutral.border_lighter);
        let header_bg = Color::from(theme.neutral.bg_base);
        let stripe_bg = Color {
            a: 0.5,
            ..Color::from(theme.neutral.bg_base)
        };
        let selected_bg = Color {
            a: 0.1,
            ..Color::from(theme.primary.base)
        };
        let text_color = Color::from(theme.neutral.text_regular);
        let header_text_color = Color::from(theme.neutral.text_primary);
        let empty_text_color = Color::from(theme.neutral.text_placeholder);

        // 表头行
        let header_cells: Vec<Element<'a, Message>> = self
            .columns
            .iter()
            .map(|col| {
                let label = if col.sortable {
                    let arrow = match (self.sort_prop.as_deref(), self.sort_order) {
                        (Some(p), SortOrder::Ascending) if p == col.prop => " ↑",
                        (Some(p), SortOrder::Descending) if p == col.prop => " ↓",
                        _ => "",
                    };
                    format!("{}{}", col.label, arrow)
                } else {
                    col.label.clone()
                };
                let t = text(label).color(header_text_color);
                let w = col.width.map(Length::Fixed).unwrap_or(Length::Fill);
                let cell = container(t)
                    .width(w)
                    .padding(Padding::from([8u16, 12u16]))
                    .style(move |_t| iced::widget::container::Style {
                        text_color: Some(header_text_color),
                        background: Some(iced::Background::Color(header_bg)),
                        border: iced::Border {
                            color: border_color,
                            width: if self.props.border { 1.0 } else { 0.0 },
                            radius: iced::border::Radius::default(),
                        },
                        shadow: iced::Shadow::default(),
                        snap: false,
                    });
                cell.into()
            })
            .collect();
        let header_row = iced::widget::Row::with_children(header_cells);

        // 数据行
        let (vstart, _) = self.visible_range();
        let visible_rows = self.visible_rows();
        let mut body_children: Vec<Element<'a, Message>> = Vec::with_capacity(visible_rows.len());

        if visible_rows.is_empty() {
            // 空数据
            let empty = container(text(self.props.empty_text.clone()).color(empty_text_color))
                .width(Length::Fill)
                .padding(Padding::from([20u16, 12u16]))
                .center_x(Length::Fill);
            body_children.push(empty.into());
        } else {
            for (i, row_data) in visible_rows.iter().enumerate() {
                let actual_idx = vstart + i;
                let is_selected = self.selected_row_index == Some(actual_idx);
                let is_stripe = self.props.stripe && (actual_idx % 2 == 1);
                let bg = if is_selected {
                    selected_bg
                } else if is_stripe {
                    stripe_bg
                } else {
                    Color::TRANSPARENT
                };
                let row_text_color = text_color;

                let cells: Vec<Element<'a, Message>> = self
                    .columns
                    .iter()
                    .map(|col| {
                        let cell_text = cell_renderer(row_data, &col.prop);
                        let w = col.width.map(Length::Fixed).unwrap_or(Length::Fill);
                        let t = text(cell_text).color(row_text_color);
                        container(t)
                            .width(w)
                            .padding(Padding::from([8u16, 12u16]))
                            .style(move |_t| iced::widget::container::Style {
                                text_color: Some(row_text_color),
                                background: Some(iced::Background::Color(Color::TRANSPARENT)),
                                border: iced::Border {
                                    color: border_color,
                                    width: if self.props.border { 1.0 } else { 0.0 },
                                    radius: iced::border::Radius::default(),
                                },
                                shadow: iced::Shadow::default(),
                                snap: false,
                            })
                            .into()
                    })
                    .collect();
                let data_row_inner = iced::widget::Row::with_children(cells);
                let data_row = container(data_row_inner)
                    .width(Length::Fill)
                    .style(move |_t| iced::widget::container::Style {
                        text_color: None,
                        background: Some(iced::Background::Color(bg)),
                        border: iced::Border {
                            color: border_color,
                            width: if self.props.border { 1.0 } else { 0.0 },
                            radius: iced::border::Radius::default(),
                        },
                        shadow: iced::Shadow::default(),
                        snap: false,
                    });
                // view_msg 模式：行点击接线（渲染模式保持原行为不接线）
                let row_elem: Element<'a, Message> = match &table_msg {
                    Some(rc) => {
                        let rc = Rc::clone(rc);
                        mouse_area(data_row)
                            .on_press(rc(TableMessage::RowClicked(actual_idx)))
                            .into()
                    }
                    None => data_row.into(),
                };
                body_children.push(row_elem);
            }
        }

        let body_col = iced::widget::Column::with_children(body_children);

        // 虚拟滚动：垂直方向由应用层消息驱动（TableMessage::Scroll），此处仅包裹布局
        let scrollable_body = scrollable(body_col).height(Length::Fill);

        let table_col =
            iced::widget::Column::with_children(vec![header_row.into(), scrollable_body.into()]);
        Element::from(table_col)
    }

    /// 冻结列三段式布局（ADR-008）：
    /// `Row[ 左冻结(头+体) | 横向可滚中段(头+体) | 右冻结(头+体) ]`
    ///
    /// - 头/体横向同步：中段表头与表体放进同一个横向 scrollable，天然同步，无双 scrollable 同步负担
    /// - 垂直方向：消息驱动虚拟滚动（渲染 visible_rows，无垂直 scrollable），与 demo 的
    ///   按钮/事件驱动模式一致
    /// - 冻结列未显式宽度时以 100px 兜底（iced 无自动测宽）
    fn render_frozen<'a, Message: Clone + 'a, CR>(
        &'a self,
        theme: &'a Theme,
        cell_renderer: &CR,
        table_msg: Option<&Rc<dyn Fn(TableMessage) -> Message + 'a>>,
    ) -> Element<'a, Message>
    where
        CR: Fn(&R, &str) -> String + 'a,
    {
        let border_color = Color::from(theme.neutral.border_lighter);
        let header_bg = Color::from(theme.neutral.bg_base);
        let stripe_bg = Color {
            a: 0.5,
            ..Color::from(theme.neutral.bg_base)
        };
        let selected_bg = Color {
            a: 0.1,
            ..Color::from(theme.primary.base)
        };
        let text_color = Color::from(theme.neutral.text_regular);
        let header_text_color = Color::from(theme.neutral.text_primary);
        let empty_text_color = Color::from(theme.neutral.text_placeholder);
        let border_on = self.props.border;

        let (left_idx, mid_idx, right_idx) = self.fixed_partition();
        let seg_width = |indices: &[usize]| -> f32 {
            indices
                .iter()
                .map(|&i| self.resolved_width(i).unwrap_or(100.0))
                .sum()
        };
        let left_w = seg_width(&left_idx);
        let right_w = seg_width(&right_idx);

        // —— 表头单元格：排序箭头 + 可选拖拽条 ——
        let header_cell = |idx: usize| -> Element<'a, Message> {
            let col = &self.columns[idx];
            let label = if col.sortable {
                let arrow = match (self.sort_prop.as_deref(), self.sort_order) {
                    (Some(p), SortOrder::Ascending) if p == col.prop => " ↑",
                    (Some(p), SortOrder::Descending) if p == col.prop => " ↓",
                    _ => "",
                };
                format!("{}{}", col.label, arrow)
            } else {
                col.label.clone()
            };
            let t = text(label).color(header_text_color);
            // 冻结列固定宽（兜底 100px）；中段未显式宽度用 Fill
            let w = match col.fixed {
                Some(_) => Length::Fixed(self.resolved_width(idx).unwrap_or(100.0)),
                None => self
                    .resolved_width(idx)
                    .map(Length::Fixed)
                    .unwrap_or(Length::Fill),
            };
            let cell = container(t)
                .width(w)
                .padding(Padding::from([8u16, 12u16]))
                .style(move |_t| iced::widget::container::Style {
                    text_color: Some(header_text_color),
                    background: Some(iced::Background::Color(header_bg)),
                    border: iced::Border {
                        color: border_color,
                        width: if border_on { 1.0 } else { 0.0 },
                        radius: iced::border::Radius::default(),
                    },
                    shadow: iced::Shadow::default(),
                    snap: false,
                });
            match (table_msg, col.resizable) {
                (Some(rc), Some(_)) => {
                    let rc_press = Rc::clone(rc);
                    let rc_move = Rc::clone(rc);
                    let rc_release = Rc::clone(rc);
                    // 拖拽条高度 = 表头高（文本 16px + 上下 padding 8px），固定值：
                    // 若用 Fill 会使整列 size_hint 变为 Fill，触发 scrollable
                    // "content must not fill vertical axis" 构造期断言
                    let drag_bar = mouse_area(
                        container(text(""))
                            .width(6)
                            .height(Length::Fixed(32.0))
                            .style(move |_t| iced::widget::container::Style {
                                text_color: None,
                                background: Some(iced::Background::Color(border_color)),
                                border: iced::Border::default(),
                                shadow: iced::Shadow::default(),
                                snap: false,
                            }),
                    )
                    .on_press(rc_press(TableMessage::ResizeStart(idx)))
                    .on_move(move |p| rc_move(TableMessage::ResizeMove(p.x)))
                    .on_release(rc_release(TableMessage::ResizeEnd));
                    iced::widget::Row::with_children(vec![cell.into(), drag_bar.into()]).into()
                }
                _ => cell.into(),
            }
        };

        // —— 数据单元格 ——
        let body_cell = |idx: usize, row_data: &R| -> Element<'a, Message> {
            let col = &self.columns[idx];
            let cell_text = cell_renderer(row_data, &col.prop);
            let w = match col.fixed {
                Some(_) => Length::Fixed(self.resolved_width(idx).unwrap_or(100.0)),
                None => self
                    .resolved_width(idx)
                    .map(Length::Fixed)
                    .unwrap_or(Length::Fill),
            };
            let t = text(cell_text).color(text_color);
            container(t)
                .width(w)
                .padding(Padding::from([8u16, 12u16]))
                .style(move |_t| iced::widget::container::Style {
                    text_color: Some(text_color),
                    background: Some(iced::Background::Color(Color::TRANSPARENT)),
                    border: iced::Border {
                        color: border_color,
                        width: if border_on { 1.0 } else { 0.0 },
                        radius: iced::border::Radius::default(),
                    },
                    shadow: iced::Shadow::default(),
                    snap: false,
                })
                .into()
        };

        // —— 单行在各段中的渲染（返回 左/中/右 三段行元素）——
        let (vstart, _) = self.visible_range();
        let visible_rows = self.visible_rows();

        let seg_row = |indices: &[usize],
                       row_data: &R,
                       bg: Color,
                       actual_idx: usize|
         -> Element<'a, Message> {
            let cells: Vec<Element<'a, Message>> =
                indices.iter().map(|&i| body_cell(i, row_data)).collect();
            let row = container(iced::widget::Row::with_children(cells))
                .width(Length::Fill)
                .style(move |_t| iced::widget::container::Style {
                    text_color: None,
                    background: Some(iced::Background::Color(bg)),
                    border: iced::Border {
                        color: border_color,
                        width: if border_on { 1.0 } else { 0.0 },
                        radius: iced::border::Radius::default(),
                    },
                    shadow: iced::Shadow::default(),
                    snap: false,
                });
            match table_msg {
                Some(rc) => {
                    let rc = Rc::clone(rc);
                    mouse_area(row)
                        .on_press(rc(TableMessage::RowClicked(actual_idx)))
                        .into()
                }
                None => row.into(),
            }
        };

        let mut left_body: Vec<Element<'a, Message>> = Vec::new();
        let mut mid_body: Vec<Element<'a, Message>> = Vec::new();
        let mut right_body: Vec<Element<'a, Message>> = Vec::new();

        if visible_rows.is_empty() {
            let empty = container(text(self.props.empty_text.clone()).color(empty_text_color))
                .width(Length::Fill)
                .padding(Padding::from([20u16, 12u16]))
                .center_x(Length::Fill);
            mid_body.push(empty.into());
        } else {
            for (i, row_data) in visible_rows.iter().enumerate() {
                let actual_idx = vstart + i;
                let is_selected = self.selected_row_index == Some(actual_idx);
                let is_stripe = self.props.stripe && (actual_idx % 2 == 1);
                let bg = if is_selected {
                    selected_bg
                } else if is_stripe {
                    stripe_bg
                } else {
                    Color::TRANSPARENT
                };
                left_body.push(seg_row(&left_idx, row_data, bg, actual_idx));
                mid_body.push(seg_row(&mid_idx, row_data, bg, actual_idx));
                right_body.push(seg_row(&right_idx, row_data, bg, actual_idx));
            }
        }

        // —— 三段组装：冻结段固定宽，中段 Fill + 横向 scrollable（头体同段） ——
        let left_col = iced::widget::Column::with_children(vec![
            iced::widget::Row::with_children(
                left_idx
                    .iter()
                    .map(|&i| header_cell(i))
                    .collect::<Vec<Element<'a, Message>>>(),
            )
            .into(),
            iced::widget::Column::with_children(left_body).into(),
        ]);
        let right_col = iced::widget::Column::with_children(vec![
            iced::widget::Row::with_children(
                right_idx
                    .iter()
                    .map(|&i| header_cell(i))
                    .collect::<Vec<Element<'a, Message>>>(),
            )
            .into(),
            iced::widget::Column::with_children(right_body).into(),
        ]);
        let mid_col = iced::widget::Column::with_children(vec![
            iced::widget::Row::with_children(
                mid_idx
                    .iter()
                    .map(|&i| header_cell(i))
                    .collect::<Vec<Element<'a, Message>>>(),
            )
            .into(),
            iced::widget::Column::with_children(mid_body).into(),
        ]);

        let mid_scroll: Element<'a, Message> = match table_msg {
            Some(rc) => {
                let rc = Rc::clone(rc);
                scrollable(mid_col)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .on_scroll(move |vp| rc(TableMessage::ScrollX(vp.absolute_offset().x)))
                    .into()
            }
            None => scrollable(mid_col)
                .width(Length::Fill)
                .height(Length::Fill)
                .into(),
        };

        let root = iced::widget::Row::with_children(vec![
            container(left_col).width(Length::Fixed(left_w)).into(),
            mid_scroll,
            container(right_col).width(Length::Fixed(right_w)).into(),
        ]);
        Element::from(root)
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
        assert!(vs.should_enable(51)); // 超过阈值启用
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
        let table: Table<String> =
            Table::new().with_rows(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
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

    // ============ 冻结列 / 列宽拖拽测试（ADR-008） ============

    fn frozen_columns() -> Vec<TableColumn> {
        vec![
            TableColumn::new("id", "ID")
                .with_width(60.0)
                .with_fixed(FixedSide::Left),
            TableColumn::new("name", "Name").with_width(200.0),
            TableColumn::new("price", "Price")
                .with_width(120.0)
                .with_resize_bounds(80.0, 300.0),
            TableColumn::new("qty", "Qty").with_width(80.0),
            TableColumn::new("op", "Op")
                .with_width(100.0)
                .with_fixed(FixedSide::Right),
        ]
    }

    #[test]
    fn test_fixed_partition_keeps_order() {
        let table: Table<String> = Table::new().with_columns(frozen_columns());
        let (left, mid, right) = table.fixed_partition();
        // 左冻结: id(0)；中段: name(1) price(2) qty(3)；右冻结: op(4)
        assert_eq!(left, vec![0]);
        assert_eq!(mid, vec![1, 2, 3]);
        assert_eq!(right, vec![4]);
        // 三段并集恰好覆盖全部列且保持原相对顺序
        let mut all: Vec<usize> = left;
        all.extend(mid);
        all.extend(right);
        all.sort_unstable();
        assert_eq!(all, (0..5).collect::<Vec<_>>());
    }

    #[test]
    fn test_is_frozen_layout() {
        let plain: Table<String> = Table::new().with_columns(vec![TableColumn::new("a", "A")]);
        assert!(!plain.is_frozen_layout());
        // 设置横向视窗即启用
        let with_vp: Table<String> = plain
            .clone()
            .with_columns(vec![TableColumn::new("a", "A")])
            .with_horizontal_viewport(600.0);
        assert!(with_vp.is_frozen_layout());
        // 带 fixed 列即启用
        let frozen: Table<String> =
            Table::new().with_columns(vec![TableColumn::new("a", "A").with_fixed(FixedSide::Left)]);
        assert!(frozen.is_frozen_layout());
    }

    #[test]
    fn test_resolved_width_override_wins_over_default() {
        let mut table: Table<String> = Table::new().with_columns(frozen_columns());
        // 未 resize：返回列默认宽
        assert_eq!(table.resolved_width(2), Some(120.0));
        // 程序化 resize +30 → 150（覆盖默认）
        table.handle(TableMessage::ResizeColumn(2, 30.0));
        assert_eq!(table.resolved_width(2), Some(150.0));
        // 其他列不受影响
        assert_eq!(table.resolved_width(1), Some(200.0));
        // 越界索引返回 None
        assert_eq!(table.resolved_width(99), None);
    }

    #[test]
    fn test_resize_column_clamped_by_bounds() {
        let mut table: Table<String> = Table::new().with_columns(frozen_columns());
        // price 列边界 [80, 300]，当前 120
        table.handle(TableMessage::ResizeColumn(2, 500.0));
        assert_eq!(table.resolved_width(2), Some(300.0)); // 上界钳制
        table.handle(TableMessage::ResizeColumn(2, -1000.0));
        assert_eq!(table.resolved_width(2), Some(80.0)); // 下界钳制
    }

    #[test]
    fn test_resize_column_default_bounds_without_explicit() {
        // name 列无 resizable → 程序化调整兜底 [40, +∞)
        let mut table: Table<String> = Table::new().with_columns(frozen_columns());
        table.handle(TableMessage::ResizeColumn(1, -500.0));
        assert_eq!(table.resolved_width(1), Some(40.0));
        // ResizeStart 对不可拖拽列不开启会话，ResizeMove 不生效
        table.handle(TableMessage::ResizeStart(1));
        table.handle(TableMessage::ResizeMove(500.0));
        assert_eq!(table.resolved_width(1), Some(40.0));
    }

    #[test]
    fn test_resize_drag_session_incremental() {
        let mut table: Table<String> = Table::new().with_columns(frozen_columns());
        // price=120, 边界 [80, 300]
        table.handle(TableMessage::ResizeStart(2)); // on_press
        table.handle(TableMessage::ResizeMove(100.0)); // 首次 move：锚定，不变宽
        assert_eq!(table.resolved_width(2), Some(120.0));
        table.handle(TableMessage::ResizeMove(140.0)); // +40 → 160
        assert_eq!(table.resolved_width(2), Some(160.0));
        table.handle(TableMessage::ResizeMove(120.0)); // -20 → 140
        assert_eq!(table.resolved_width(2), Some(140.0));
        table.handle(TableMessage::ResizeEnd); // 结束会话
        table.handle(TableMessage::ResizeMove(999.0)); // 会话外 move 不生效
        assert_eq!(table.resolved_width(2), Some(140.0));
    }

    #[test]
    fn test_scroll_x_clamped_by_frozen_widths() {
        // 视窗 400，冻结总宽 = 60(左id) + 100(右op) = 160 → 中段可用 240
        // 中段显式宽度 = 200+120+80 = 400 → max_scroll_x = 400-240 = 160
        let mut table: Table<String> = Table::new()
            .with_columns(frozen_columns())
            .with_horizontal_viewport(400.0);
        table.handle(TableMessage::ScrollX(99999.0));
        assert_eq!(table.scroll_x(), 160.0);
        // 负值钳为 0
        table.handle(TableMessage::ScrollX(-50.0));
        assert_eq!(table.scroll_x(), 0.0);
    }

    #[test]
    fn test_frozen_total_width_and_mid_content() {
        let table: Table<String> = Table::new()
            .with_columns(frozen_columns())
            .with_horizontal_viewport(400.0);
        // 冻结总宽 = 60 + 100 = 160
        assert_eq!(table.frozen_total_width(), 160.0);
        // 中段显式宽度 = 200 + 120 + 80 = 400
        assert_eq!(table.mid_content_width(), 400.0);
        // 中段可用 = 400 - 160 = 240
        assert_eq!(table.mid_available_width(), 240.0);
    }

    #[test]
    fn test_with_columns_resets_runtime_widths() {
        let mut table: Table<String> = Table::new().with_columns(frozen_columns());
        table.handle(TableMessage::ResizeColumn(2, 50.0));
        assert_eq!(table.resolved_width(2), Some(170.0));
        // 重新设置列 → 运行时覆盖清零，与列数对齐
        table = table.with_columns(frozen_columns());
        assert_eq!(table.resolved_width(2), Some(120.0));
    }
}
