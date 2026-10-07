//! HarUI Table 示例 — 虚拟滚动 + 冻结列 + 列宽拖拽（ADR-008 / W2）
//!
//! - 垂直虚拟滚动：Scroll Up/Down 按钮驱动 `TableMessage::Scroll`（消息驱动模式）
//! - 冻结列：ID 列固定在左、操作列固定在右，中段横向滚动时保持原位
//! - 列宽拖拽：Name / Value / Qty 列表头右缘有拖拽条（`view_msg` 交互接线）
//! - 行点击：`RowClicked` → 选中行展示

use std::time::Instant;

use har_ui_components::table::{
    FixedSide, Table, TableColumn, TableMessage, TableProps, TableRow, VirtualScroll,
};
use har_ui_core::Theme;
use har_ui_core::devtools::fps_meter::FpsMeter;
use iced::widget::{Row, column, container, text};
use iced::{Element, Length, Task};

const ROW_HEIGHT: f32 = 30.0;
const VIEWPORT_HEIGHT: f32 = 300.0;
/// 横向视窗 = 800 窗口 - 左右 padding 40×2 - 余量
const H_VIEWPORT_WIDTH: f32 = 680.0;

pub struct State {
    theme: Theme,
    table: Table<TableRow>,
    clicked_row: Option<String>,
    /// FPS 测量（window::frames 订阅驱动，路线图 W5）
    fps: FpsMeter,
    show_fps: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    ScrollUp,
    ScrollDown,
    TableMsg(TableMessage),
    ToggleFps,
    /// 逐帧采样（window::frames 订阅）
    Frame(Instant),
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::ScrollUp => {
            if let Some(vs) = state.table.virtual_scroll().cloned() {
                let new_offset = (vs.scroll_offset - VIEWPORT_HEIGHT).max(0.0);
                state.table.handle(TableMessage::Scroll(new_offset));
            }
            Task::none()
        }
        Message::ScrollDown => {
            if let Some(vs) = state.table.virtual_scroll().cloned() {
                let new_offset = vs.scroll_offset + VIEWPORT_HEIGHT;
                state.table.handle(TableMessage::Scroll(new_offset));
            }
            Task::none()
        }
        Message::ToggleFps => {
            state.show_fps = !state.show_fps;
            state.fps.reset();
            Task::none()
        }
        Message::Frame(now) => {
            state.fps.record(now);
            Task::none()
        }
        Message::TableMsg(msg) => {
            if let TableMessage::RowClicked(idx) = &msg {
                state.clicked_row = state
                    .table
                    .rows()
                    .get(*idx)
                    .and_then(|r| r.get("name").cloned());
            }
            state.table.handle(msg);
            Task::none()
        }
    }
}

fn view(state: &State) -> Element<'_, Message> {
    let theme = &state.theme;
    let (vstart, vend) = state.table.visible_range();
    let total = state.table.rows().len();
    let sel = state.clicked_row.clone().unwrap_or_else(|| "-".to_string());

    let info = text(format!(
        "Total: {} | Visible: [{}, {}) | Selected: {} | ScrollX: {:.0}（拖拽表头分隔条可调列宽）",
        total,
        vstart,
        vend,
        sel,
        state.table.scroll_x()
    ))
    .size(14);

    let scroll_up = iced::widget::button("Scroll Up").on_press(Message::ScrollUp);
    let scroll_down = iced::widget::button("Scroll Down").on_press(Message::ScrollDown);
    let fps_label = if state.show_fps { "FPS ✓" } else { "FPS" };
    let fps_btn = iced::widget::button(fps_label)
        .on_press(Message::ToggleFps)
        .padding(iced::Padding::from([4u16, 12u16]));
    let controls = Row::new()
        .push(scroll_up)
        .push(scroll_down)
        .push(fps_btn)
        .spacing(8);

    // 交互模式：横向滚动 / 列宽拖拽 / 行点击统一经 Message::TableMsg 接线
    let table_elem = state.table.view_msg(
        theme,
        |row, prop| row.get(prop).cloned().unwrap_or_default(),
        Message::TableMsg,
    );

    let content = column![
        text("HarUI — Table Demo").size(24),
        info,
        controls,
        container(table_elem).height(Length::Fixed(VIEWPORT_HEIGHT)),
    ]
    .spacing(16)
    .padding(40);

    let layout = container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill);
    // FPS overlay 叠放（右上角）
    let fps_layer: Option<Element<'_, Message>> = if state.show_fps {
        state.fps.overlay_view(&state.theme)
    } else {
        None
    };
    match fps_layer {
        Some(overlay) => {
            let layer = container(overlay)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(iced::alignment::Horizontal::Right)
                .align_y(iced::alignment::Vertical::Top)
                .padding(iced::Padding::from([12u16, 12u16]));
            iced::widget::Stack::with_children(vec![layout.into(), layer.into()]).into()
        }
        None => layout.into(),
    }
}

fn main() -> iced::Result {
    iced::application("HarUI — Table Demo", update, view)
        .window_size(iced::Size::new(800.0, 600.0))
        .subscription(|_state| iced::window::frames().map(Message::Frame))
        .run_with(|| {
            let columns = vec![
                TableColumn::new("id", "ID")
                    .with_width(60.0)
                    .with_fixed(FixedSide::Left),
                TableColumn::new("name", "Name")
                    .with_width(200.0)
                    .with_resize_bounds(100.0, 400.0),
                TableColumn::new("value", "Value")
                    .with_width(120.0)
                    .with_resize_bounds(80.0, 300.0),
                TableColumn::new("qty", "Qty")
                    .with_width(80.0)
                    .with_resize_bounds(50.0, 200.0),
                TableColumn::new("op", "Op")
                    .with_width(100.0)
                    .with_fixed(FixedSide::Right),
            ];

            let rows: Vec<TableRow> = (0..1000)
                .map(|i| {
                    let mut row = TableRow::new();
                    row.insert("id".to_string(), i.to_string());
                    row.insert("name".to_string(), format!("Item {}", i));
                    row.insert("value".to_string(), format!("{}", i * 10));
                    row.insert("qty".to_string(), format!("{}", i % 100));
                    row.insert("op".to_string(), "详情".to_string());
                    row
                })
                .collect();

            let table = Table::new()
                .with_columns(columns)
                .with_rows(rows)
                .with_props(TableProps::new().with_stripe(true).with_border(true))
                .with_virtual_scroll(VirtualScroll::new(ROW_HEIGHT, VIEWPORT_HEIGHT))
                .with_horizontal_viewport(H_VIEWPORT_WIDTH);

            let state = State {
                theme: Theme::element_light(),
                table,
                clicked_row: None,
                fps: FpsMeter::new(),
                show_fps: false,
            };
            (state, Task::none())
        })
}
