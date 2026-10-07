//! HarUI Pagination 示例

use har_ui_components::pagination::{Pagination, PaginationMessage};
use har_ui_core::Theme;
use iced::widget::{column, container, text};
use iced::{Element, Length, Task};

pub struct State {
    theme: Theme,
    pagination: Pagination,
}

#[derive(Debug, Clone)]
pub enum Message {
    PageJump(i64),
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::PageJump(p) => {
            match p {
                -1 => state.pagination.handle(PaginationMessage::Prev),
                -2 => state.pagination.handle(PaginationMessage::Next),
                other => state.pagination.handle(PaginationMessage::JumpTo(other)),
            }
            Task::none()
        }
    }
}

fn view(state: &State) -> Element<'_, Message> {
    let theme = &state.theme;
    let pagination_elem = state.pagination.view(theme, Message::PageJump);

    let info = text(format!(
        "Page {} of {}",
        state.pagination.current_page(),
        state.pagination.total_pages()
    ))
    .size(18);

    let content = column![
        text("HarUI — Pagination Demo").size(24),
        info,
        pagination_elem,
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
    iced::application(
        || {
            let state = State {
                theme: Theme::element_light(),
                pagination: Pagination::new(100, 10),
            };
            (state, Task::none())
        },
        update,
        view,
    )
    .title(|_state: &State| String::from("HarUI — Pagination Demo"))
    .window_size(iced::Size::new(700.0, 400.0))
    .run()
}
