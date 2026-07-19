//! HarUI Card 示例

use har_ui_components::card::{Card, CardMessage, CardShadow};
use har_ui_core::Theme;
use iced::widget::{button, column, container, text};
use iced::{Element, Length, Task};

pub struct State {
    theme: Theme,
    always_card: Card,
    hover_card: Card,
    never_card: Card,
    hover_on: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    ToggleHover,
    CardNoop,
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::ToggleHover => {
            state.hover_on = !state.hover_on;
            if state.hover_on {
                state.hover_card.handle(CardMessage::Hovered);
            } else {
                state.hover_card.handle(CardMessage::Unhovered);
            }
            Task::none()
        }
        Message::CardNoop => Task::none(),
    }
}

fn view(state: &State) -> Element<'_, Message> {
    let theme = &state.theme;

    let always_elem = state.always_card.view(theme).map(|_| Message::CardNoop);
    let hover_elem = state.hover_card.view(theme).map(|_| Message::CardNoop);
    let never_elem = state.never_card.view(theme).map(|_| Message::CardNoop);

    let toggle_label = if state.hover_on { "Unhover" } else { "Hover" };
    let toggle_btn = button(toggle_label).on_press(Message::ToggleHover);

    let content = column![
        text("HarUI — Card Demo").size(24),
        toggle_btn,
        always_elem,
        hover_elem,
        never_elem,
    ]
    .spacing(16)
    .padding(40)
    .max_width(500);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .into()
}

fn main() -> iced::Result {
    iced::application("HarUI — Card Demo", update, view)
        .window_size(iced::Size::new(700.0, 700.0))
        .run_with(|| {
            let always_card = Card::new("This card always shows shadow.")
                .with_header("Card — Shadow Always")
                .with_footer("Footer: always shadow");
            let hover_card = Card::new("Toggle hover state to see shadow change.")
                .with_header("Card — Shadow Hover")
                .with_footer("Footer: hover shadow")
                .with_shadow(CardShadow::Hover);
            let never_card = Card::new("This card never shows shadow.")
                .with_header("Card — Shadow Never")
                .with_footer("Footer: never shadow")
                .with_shadow(CardShadow::Never);

            let state = State {
                theme: Theme::element_light(),
                always_card,
                hover_card,
                never_card,
                hover_on: false,
            };
            (state, Task::none())
        })
}
