//! HarUI Upload 示例

use har_ui_components::upload::{Upload, UploadFile, UploadMessage};
use har_ui_core::Theme;
use iced::widget::{column, container, text};
use iced::{Element, Length, Task};

pub struct State {
    theme: Theme,
    upload: Upload,
    trigger_count: u32,
}

#[derive(Debug, Clone)]
pub enum Message {
    Trigger,
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::Trigger => {
            state.trigger_count += 1;
            let file = UploadFile::new(
                format!("demo_file_{}.txt", state.trigger_count),
                1024 * (state.trigger_count as u64),
            );
            state.upload.handle(UploadMessage::AddFile(file));
            Task::none()
        }
    }
}

fn view(state: &State) -> Element<'_, Message> {
    let count = state.upload.file_list().len();
    let status = text(format!("已添加文件数: {}", count)).size(14);

    let upload_view = state.upload.view(&state.theme, || Message::Trigger);

    let content = column![text("HarUI — Upload Demo").size(24), status, upload_view,]
        .spacing(16)
        .padding(40);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .into()
}

fn main() -> iced::Result {
    iced::application("HarUI — Upload Demo", update, view)
        .window_size(iced::Size::new(800.0, 600.0))
        .run_with(|| {
            let upload = Upload::new().with_multiple(true).with_limit(5);
            let state = State {
                theme: Theme::element_light(),
                upload,
                trigger_count: 0,
            };
            (state, Task::none())
        })
}
