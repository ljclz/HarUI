//! HarUI DatePicker 示例

use har_ui_components::date_picker::{DatePicker, DatePickerMessage, SimpleDate};
use har_ui_core::Theme;
use iced::widget::{column, container, text};
use iced::{Element, Length, Task};

pub struct State {
    theme: Theme,
    picker: DatePicker,
}

#[derive(Debug, Clone)]
pub enum Message {
    Pick(String),
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::Pick(s) => {
            let msg = match s.as_str() {
                "__toggle__" => {
                    if state.picker.visible() {
                        DatePickerMessage::Close
                    } else {
                        DatePickerMessage::Open
                    }
                }
                "__prev_month__" => DatePickerMessage::PrevMonth,
                "__next_month__" => DatePickerMessage::NextMonth,
                "__prev_year__" => DatePickerMessage::PrevYear,
                "__next_year__" => DatePickerMessage::NextYear,
                _ => {
                    let parts: Vec<&str> = s.split('-').collect();
                    if parts.len() != 3 {
                        return Task::none();
                    }
                    let year = match parts[0].parse::<i32>() {
                        Ok(v) => v,
                        Err(_) => return Task::none(),
                    };
                    let month = match parts[1].parse::<u32>() {
                        Ok(v) => v,
                        Err(_) => return Task::none(),
                    };
                    let day = match parts[2].parse::<u32>() {
                        Ok(v) => v,
                        Err(_) => return Task::none(),
                    };
                    DatePickerMessage::SelectDate(SimpleDate::new(year, month, day))
                }
            };
            state.picker.handle(msg);
            Task::none()
        }
    }
}

fn view(state: &State) -> Element<'_, Message> {
    let selected = state.picker.formatted_value();
    let display = if selected.is_empty() {
        "（未选择）".to_string()
    } else {
        format!("已选择: {}", selected)
    };
    let status = text(display).size(14);

    let picker_view = state.picker.view(&state.theme, |s| Message::Pick(s));

    let content = column![
        text("HarUI — DatePicker Demo").size(24),
        status,
        picker_view,
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
    iced::application("HarUI — DatePicker Demo", update, view)
        .window_size(iced::Size::new(800.0, 600.0))
        .run_with(|| {
            let state = State {
                theme: Theme::element_light(),
                picker: DatePicker::new(),
            };
            (state, Task::none())
        })
}
