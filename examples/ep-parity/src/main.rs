//! EP 复刻对照页 — 1:1 复刻 element-plus.org Button demo（像素对照基准，W9 视觉管线）
//!
//! 布局规格对齐 EP 官网「基础用法」demo：
//! - 三行按钮：solid / plain / round，每行 6 个（Default/Primary/Success/Info/Warning/Danger）
//! - 水平间距 12px、行距 12px、页面白底、左上 20px 边距
//! - 按钮：Default 尺寸（padding 8×15，EP 同款），文字 14px
//!
//! 截图工作流：
//! 1. EP 参照帧：agent-browser 截 element-plus.org/zh-CN/component/button.html，裁 demo 卡片区
//! 2. 本页帧：trunk build 后 agent-browser 截整页（本页无多余 chrome）
//! 3. python scripts/visual_compare.py <ep-crop> <harui-crop>

use har_ui_components::button::{Button, ButtonType};
use har_ui_core::Theme;
use iced::widget::{column, container, text};
use iced::{Color, Element, Length, Padding, Task};

/// EP demo 行数据：(label, type)
const ROW_TYPES: [(&str, ButtonType); 6] = [
    ("Default", ButtonType::Default),
    ("Primary", ButtonType::Primary),
    ("Success", ButtonType::Success),
    ("Info", ButtonType::Info),
    ("Warning", ButtonType::Warning),
    ("Danger", ButtonType::Danger),
];

struct State {
    theme: Theme,
    solid: Vec<Button>,
    plain: Vec<Button>,
    round: Vec<Button>,
}

#[derive(Debug, Clone)]
enum Message {
    Clicked,
}

fn update(_state: &mut State, _msg: Message) -> Task<Message> {
    Task::none()
}

/// 一行按钮：从 State 中已持有的组件渲染（Element 借用 State 存活期）
fn button_row<'a>(state: &'a State, buttons: &'a [Button]) -> Element<'a, Message> {
    let mut row = iced::widget::Row::new().spacing(12.0);
    for btn in buttons {
        row = row.push(btn.view(&state.theme, Message::Clicked));
    }
    row.into()
}

fn view(state: &State) -> Element<'_, Message> {
    // EP 按钮 14px 文字说明（iced 默认 16px，差异记录在案）
    let note = text("Button demo - HarUI replica (EP spec aligned)")
        .size(14)
        .color(Color::from(state.theme.neutral.text_regular));

    let content = column![
        note,
        button_row(state, &state.solid), // solid
        button_row(state, &state.plain), // plain
        button_row(state, &state.round), // round
    ]
    .spacing(12.0)
    .padding(Padding::from(20u16));

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_t| iced::widget::container::Style {
            text_color: None,
            background: Some(iced::Background::Color(Color::WHITE)),
            border: iced::Border::default(),
            shadow: iced::Shadow::default(),
            snap: false,
        })
        .into()
}

fn run_app() -> iced::Result {
    iced::application(
        || {
            let mk = |plain: bool, round: bool| -> Vec<Button> {
                ROW_TYPES
                    .iter()
                    .map(|(label, btype)| {
                        Button::new(*label)
                            .with_type(*btype)
                            .plain(plain)
                            .round(round)
                    })
                    .collect()
            };
            (
                State {
                    theme: Theme::element_light(),
                    solid: mk(false, false),
                    plain: mk(true, false),
                    round: mk(false, true),
                },
                Task::none(),
            )
        },
        update,
        view,
    )
    .title(|_state: &State| String::from("EP Parity — Button"))
    .run()
}

/// 原生入口
#[cfg(not(target_arch = "wasm32"))]
fn main() -> iced::Result {
    run_app()
}

/// Web 入口（WASM 对照管线）
#[cfg(target_arch = "wasm32")]
fn main() -> Result<(), wasm_bindgen::JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    run_app().expect("ep-parity failed to run");
    Ok(())
}
