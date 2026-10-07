//! HarUI Dialog 示例

use har_ui_components::dialog::{Dialog, DialogMessage};
use har_ui_core::Theme;
use iced::widget::{Row, button, column, container, text};
use iced::{Color, Element, Length, Padding, Task};

pub struct State {
    theme: Theme,
    dialog: Dialog,
    last_action: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    OpenDialog,
    Confirm,
    Cancel,
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::OpenDialog => {
            state.dialog.handle(DialogMessage::Open);
            Task::none()
        }
        Message::Confirm => {
            state.last_action = "Confirmed".to_string();
            state.dialog.handle(DialogMessage::Close);
            Task::none()
        }
        Message::Cancel => {
            state.last_action = "Cancelled".to_string();
            state.dialog.handle(DialogMessage::Close);
            Task::none()
        }
    }
}

fn view(state: &State) -> Element<'_, Message> {
    let theme = &state.theme;

    let trigger = button("Open Dialog")
        .on_press(Message::OpenDialog)
        .padding(Padding::from([8u16, 16u16]));
    let status = text(format!("Last action: {}", state.last_action)).size(16);

    let main_content = column![text("HarUI — Dialog Demo").size(24), trigger, status]
        .spacing(16)
        .padding(40);

    if !state.dialog.is_visible() {
        return container(main_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .into();
    }

    let title_text = text(state.dialog.title().to_string())
        .color(Color::from(theme.neutral.text_primary))
        .size(16);
    let close_btn = button(text("×").color(Color::from(theme.neutral.text_regular)))
        .padding(Padding::from([2u16, 8u16]))
        .on_press(Message::Cancel);
    let header = Row::new()
        .push(title_text)
        .push(iced::widget::Space::new().width(Length::Fill))
        .push(close_btn)
        .align_y(iced::Alignment::Center)
        .padding(Padding::from([12u16, 16u16]));

    let body_text =
        text(state.dialog.content().to_string()).color(Color::from(theme.neutral.text_regular));
    let body = container(body_text)
        .width(Length::Fill)
        .padding(Padding::from(16u16));

    let confirm_btn = button("Confirm")
        .on_press(Message::Confirm)
        .padding(Padding::from([6u16, 16u16]));
    let cancel_btn = button("Cancel")
        .on_press(Message::Cancel)
        .padding(Padding::from([6u16, 16u16]));
    let footer = Row::new()
        .push(iced::widget::Space::new().width(Length::Fill))
        .push(cancel_btn)
        .push(confirm_btn)
        .spacing(8)
        .padding(Padding::from([12u16, 16u16]));

    let dialog_box = container(
        iced::widget::Column::new()
            .push(header)
            .push(body)
            .push(footer),
    )
    .max_width(500.0)
    .style(move |_t| iced::widget::container::Style {
        text_color: Some(Color::from(theme.neutral.text_primary)),
        background: Some(iced::Background::Color(Color::from(
            theme.neutral.bg_overlay,
        ))),
        border: iced::Border {
            color: Color::from(theme.neutral.border_lighter),
            width: 1.0,
            radius: iced::border::radius(4.0),
        },
        shadow: iced::Shadow {
            color: Color {
                a: 0.3,
                ..Color::BLACK
            },
            offset: iced::Vector::new(0.0, 4.0),
            blur_radius: 16.0,
        },
        snap: false,
    });

    let overlay = container(
        container(dialog_box)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(move |_t| iced::widget::container::Style {
        text_color: None,
        background: Some(iced::Background::Color(Color {
            a: 0.5,
            ..Color::BLACK
        })),
        border: iced::Border::default(),
        shadow: iced::Shadow::default(),
        snap: false,
    });

    overlay.into()
}

fn main() -> iced::Result {
    iced::application(
        || {
            let state = State {
                theme: Theme::element_light(),
                dialog: Dialog::new("Confirm Action", "Are you sure you want to proceed?"),
                last_action: "None".to_string(),
            };
            (state, Task::none())
        },
        update,
        view,
    )
    .title(|_state: &State| String::from("HarUI — Dialog Demo"))
    .window_size(iced::Size::new(800.0, 600.0))
    .run()
}
