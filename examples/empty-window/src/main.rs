//! HarUI 空窗口示例 — M0 里程碑验证
//!
//! 展示 HarUI 主题系统加载和 iced 窗口初始化。
//! 验证：
//! 1. har-ui-core 可被外部 crate 引用
//! 2. Theme::element_light() / element_dark() 可正常构造
//! 3. iced 窗口可创建并显示主题色

use har_ui_core::Theme;
use iced::widget::{button, column, container, text};
use iced::{Color, Element, Length, Task};

pub struct State {
    theme: Option<Theme>,
}

#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum Message {
    ToggleTheme,
    ThemeLoaded(Theme),
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::ToggleTheme => {
            if let Some(current) = &state.theme {
                let new_theme = if current.is_dark {
                    Theme::element_light()
                } else {
                    Theme::element_dark()
                };
                state.theme = Some(new_theme);
            }
            Task::none()
        }
        Message::ThemeLoaded(theme) => {
            state.theme = Some(theme);
            Task::none()
        }
    }
}

fn view(state: &State) -> Element<'_, Message> {
    let theme = state.theme.as_ref();

    let title_text = text("HarUI — Empty Window").size(28);
    let theme_name = text(format!(
        "Theme: {}",
        theme.map(|t| t.name.as_str()).unwrap_or("Loading...")
    ))
    .size(18);
    let theme_kind = text(format!(
        "Is Dark: {}",
        theme.map(|t| t.is_dark).unwrap_or(false)
    ))
    .size(16);

    let primary_color_text = text(format!(
        "Primary: {}",
        theme
            .map(|t| format!(
                "#{:02X}{:02X}{:02X}",
                t.primary.base.r, t.primary.base.g, t.primary.base.b
            ))
            .unwrap_or_else(|| "—".to_string())
    ))
    .size(16);

    let toggle_btn = button("Toggle Theme")
        .on_press(Message::ToggleTheme)
        .padding(10);

    let content = column![
        title_text,
        theme_name,
        theme_kind,
        primary_color_text,
        toggle_btn,
    ]
    .spacing(16)
    .padding(40);

    // 用 primary 色作为背景（如果有主题）
    let bg_color = theme
        .map(|t| Color::from(t.primary.base))
        .unwrap_or(Color::WHITE);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(move |_theme| container::Style {
            background: Some(iced::Background::Color(bg_color)),
            text_color: Some(Color::WHITE),
            ..container::Style::default()
        })
        .into()
}

fn main() -> iced::Result {
    iced::application("HarUI — Empty Window (M0 Verification)", update, view)
        .window_size(iced::Size::new(640.0, 480.0))
        .run_with(|| {
            let state = State { theme: None };
            (
                state,
                Task::perform(async { Theme::element_light() }, Message::ThemeLoaded),
            )
        })
}
