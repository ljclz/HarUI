//! HarUI Select 示例

use har_ui_components::select::{Select, SelectMessage, SelectOption};
use har_ui_core::Theme;
use iced::widget::{column, container, text};
use iced::{Element, Length, Task};

pub struct State {
    theme: Theme,
    select: Select,
}

#[derive(Debug, Clone)]
pub enum Message {
    Choose(String),
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::Choose(s) => {
            match s.as_str() {
                "__trigger__" => state.select.handle(SelectMessage::Open),
                "__clear__" => state.select.handle(SelectMessage::Clear),
                s if s.starts_with("__query:") => {
                    let q = s.strip_prefix("__query:").unwrap_or("");
                    state.select.handle(SelectMessage::Query(q.to_string()));
                }
                other => state
                    .select
                    .handle(SelectMessage::Choose(other.to_string())),
            }
            Task::none()
        }
    }
}

fn view(state: &State) -> Element<'_, Message> {
    let theme = &state.theme;
    let select_elem = state.select.view(theme, Message::Choose);

    let current = state
        .select
        .display_label()
        .unwrap_or_else(|| "(none)".to_string());

    let content = column![
        text("HarUI — Select Demo").size(24),
        select_elem,
        text(format!("Current: {}", current)).size(16),
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
    iced::application("HarUI — Select Demo", update, view)
        .window_size(iced::Size::new(600.0, 500.0))
        .run_with(|| {
            let select = Select::new()
                .with_clearable(true)
                .with_filterable(true)
                .with_option(SelectOption::new("beijing", "Beijing"))
                .with_option(SelectOption::new("shanghai", "Shanghai"))
                .with_option(SelectOption::new("guangzhou", "Guangzhou"))
                .with_option(SelectOption::new("shenzhen", "Shenzhen"))
                .with_option(SelectOption::new("hangzhou", "Hangzhou"))
                .with_option(SelectOption::new("chengdu", "Chengdu"))
                .with_option(SelectOption::new("nanjing", "Nanjing"))
                .with_option(SelectOption::new("xian", "Xi'an"));

            let state = State {
                theme: Theme::element_light(),
                select,
            };
            (state, Task::none())
        })
}
