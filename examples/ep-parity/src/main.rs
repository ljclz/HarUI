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

/// Grid 泛型演示行 — 实现 `grid::GridItem`
#[derive(Debug, Clone)]
struct DemoGridItem {
    id: String,
    name: String,
}
impl har_ui_components::grid::GridItem for DemoGridItem {
    fn item_id(&self) -> String {
        self.id.clone()
    }
}

/// Table 泛型演示行 — 实现 `table::Identifiable`
#[derive(Debug, Clone)]
struct DemoRow {
    id: String,
    name: String,
    price: String,
}
impl har_ui_components::table::Identifiable for DemoRow {
    fn id(&self) -> String {
        self.id.clone()
    }
}
fn demo_row_field(r: &DemoRow, prop: &str) -> String {
    match prop {
        "name" => r.name.clone(),
        "price" => r.price.clone(),
        _ => String::new(),
    }
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
        "affix" => leak(affix::Affix::new()).view(t, text("固定内容").size(14).into()),
        "backtop" => leak(backtop::Backtop::new()).view(t, Message::Noop),
        "calendar" => leak(calendar::Calendar::new(2026, 10)).view(t, |_| Message::Noop),
        "carousel" => leak(carousel::Carousel::new()).view(t, || Message::Noop, || Message::Noop),
        "cascader" => {
            let cs = leak(cascader::Cascader::new().with_options(vec![
                    cascader::CascaderNode::new("zhejiang", "浙江").with_children(vec![
                        cascader::CascaderNode::new("hangzhou", "杭州"),
                        cascader::CascaderNode::new("ningbo", "宁波"),
                    ]),
                    cascader::CascaderNode::new("jiangsu", "江苏").with_children(vec![
                        cascader::CascaderNode::new("nanjing", "南京"),
                    ]),
                ]));
            cs.view(t, |_| Message::Noop)
        }
        "color_picker" => leak(color_picker::ColorPicker::new())
            .view(t)
            .map(|_| Message::Noop),
        "customer_display" => leak(customer_display::CustomerDisplay::new())
            .view(t)
            .map(|_| Message::Noop),
        "descriptions" => leak(
            descriptions::Descriptions::new()
                .with_title("用户信息")
                .with_item(descriptions::DescriptionsItem::new("姓名", "张三"))
                .with_item(descriptions::DescriptionsItem::new("手机", "138****8888")),
        )
        .view(t)
        .map(|_| Message::Noop),
        "drawer" => leak(drawer::Drawer::new()).view(t, || Message::Noop),
        "grid" => {
            let g = leak(
                grid::Grid::<DemoGridItem>::new()
                    .with_props(grid::GridProps::new().with_columns(3).with_gap(8.0))
                    .with_items(vec![
                        DemoGridItem {
                            id: "1".into(),
                            name: "商品一".into(),
                        },
                        DemoGridItem {
                            id: "2".into(),
                            name: "商品二".into(),
                        },
                        DemoGridItem {
                            id: "3".into(),
                            name: "商品三".into(),
                        },
                    ]),
            );
            let children: Vec<Element<'static, Message>> = g
                .items()
                .iter()
                .map(|i| {
                    container(text(i.name.clone()).size(13))
                        .width(Length::Fill)
                        .padding(Padding::from(8u16))
                        .style(move |_| container::Style {
                            background: Some(iced::Background::Color(Color::from(
                                t.neutral.bg_base,
                            ))),
                            border: iced::Border {
                                color: Color::from(t.neutral.border_lighter),
                                width: 1.0,
                                radius: iced::border::radius(4.0),
                            },
                            ..Default::default()
                        })
                        .into()
                })
                .collect();
            g.view(t, children)
        }
        "hang_order" => {
            leak(hang_order::HangOrder::new()).view(t, |_| Message::Noop, |_| Message::Noop)
        }
        "keypad" => leak(keypad::Keypad::new()).view(t, |_| Message::Noop),
        "page_header" => leak(page_header::PageHeader::new()).view(t, || Message::Noop),
        "popover" => leak(popover::Popover::new())
            .view(t, text("气泡内容").size(13).into(), || Message::Noop)
            .map(|_| Message::Noop),
        "scrollbar" => {
            let sb = leak(scrollbar::Scrollbar::new().with_height(100));
            let content = iced::widget::Column::with_children(
                (1..=10)
                    .map(|i| text(format!("滚动内容第 {} 行", i)).size(13).into())
                    .collect::<Vec<iced::Element<'static, Message>>>(),
            )
            .spacing(6)
            .padding(Padding::from(8u16));
            sb.view(t, content.into())
        }
        "status_bar" => leak(
            status_bar::StatusBar::new()
                .with_left_item(status_bar::StatusItem::new(
                    "称重",
                    status_bar::StatusLevel::Success,
                ))
                .with_left_item(status_bar::StatusItem::new(
                    "扫码",
                    status_bar::StatusLevel::Warning,
                ))
                .with_right_item(status_bar::StatusItem::new(
                    "网络",
                    status_bar::StatusLevel::Info,
                )),
        )
        .view(t)
        .map(|_| Message::Noop),
        "time_picker" => leak(time_picker::TimePicker::new()).view(t, |_| Message::Noop),
        "tooltip" => {
            let tp = leak(tooltip::Tooltip::new("提示内容").with_visible(true));
            tp.view(t, text("悬停/点击查看提示").size(14).into())
        }
        "tour" => {
            let tr = leak(
                tour::Tour::new()
                    .with_step(tour::TourStep::new("欢迎", "引导第一步"))
                    .with_step(tour::TourStep::new("完成", "引导结束")),
            );
            tr.handle(tour::TourMessage::Open);
            tr.view(t, |_| Message::Noop)
        }
        "transfer" => leak(transfer::Transfer::new(vec![
            transfer::TransferItem::new("a", "选项A", transfer::Side::Left),
            transfer::TransferItem::new("b", "选项B", transfer::Side::Left),
        ]))
        .view(t, |_| Message::Noop),
        "tree" => leak(tree::Tree::new().with_data(vec![
            tree::TreeNode::new("1", "根节点").with_child(tree::TreeNode::new("1-1", "子节点")),
        ]))
        .view(t, |_| Message::Noop, |_| Message::Noop),
        "tree_select" => leak(tree_select::TreeSelect::new()).view(t, |_| Message::Noop),
        "watermark" => {
            let wm = leak(watermark::Watermark::new("HarUI 水印"));
            wm.view(
                t,
                container(text("内容区域"))
                    .width(Length::Fill)
                    .height(Length::Fixed(100.0))
                    .center_x(Length::Fill)
                    .center_y(Length::Fill)
                    .into(),
            )
        }
        "form" => {
            let f = leak(
                form::Form::new()
                    .with_item(form::FormItem::new("name", "姓名"))
                    .with_item(form::FormItem::new("phone", "手机号")),
            );
            f.view(t, |item| text(item.label().to_string()).size(13).into())
        }
        "menu" => {
            let m = leak(menu::MenuState::new());
            m.register_item(menu::MenuItem::new("1", "首页"));
            m.register_item(menu::MenuItem::new("2", "订单管理"));
            m.register_item(menu::MenuItem::new("3", "设置"));
            m.view(t, |_| Message::Noop)
        }
        "message_box" => {
            let mb = leak(message_box::MessageBox::new("提示", "操作成功"));
            mb.handle(message_box::MessageBoxMessage::Open);
            mb.view(t, |_| Message::Noop)
        }
        "notification" => leak(
            notification::Notification::new("通知标题", "这是一条通知内容")
                .with_type(notification::NotificationType::Success),
        )
        .view(t)
        .map(|_| Message::Noop),
        "payment" => {
            leak(payment::Payment::new(100.0)).view(t, |_| Message::Noop, || Message::Noop)
        }
        "table" => {
            let tb = leak(
                table::Table::<DemoRow>::new()
                    .with_columns(vec![
                        table::TableColumn::new("name", "名称").with_width(120.0),
                        table::TableColumn::new("price", "价格").with_width(80.0),
                    ])
                    .with_rows(vec![
                        DemoRow {
                            id: "1".into(),
                            name: "商品A".into(),
                            price: "10.00".into(),
                        },
                        DemoRow {
                            id: "2".into(),
                            name: "商品B".into(),
                            price: "20.00".into(),
                        },
                        DemoRow {
                            id: "3".into(),
                            name: "商品C".into(),
                            price: "30.00".into(),
                        },
                    ])
                    .with_props(table::TableProps::new().with_border(true))
                    .with_field_extractor(demo_row_field),
            );
            tb.view(t, demo_row_field, |_| Message::Noop)
        }
        "badge" => leak(
            badge::Badge::new(badge::BadgeValue::Number(5))
                .with_max(99)
                .with_position(badge::BadgePosition::TopRight),
        )
        .view(t)
        .map(|_| Message::Noop),
        "breadcrumb" => leak(
            breadcrumb::Breadcrumb::new()
                .with_item(breadcrumb::BreadcrumbItem::new("首页"))
                .with_item(breadcrumb::BreadcrumbItem::new("组件"))
                .with_item(breadcrumb::BreadcrumbItem::new("面包屑"))
                .with_separator("/"),
        )
        .view(t, |_| Message::Noop),
        "date_picker" => leak(date_picker::DatePicker::new().with_placeholder("选择日期"))
            .view(t, |_| Message::Noop),
        "dialog" => {
            let d = leak(dialog::Dialog::new("提示", "这是对话框内容"));
            d.handle(dialog::DialogMessage::Open);
            d.handle(dialog::DialogMessage::AnimationFinished);
            d.view(t, Message::Noop)
        }
        "dropdown" => {
            let d = leak(
                dropdown::Dropdown::new()
                    .with_item(dropdown::DropdownItem::new("edit", "编辑"))
                    .with_item(dropdown::DropdownItem::new("del", "删除"))
                    .with_trigger(dropdown::DropdownTrigger::Click),
            );
            d.handle(dropdown::DropdownMessage::Show);
            d.view(t, || Message::Noop)
        }
        "image" => {
            let img = leak(
                image::Image::new("https://example.com/demo.png")
                    .with_alt("示例图片")
                    .with_size(120.0, 80.0),
            );
            img.handle(image::ImageMessage::Load);
            img.handle(image::ImageMessage::LoadSuccess);
            img.view(t, Some(text("示例图片").size(13).into()), |_| {
                Message::Noop
            })
        }
        "loading" => {
            let l = leak(loading::Loading::new().with_text("加载中…"));
            l.handle(loading::LoadingMessage::Start);
            l.view(
                t,
                container(text("业务内容区域"))
                    .width(Length::Fill)
                    .height(Length::Fixed(100.0))
                    .center_x(Length::Fill)
                    .center_y(Length::Fill)
                    .into(),
            )
        }
        "message" => leak(message::MessageItem::new(
            "操作成功",
            message::MessageType::Success,
        ))
        .view(t)
        .map(|_| Message::Noop),
        "popconfirm" => {
            let p = leak(
                popconfirm::Popconfirm::new("确认删除该记录？")
                    .with_visible(true)
                    .with_confirm_text("确定")
                    .with_cancel_text("取消"),
            );
            p.view(
                t,
                text("删除").size(14).into(),
                || Message::Noop,
                || Message::Noop,
            )
        }
        "space" => {
            let sp = leak(space::Space::new().with_size(8));
            let children: Vec<Element<'static, Message>> = vec![
                leak(tag::Tag::new("标签一")).view(t, || Message::Noop),
                leak(tag::Tag::new("标签二")).view(t, || Message::Noop),
                leak(tag::Tag::new("标签三")).view(t, || Message::Noop),
            ];
            sp.view(t, children)
        }
        "splitter" => {
            let sp = leak(splitter::Splitter::new(2));
            let panes: Vec<Element<'static, Message>> = vec![
                container(text("左面板"))
                    .padding(Padding::from(8u16))
                    .into(),
                container(text("右面板"))
                    .padding(Padding::from(8u16))
                    .into(),
            ];
            sp.view(t, panes)
        }
        "tabs" => leak(
            tabs::Tabs::new()
                .with_item(tabs::TabItem::new("a", "标签一"))
                .with_item(tabs::TabItem::new("b", "标签二")),
        )
        .view(t, |_| Message::Noop),
        "upload" => leak(upload::Upload::new()).view(t, || Message::Noop),
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
