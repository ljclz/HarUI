//! HarUI Input 示例

use har_ui_components::input::{Input, InputMessage, InputMode};
use har_ui_core::Theme;
use iced::widget::{column, container, text};
use iced::{Element, Length, Task};

pub struct State {
    theme: Theme,
    inputs: Vec<Input>,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(usize, String),
}

fn sync_value(input: &mut Input, new_value: &str) {
    input.handle(InputMessage::Clear);
    for ch in new_value.chars() {
        input.handle(InputMessage::Char(ch));
    }
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::InputChanged(idx, val) => {
            if idx < state.inputs.len() {
                sync_value(&mut state.inputs[idx], &val);
            }
            Task::none()
        }
    }
}

fn view(state: &State) -> Element<'_, Message> {
    let theme = &state.theme;

    let mut children: Vec<Element<'_, Message>> = Vec::new();
    children.push(text("HarUI — Input Demo").size(24).into());

    for (idx, input) in state.inputs.iter().enumerate() {
        let label = match input.mode() {
            InputMode::Text => "Text mode (clearable)",
            InputMode::Password => "Password mode",
            InputMode::Digit => "Digit mode (maxlength=6)",
            InputMode::Price => "Price mode (prefix + suffix)",
        };
        let input_elem = input.view(theme, move |s| Message::InputChanged(idx, s));
        let value_text = text(format!("Value: {}", input.value())).size(14);
        let col = column![text(label).size(16), input_elem, value_text].spacing(8);
        children.push(col.into());
    }

    let content = iced::widget::Column::with_children(children)
        .spacing(16)
        .padding(40);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .into()
}

fn main() -> iced::Result {
    iced::application("HarUI — Input Demo", update, view)
        .window_size(iced::Size::new(800.0, 700.0))
        .run_with(|| {
            let inputs = vec![
                Input::new().with_placeholder("Enter text").with_clearable(true),
                Input::new()
                    .with_mode(InputMode::Password)
                    .with_placeholder("Password"),
                Input::new()
                    .with_mode(InputMode::Digit)
                    .with_placeholder("Digits only")
                    .with_maxlength(6),
                Input::new()
                    .with_mode(InputMode::Price)
                    .with_placeholder("0.00")
                    .with_prefix("$")
                    .with_suffix("USD"),
            ];
            let state = State {
                theme: Theme::element_light(),
                inputs,
            };
            (state, Task::none())
        })
}
