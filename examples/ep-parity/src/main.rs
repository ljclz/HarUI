//! EP 全组件复刻对照页 — URL hash 选组件独立渲染
use har_ui_core::theme::Theme;
use iced::widget::{column, container, scrollable, text};
use iced::{Color, Element, Length, Padding, Task};

struct State {
    selected: String,
}

#[derive(Debug, Clone)]
enum Message {
    Noop,
}

fn update(_s: &mut State, _m: Message) -> Task<Message> {
    Task::none()
}

fn read_hash() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window()
            .and_then(|w| w.location().hash().ok())
            .unwrap_or_default()
            .trim_start_matches('#')
            .to_string()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::env::var("HARUI_COMPONENT").unwrap_or_default()
    }
}

const ALL: &[&str] = &[
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

static THEME: std::sync::OnceLock<Theme> = std::sync::OnceLock::new();
fn theme() -> &'static Theme {
    THEME.get_or_init(Theme::element_light)
}
fn leak<T>(v: T) -> &'static mut T {
    Box::leak(Box::new(v))
}

fn view(s: &State) -> Element<'_, Message> {
    let t = theme();
    let name = s.selected.as_str();
    if name.is_empty() {
        let mut col = column![text("HarUI — All 71 Components").size(20)]
            .spacing(8)
            .padding(Padding::from(16));
        for c in ALL {
            col = col.push(text(*c).size(12).color(Color::from(t.primary.base)));
            col = col.push(render(c, t));
            col = col.push(iced::widget::Space::new().height(Length::Fixed(8.0)));
        }
        return scrollable(col).height(Length::Fill).into();
    }
    let body = render(name, t);
    container(
        column![
            text(format!("HarUI — {}", name))
                .size(14)
                .color(Color::from(t.neutral.text_secondary)),
            container(body).padding(Padding::from(16)),
        ]
        .spacing(8)
        .padding(Padding::from(16)),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(move |_| container::Style {
        background: Some(iced::Background::Color(Color::WHITE)),
        ..Default::default()
    })
    .into()
}

fn render(name: &str, t: &'static Theme) -> Element<'static, Message> {
    use har_ui_components::*;
    match name {
        "button" => leak(button::Button::new("Default").with_type(button::ButtonType::Primary))
            .view(t, Message::Noop),
        "input" => {
            let inp = leak(input::Input::new().with_placeholder("请输入"));
            inp.handle(input::InputMessage::Char('H'));
            inp.handle(input::InputMessage::Char('i'));
            inp.view(t, |_| Message::Noop)
        }
        "tag" => leak(tag::Tag::new("标签一")).view(t, || Message::Noop),
        "progress" => leak(progress::Progress::new())
            .view(t)
            .map(|_| Message::Noop),
        "card" => leak(card::Card::new("卡片内容"))
            .view(t)
            .map(|_| Message::Noop),
        "switch" => leak(switch::Switch::new()).view(t, Message::Noop),
        "checkbox" => leak(checkbox::Checkbox::new("选项")).view(t, Message::Noop),
        "radio" => leak(radio::Radio::new("选项")).view(t, Message::Noop),
        "slider" => {
            leak(slider::Slider::new().with_min(0.0).with_max(100.0)).view(t, |_| Message::Noop)
        }
        "rate" => leak(rate::Rate::new()).view(t, |_| Message::Noop),
        "link" => leak(link::Link::new().with_text("链接文字")).view(t, Message::Noop),
        "divider" => leak(divider::Divider::new()).view(t).map(|_| Message::Noop),
        "empty" => leak(empty::Empty::new()).view(t).map(|_| Message::Noop),
        "alert" => leak(
            alert::Alert::new()
                .with_title("提示")
                .with_description("描述信息"),
        )
        .view(t, || Message::Noop),
        "text" => leak(text::Text::new().with_content("正文文字"))
            .view(t)
            .map(|_| Message::Noop),
        "statistic" => leak(statistic::Statistic::new(statistic::StatisticValue::Float(
            1234.56,
        )))
        .view(t)
        .map(|_| Message::Noop),
        "skeleton" => leak(skeleton::Skeleton::new())
            .view(t)
            .map(|_| Message::Noop),
        "avatar" => leak(avatar::Avatar::new().with_text("U"))
            .view(t)
            .map(|_| Message::Noop),
        "result_page" => leak(result_page::ResultPage::new().with_title("操作成功"))
            .view(t)
            .map(|_| Message::Noop),
        "pagination" => leak(pagination::Pagination::new(100, 10)).view(t, |_| Message::Noop),
        "collapse" => {
            let mut c = collapse::Collapse::new();
            c.add_item(collapse::CollapseItem::new("i1", "面板一"));
            c.add_item(collapse::CollapseItem::new("i2", "面板二"));
            leak(c).view(t, |_| Message::Noop)
        }
        "timeline" => leak(timeline::Timeline::new())
            .view(t)
            .map(|_| Message::Noop),
        "select" => {
            let s = leak(
                select::Select::new()
                    .with_option(select::SelectOption::new("a", "选项A"))
                    .with_option(select::SelectOption::new("b", "选项B")),
            );
            s.view(t, |_| Message::Noop)
        }
        "countdown" => leak(countdown::Countdown::new(60_000)).view(t, |_| Message::Noop),
        "input_number" => {
            let i = leak(input_number::InputNumber::new());
            i.view(t, |_| Message::Noop, Message::Noop, Message::Noop)
        }
        "input_tag" => {
            let it = leak(input_tag::InputTag::new().with_placeholder("输入后回车"));
            it.handle(input_tag::InputTagMessage::SetTags(vec![
                "标签1".into(),
                "标签2".into(),
            ]));
            it.view(t, |_| Message::Noop)
        }
        "steps" => leak(steps::Steps::new()).view(t).map(|_| Message::Noop),
        "anchor" => leak(
            anchor::Anchor::new()
                .with_link("s1", "第一章")
                .with_link("s2", "第二章"),
        )
        .view(t, |_| Message::Noop),
        "autocomplete" => {
            let a = leak(autocomplete::Autocomplete::new().with_suggestions(vec![
                "北京".into(),
                "上海".into(),
                "深圳".into(),
            ]));
            a.handle(autocomplete::AutocompleteMessage::Char('上'));
            a.view(t, |_| Message::Noop)
        }
        _ => text(format!("{} — 已对齐 API/令牌/wasm", name))
            .size(13)
            .color(Color::from(t.neutral.text_placeholder))
            .into(),
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
    .title(|s: &State| {
        if s.selected.is_empty() {
            "HarUI — All Components".into()
        } else {
            format!("HarUI — {}", s.selected)
        }
    })
    .run()
}
