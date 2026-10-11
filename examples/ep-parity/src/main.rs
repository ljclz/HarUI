//! EP 全组件复刻对照页 — URL hash 选组件独立渲染（W9 像素对照管线）
//!
//! 用法：
//! - `http://…/#button` → 只渲染 Button
//! - `http://…/#input` → 只渲染 Input
//! - 无 hash（或 native `HARUI_COMPONENT=x`）→ 纵向渲染全部组件
//!
//! 每个组件在白底独立渲染，适配 agent-browser 截帧 + visual_compare.py 对比。

use har_ui_core::theme::Theme;
use iced::widget::{column, container, scrollable, text};
use iced::{Color, Element, Length, Padding, Task};

struct State {
    selected: String,
}

static THEME: std::sync::OnceLock<Theme> = std::sync::OnceLock::new();

fn get_theme() -> &'static Theme {
    THEME.get_or_init(Theme::element_light)
}

#[derive(Debug, Clone)]
enum Message {
    Noop,
}

fn update(_state: &mut State, _msg: Message) -> Task<Message> {
    Task::none()
}

/// 组件名列表（71 个，与 lib.rs pub mod 一致）
const ALL_COMPONENTS: &[&str] = &[
    "affix",
    "alert",
    "anchor",
    "autocomplete",
    "avatar",
    "backtop",
    "badge",
    "breadcrumb",
    "button",
    "calendar",
    "card",
    "carousel",
    "cascader",
    "checkbox",
    "collapse",
    "color_picker",
    "countdown",
    "customer_display",
    "date_picker",
    "descriptions",
    "dialog",
    "divider",
    "drawer",
    "dropdown",
    "empty",
    "form",
    "grid",
    "hang_order",
    "image",
    "input",
    "input_number",
    "input_tag",
    "keypad",
    "link",
    "loading",
    "menu",
    "message",
    "message_box",
    "notification",
    "page_header",
    "pagination",
    "payment",
    "popconfirm",
    "popover",
    "progress",
    "radio",
    "rate",
    "result_page",
    "scrollbar",
    "select",
    "skeleton",
    "slider",
    "space",
    "splitter",
    "statistic",
    "status_bar",
    "steps",
    "switch",
    "table",
    "tabs",
    "tag",
    "text",
    "time_picker",
    "timeline",
    "tooltip",
    "tour",
    "transfer",
    "tree",
    "tree_select",
    "upload",
    "watermark",
];

fn view(state: &State) -> Element<'_, Message> {
    let theme = get_theme();
    let name = state.selected.as_str();

    if name.is_empty() {
        let mut col = column![text("HarUI — All Components").size(20),]
            .spacing(8)
            .padding(Padding::from(16u16));
        for comp in ALL_COMPONENTS {
            col = col.push(text(*comp).size(12).color(Color::from(theme.primary.base)));
            col = col.push(render_component(comp, theme));
            col = col.push(iced::widget::Space::new().height(Length::Fixed(8.0)));
        }
        return scrollable(col).height(Length::Fill).into();
    }

    let body = render_component(name, theme);
    container(
        column![
            text(format!("HarUI — {}", name))
                .size(14)
                .color(Color::from(theme.neutral.text_secondary)),
            container(body).padding(Padding::from(16u16)),
        ]
        .spacing(8)
        .padding(Padding::from(16u16)),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(move |_t| iced::widget::container::Style {
        background: Some(iced::Background::Color(Color::WHITE)),
        ..container::Style::default()
    })
    .into()
}

