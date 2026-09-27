//! HarUI Button 示例

use har_ui_components::button::{Button, ButtonType};
use har_ui_core::Theme;
use iced::widget::{Row, column, container, text};
use iced::{Element, Length, Task};

pub struct State {
    theme: Theme,
    type_buttons: Vec<Button>,
    plain_buttons: Vec<Button>,
    shape_buttons: Vec<Button>,
    click_count: u32,
}

#[derive(Debug, Clone)]
pub enum Message {
    Clicked,
}

fn update(state: &mut State, _message: Message) -> Task<Message> {
    state.click_count += 1;
    Task::none()
}

fn view(state: &State) -> Element<'_, Message> {
    let theme = &state.theme;

    let mut type_row = Row::new().spacing(8).align_y(iced::Alignment::Center);
    for btn in &state.type_buttons {
        type_row = type_row.push(btn.view(theme, Message::Clicked));
    }

    let mut plain_row = Row::new().spacing(8).align_y(iced::Alignment::Center);
    for btn in &state.plain_buttons {
        plain_row = plain_row.push(btn.view(theme, Message::Clicked));
    }

    let mut shape_row = Row::new().spacing(8).align_y(iced::Alignment::Center);
    for btn in &state.shape_buttons {
        shape_row = shape_row.push(btn.view(theme, Message::Clicked));
    }

    let content = column![
        text("HarUI — Button Demo").size(24),
        text(format!("Click count: {}", state.click_count)).size(18),
        type_row,
        plain_row,
        shape_row,
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
    iced::application("HarUI — Button Demo", update, view)
        .window_size(iced::Size::new(900.0, 500.0))
        .run_with(|| {
            let type_buttons = vec![
                Button::new("Default"),
                Button::new("Primary").with_type(ButtonType::Primary),
                Button::new("Success").with_type(ButtonType::Success),
                Button::new("Warning").with_type(ButtonType::Warning),
                Button::new("Danger").with_type(ButtonType::Danger),
                Button::new("Info").with_type(ButtonType::Info),
            ];
            let plain_buttons = vec![
                Button::new("Plain Primary")
                    .with_type(ButtonType::Primary)
                    .plain(true),
                Button::new("Plain Success")
                    .with_type(ButtonType::Success)
                    .plain(true),
                Button::new("Plain Danger")
                    .with_type(ButtonType::Danger)
                    .plain(true),
            ];
            let shape_buttons = vec![
                Button::new("Round")
                    .with_type(ButtonType::Primary)
                    .round(true),
                Button::new("R").with_type(ButtonType::Primary).circle(true),
                Button::new("Disabled")
                    .with_type(ButtonType::Primary)
                    .disabled(true),
            ];
            let state = State {
                theme: Theme::element_light(),
                type_buttons,
                plain_buttons,
                shape_buttons,
                click_count: 0,
            };
            (state, Task::none())
        })
}
