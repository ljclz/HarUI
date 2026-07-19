//! HarUI Table 示例

use har_ui_components::table::{
    TableColumn, TableMessage, TableProps, Table, TableRow, VirtualScroll,
};
use har_ui_core::Theme;
use iced::widget::{column, container, text, Row};
use iced::{Element, Length, Task};

const ROW_HEIGHT: f32 = 30.0;
const VIEWPORT_HEIGHT: f32 = 300.0;

pub struct State {
    theme: Theme,
    table: Table<TableRow>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ScrollUp,
    ScrollDown,
    TableMsg(TableMessage),
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
        Message::TableMsg(msg) => {
            state.table.handle(msg);
            Task::none()
        }
    }
}

fn view(state: &State) -> Element<'_, Message> {
    let theme = &state.theme;
    let (vstart, vend) = state.table.visible_range();
    let total = state.table.rows().len();

    let info = text(format!(
        "Total rows: {} | Visible: [{}, {}) | Use Scroll Up/Down buttons to change view",
        total, vstart, vend
    ))
    .size(14);

    let scroll_up = iced::widget::button("Scroll Up").on_press(Message::ScrollUp);
    let scroll_down = iced::widget::button("Scroll Down").on_press(Message::ScrollDown);
    let controls = Row::new().push(scroll_up).push(scroll_down).spacing(8);

    let table_elem = state.table.view(
        theme,
        |row, prop| row.get(prop).cloned().unwrap_or_default(),
        |idx| Message::TableMsg(TableMessage::RowClicked(idx)),
    );

    let content = column![
        text("HarUI — Table Demo").size(24),
        info,
        controls,
        container(table_elem).height(Length::Fixed(VIEWPORT_HEIGHT)),
    ]
    .spacing(16)
    .padding(40);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .into()
}

fn main() -> iced::Result {
    iced::application("HarUI — Table Demo", update, view)
        .window_size(iced::Size::new(800.0, 600.0))
        .run_with(|| {
            let columns = vec![
                TableColumn::new("id", "ID").with_width(80.0),
                TableColumn::new("name", "Name").with_width(200.0),
                TableColumn::new("value", "Value").with_width(120.0),
            ];

            let rows: Vec<TableRow> = (0..1000)
                .map(|i| {
                    let mut row = TableRow::new();
                    row.insert("id".to_string(), i.to_string());
                    row.insert("name".to_string(), format!("Item {}", i));
                    row.insert("value".to_string(), format!("{}", i * 10));
                    row
                })
                .collect();

            let table = Table::new()
                .with_columns(columns)
                .with_rows(rows)
                .with_props(TableProps::new().with_stripe(true).with_border(true))
                .with_virtual_scroll(VirtualScroll::new(ROW_HEIGHT, VIEWPORT_HEIGHT));

            let state = State {
                theme: Theme::element_light(),
                table,
            };
            (state, Task::none())
        })
}