/// 按组件名渲染默认态视图
fn render_component(name: &str, theme: &'static Theme) -> Element<'static, Message> {
    use har_ui_components::*;
    // Box::leak：组件实例泄漏获得 'static，Element 引用安全返回。
    // 本页仅在 boot 时渲染一次，泄漏量可控（71 个组件实例 ≈ 几十 KB）。
    fn leak<T>(v: T) -> &'static mut T {
        Box::leak(Box::new(v))
    }
    // 每个分支返回 Element<'static, Message>——无 Message 依赖的组件
    // view() 返回 Element<'_, ()>，通过 .map(|_| Message::Noop) 统一类型
    match name {
        "button" => leak(button::Button::new("Default").with_type(button::ButtonType::Primary))
            .view(theme, Message::Noop),
        "input" => {
            let inp = leak(input::Input::new().with_placeholder("请输入内容"));
            inp.handle(input::InputMessage::Char('H'));
            inp.handle(input::InputMessage::Char('i'));
            inp.view(theme, |_| Message::Noop)
        }
        "tag" => leak(tag::Tag::new("标签一")).view(theme, || Message::Noop),
        "progress" => leak(progress::Progress::new())
            .view(theme)
            .map(|_| Message::Noop),
        "card" => leak(card::Card::new("卡片内容"))
            .view(theme)
            .map(|_| Message::Noop),
        "switch" => leak(switch::Switch::new()).view(theme, Message::Noop),
        "checkbox" => leak(checkbox::Checkbox::new("选项")).view(theme, Message::Noop),
        "radio" => leak(radio::Radio::new("选项")).view(theme, Message::Noop),
        "slider" => {
            leak(slider::Slider::new().with_min(0.0).with_max(100.0)).view(theme, |_| Message::Noop)
        }
        "rate" => leak(rate::Rate::new()).view(theme, |_| Message::Noop),
        "link" => leak(link::Link::new().with_text("链接文字")).view(theme, Message::Noop),
        "divider" => leak(divider::Divider::new())
            .view(theme)
            .map(|_| Message::Noop),
        "empty" => leak(empty::Empty::new()).view(theme).map(|_| Message::Noop),
        "alert" => leak(
            alert::Alert::new()
                .with_title("提示")
                .with_description("这是一条提示信息"),
        )
        .view(theme, || Message::Noop),
        "badge" => leak(badge::Badge::new(badge::BadgeValue::Number(5)))
            .view(theme)
            .map(|_| Message::Noop),
        "text" => leak(text::Text::new().with_content("正文文字"))
            .view(theme)
            .map(|_| Message::Noop),
        "breadcrumb" => leak(breadcrumb::Breadcrumb::new()).view(theme, |_| Message::Noop),
        "statistic" => leak(statistic::Statistic::new(statistic::StatisticValue::Float(
            1234.56,
        )))
        .view(theme)
        .map(|_| Message::Noop),
        "skeleton" => leak(skeleton::Skeleton::new())
            .view(theme)
            .map(|_| Message::Noop),
        "space" => text("Space 布局组件（无独立视觉输出）")
            .size(13)
            .color(Color::from(theme.neutral.text_placeholder))
            .into(),
        "avatar" => leak(avatar::Avatar::new().with_text("U"))
            .view(theme)
            .map(|_| Message::Noop),
        "result_page" => leak(result_page::ResultPage::new().with_title("操作成功"))
            .view(theme)
            .map(|_| Message::Noop),
        "date_picker" => leak(date_picker::DatePicker::new()).view(theme, |_| Message::Noop),
        "pagination" => leak(pagination::Pagination::new(100, 10)).view(theme, |_| Message::Noop),
        "collapse" => {
            let mut c = collapse::Collapse::new();
            c.add_item(collapse::CollapseItem::new("item1", "面板一"));
            c.add_item(collapse::CollapseItem::new("item2", "面板二"));
            leak(c).view(theme, |_| Message::Noop)
        }
        "timeline" => leak(timeline::Timeline::new())
            .view(theme)
            .map(|_| Message::Noop),
        _ => text(format!("组件 {} 默认态渲染待接入", name))
            .size(13)
            .color(Color::from(theme.neutral.text_placeholder))
            .into(),
    }
}

/// 读取 URL hash / 环境变量
fn read_hash() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window()
            .and_then(|w| w.location().hash().ok())
            .map(|h| h.trim_start_matches('#').to_string())
            .unwrap_or_default()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::env::var("HARUI_COMPONENT").unwrap_or_default()
    }
}

fn main() -> iced::Result {
    iced::application(
        || {
            (
                State {
                    selected: read_hash(),
                },
                Task::none(),
            )
        },
        update,
        view,
    )
    .title(|state: &State| {
        if state.selected.is_empty() {
            String::from("HarUI — All Components")
        } else {
            format!("HarUI — {}", state.selected)
        }
    })
    .run()
}
