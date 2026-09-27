//! HarUI Tabs 示例

use har_ui_components::tabs::{TabItem, Tabs, TabsMessage};
use har_ui_core::Theme;
use iced::widget::{column, container, text};
use iced::{Element, Length, Task};

pub struct State {
    theme: Theme,
    tabs: Tabs,
}

#[derive(Debug, Clone)]
pub enum Message {
    TabSelected(String),
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::TabSelected(id) => {
            state.tabs.handle(TabsMessage::Select(id));
            Task::none()
        }
    }
}

fn view(state: &State) -> Element<'_, Message> {
    let active_label = state
        .tabs
        .active()
        .and_then(|id| {
            state
                .tabs
                .items()
                .iter()
                .find(|i| &i.id == id)
                .map(|i| i.label.as_str())
        })
        .unwrap_or("（无）");

    let status = text(format!("当前激活: {}", active_label)).size(14);

    let tabs_view = state.tabs.view(&state.theme, Message::TabSelected);

    let content = column![text("HarUI — Tabs Demo").size(24), status, tabs_view,]
        .spacing(16)
        .padding(40);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .into()
}

fn main() -> iced::Result {
    iced::application("HarUI — Tabs Demo", update, view)
        .window_size(iced::Size::new(800.0, 600.0))
        .run_with(|| {
            let tabs = Tabs::new()
                .with_item(TabItem::new("user", "用户"))
                .with_item(TabItem::new("role", "角色"))
                .with_item(TabItem::new("perm", "权限"))
                .with_item(TabItem::new("log", "日志"));
            let state = State {
                theme: Theme::element_light(),
                tabs,
            };
            (state, Task::none())
        })
}
