//! EP 全组件复刻对照页 — URL hash 选组件独立渲染
use har_ui_core::theme::Theme;
use iced::widget::{Space, column, container, mouse_area, row, scrollable, text};
use iced::{Color, Element, Font, Length, Padding, Task};

struct State {
    selected: String,
}

#[derive(Debug, Clone)]
enum Message {
    Noop,
    Select(&'static str),
}

fn update(s: &mut State, m: Message) -> Task<Message> {
    match m {
        Message::Noop => {}
        Message::Select(name) => {
            s.selected = name.to_string();
            // 同步 URL hash，刷新/分享链接直达同一组件（不触发页面重载）
            #[cfg(target_arch = "wasm32")]
            {
                if let Some(w) = web_sys::window() {
                    let _ = w.location().set_hash(&format!("#{}", name));
                }
            }
        }
    }
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

/// EP 文档站分组（与 element-plus.org 侧栏目录一致；收银业务组件为 HarPOS 扩展组）
const GROUPS: &[(&str, &[&str])] = &[
    ("基础组件", &["button", "link", "text", "scrollbar"]),
    ("布局", &["grid", "divider", "space", "splitter"]),
    (
        "导航",
        &[
            "affix",
            "anchor",
            "backtop",
            "breadcrumb",
            "dropdown",
            "menu",
            "page_header",
            "pagination",
            "steps",
            "tabs",
        ],
    ),
    (
        "数据录入",
        &[
            "autocomplete",
            "cascader",
            "checkbox",
            "color_picker",
            "date_picker",
            "form",
            "input",
            "input_number",
            "input_tag",
            "radio",
            "rate",
            "select",
            "slider",
            "switch",
            "time_picker",
            "transfer",
            "tree_select",
            "upload",
        ],
    ),
    (
        "数据展示",
        &[
            "avatar",
            "badge",
            "calendar",
            "card",
            "carousel",
            "collapse",
            "countdown",
            "descriptions",
            "empty",
            "image",
            "progress",
            "result_page",
            "skeleton",
            "statistic",
            "table",
            "tag",
            "timeline",
            "tree",
        ],
    ),
    (
        "反馈",
        &[
            "alert",
            "dialog",
            "drawer",
            "loading",
            "message",
            "message_box",
            "notification",
            "popconfirm",
            "popover",
            "tooltip",
            "tour",
        ],
    ),
    ("配置", &["watermark"]),
    (
        "收银业务",
        &[
            "customer_display",
            "hang_order",
            "keypad",
            "payment",
            "status_bar",
        ],
    ),
];

/// 组件中文名（与 Element Plus 官方命名一致；收银业务为业务命名）
fn label(name: &str) -> &'static str {
    match name {
        "affix" => "固钉",
        "alert" => "提示",
        "anchor" => "锚点",
        "autocomplete" => "自动补全",
        "avatar" => "头像",
        "backtop" => "返回顶部",
        "badge" => "徽章",
        "breadcrumb" => "面包屑",
        "button" => "按钮",
        "calendar" => "日历",
        "card" => "卡片",
        "carousel" => "走马灯",
        "cascader" => "级联选择器",
        "checkbox" => "多选框",
        "collapse" => "折叠面板",
        "color_picker" => "颜色选择器",
        "countdown" => "倒计时",
        "customer_display" => "客显屏",
        "date_picker" => "日期选择器",
        "descriptions" => "描述列表",
        "dialog" => "对话框",
        "divider" => "分割线",
        "drawer" => "抽屉",
        "dropdown" => "下拉菜单",
        "empty" => "空状态",
        "form" => "表单",
        "grid" => "栅格",
        "hang_order" => "挂单",
        "image" => "图片",
        "input" => "输入框",
        "input_number" => "数字输入框",
        "input_tag" => "输入标签",
        "keypad" => "收银键盘",
        "link" => "链接",
        "loading" => "加载",
        "menu" => "导航菜单",
        "message" => "消息提示",
        "message_box" => "弹框",
        "notification" => "通知",
        "page_header" => "页头",
        "pagination" => "分页",
        "payment" => "收银支付",
        "popconfirm" => "气泡确认框",
        "popover" => "气泡弹出框",
        "progress" => "进度条",
        "radio" => "单选框",
        "rate" => "评分",
        "result_page" => "结果页",
        "scrollbar" => "滚动条",
        "select" => "选择器",
        "skeleton" => "骨架屏",
        "slider" => "滑块",
        "space" => "间距",
        "splitter" => "分割面板",
        "statistic" => "统计数值",
        "status_bar" => "状态栏",
        "steps" => "步骤条",
        "switch" => "开关",
        "table" => "表格",
        "tabs" => "标签页",
        "tag" => "标签",
        "text" => "文本",
        "time_picker" => "时间选择器",
        "timeline" => "时间线",
        "tooltip" => "文字提示",
        "tour" => "漫游式引导",
        "transfer" => "穿梭框",
        "tree" => "树形控件",
        "tree_select" => "树形选择器",
        "upload" => "上传",
        "watermark" => "水印",
        _ => "",
    }
}

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
    stock: String,
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
        "stock" => r.stock.clone(),
        _ => String::new(),
    }
}

fn view(s: &State) -> Element<'_, Message> {
    let t = theme();
    let name = s.selected.as_str();
    // 空 hash 或未知组件名 → 文档站式总览（侧栏 + 分组卡片），避免白屏/堆叠页
    if name.is_empty() || !ALL.contains(&name) {
        return docs_layout(name, t).into();
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

/// 文档站式总览布局：顶栏 + 左侧 EP 分组侧栏 + 右侧总览卡片（点击进 #组件名 独立页）
fn docs_layout(unknown: &str, t: &'static Theme) -> Element<'static, Message> {
    let header = container(row![
        text("HarUI").size(18).color(Color::from(t.primary.base)),
        text(" 组件库")
            .size(18)
            .color(Color::from(t.neutral.text_primary)),
        Space::new().width(Length::Fill),
        text("71 组件 · 对齐 Element Plus 规格")
            .size(12)
            .color(Color::from(t.neutral.text_secondary)),
    ])
    .width(Length::Fill)
    .height(Length::Fixed(48.0))
    .padding(Padding {
        top: 0.0,
        bottom: 0.0,
        left: 24.0,
        right: 24.0,
    });

    let rule = |w: Length, h: Length| {
        container(
            Space::new()
                .width(Length::Fixed(0.0))
                .height(Length::Fixed(0.0)),
        )
        .width(w)
        .height(h)
        .style(move |_| container::Style {
            background: Some(iced::Background::Color(Color::from(
                t.neutral.border_lighter,
            ))),
            ..Default::default()
        })
    };

    // 左侧栏：分组目录，点击切换组件
    let mut side = column![];
    for &(gname, items) in GROUPS {
        side = side.push(
            container(
                text(gname)
                    .size(12)
                    .color(Color::from(t.neutral.text_placeholder)),
            )
            .width(Length::Fill)
            .padding(Padding {
                top: 18.0,
                bottom: 6.0,
                left: 24.0,
                right: 24.0,
            }),
        );
        for &id in items.iter() {
            side = side.push(
                mouse_area(
                    container(
                        text(label(id))
                            .size(14)
                            .color(Color::from(t.neutral.text_regular)),
                    )
                    .width(Length::Fill)
                    .padding(Padding {
                        top: 7.0,
                        bottom: 7.0,
                        left: 24.0,
                        right: 24.0,
                    }),
                )
                .on_press(Message::Select(id)),
            );
        }
    }
    let sidebar = scrollable(side)
        .width(Length::Fixed(240.0))
        .height(Length::Fill);

    // 右侧总览：标题 + 分组卡片栅格（每行 4 张）
    let mut main_col = column![].spacing(12).padding(Padding {
        top: 28.0,
        bottom: 32.0,
        left: 32.0,
        right: 32.0,
    });
    main_col = main_col.push(
        text("HarUI 组件库")
            .size(26)
            .color(Color::from(t.neutral.text_primary)),
    );
    main_col = main_col.push(
        text("71 组件 · 与 Element Plus 对齐 · 像素对照复刻页")
            .size(14)
            .color(Color::from(t.neutral.text_secondary)),
    );
    if !unknown.is_empty() {
        main_col = main_col.push(
            text(format!(
                "未知组件 “{}”，已回退总览。正确格式：#button / #table / #form …",
                unknown
            ))
            .size(13)
            .color(Color::from(t.warning.base)),
        );
    }
    main_col = main_col.push(
        text("点击卡片进入组件独立对照页（URL hash 路由，可刷新/分享）")
            .size(12)
            .color(Color::from(t.neutral.text_placeholder)),
    );
    for &(gname, items) in GROUPS {
        main_col = main_col.push(
            row![
                text(gname)
                    .size(16)
                    .color(Color::from(t.neutral.text_primary)),
                text(format!(" {}", items.len()))
                    .size(12)
                    .color(Color::from(t.neutral.text_placeholder)),
            ]
            .spacing(2),
        );
        for chunk in items.chunks(4) {
            let mut r = row![].spacing(12);
            for &id in chunk.iter() {
                r = r.push(component_card(id, t));
            }
            main_col = main_col.push(r);
        }
    }
    main_col = main_col.push(
        text("收银业务组件为 HarPOS 扩展（EP 无对应页）；对照基线帧存 docs/visual-baseline/。")
            .size(12)
            .color(Color::from(t.neutral.text_placeholder)),
    );
    let main_area = scrollable(main_col)
        .width(Length::Fill)
        .height(Length::Fill);

    container(
        column![
            header,
            rule(Length::Fill, Length::Fixed(1.0)),
            row![sidebar, rule(Length::Fixed(1.0), Length::Fill), main_area].height(Length::Fill),
        ]
        .width(Length::Fill)
        .height(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(move |_| container::Style {
        background: Some(iced::Background::Color(Color::WHITE)),
        ..Default::default()
    })
    .into()
}

/// 总览页组件卡片：组件 id（主色）+ 中文名，点击进入 #组件名
fn component_card(id: &'static str, t: &'static Theme) -> Element<'static, Message> {
    mouse_area(
        container(
            column![
                text(id).size(14).color(Color::from(t.primary.base)),
                text(label(id))
                    .size(12)
                    .color(Color::from(t.neutral.text_secondary)),
            ]
            .spacing(4),
        )
        .width(Length::Fixed(224.0))
        .padding(Padding {
            top: 14.0,
            bottom: 14.0,
            left: 16.0,
            right: 16.0,
        })
        .style(move |_| container::Style {
            background: Some(iced::Background::Color(Color::WHITE)),
            border: iced::Border {
                color: Color::from(t.neutral.border_light),
                width: 1.0,
                radius: 6.0.into(),
            },
            ..Default::default()
        }),
    )
    .on_press(Message::Select(id))
    .into()
}

/// EP 文档风格演示段：小节标题 + 带边框演示盒（对齐 element-plus.org 演示区观感）
fn section(title: &str, body: Element<'static, Message>) -> Element<'static, Message> {
    let t = theme();
    column![
        text(title.to_string())
            .size(15)
            .color(Color::from(t.neutral.text_primary)),
        container(body)
            .width(Length::Fill)
            .padding(Padding::from(20u16))
            .style(move |_| container::Style {
                background: Some(iced::Background::Color(Color::WHITE)),
                border: iced::Border {
                    color: Color::from(t.neutral.border_lighter),
                    width: 1.0,
                    radius: iced::border::radius(4.0),
                },
                ..Default::default()
            }),
    ]
    .spacing(10)
    .into()
}

/// 固定宽度包裹（EP 输入类控件默认 ~180px，避免拉伸全宽）
fn at_width(e: Element<'static, Message>, w: f32) -> Element<'static, Message> {
    container(e).width(Length::Fixed(w)).into()
}

fn render(name: &str, t: &'static Theme) -> Element<'static, Message> {
    use har_ui_components::*;
    match name {
        "button" => {
            let names = ["Default", "Primary", "Success", "Info", "Warning", "Danger"];
            let types = [
                button::ButtonType::Default,
                button::ButtonType::Primary,
                button::ButtonType::Success,
                button::ButtonType::Info,
                button::ButtonType::Warning,
                button::ButtonType::Danger,
            ];
            let row_of = |plain: bool, round: bool, disabled: bool| {
                let mut r = row![].spacing(12);
                for i in 0..6 {
                    let mut b = button::Button::new(names[i].to_string()).with_type(types[i]);
                    if plain {
                        b = b.plain(true);
                    }
                    if round {
                        b = b.round(true);
                    }
                    if disabled {
                        b = b.disabled(true);
                    }
                    r = r.push(leak(b).view(t, Message::Noop));
                }
                r
            };
            column![
                section("基础用法", row_of(false, false, false).into()),
                section("朴素按钮", row_of(true, false, false).into()),
                section("圆角按钮", row_of(false, true, false).into()),
                section("禁用状态", row_of(false, false, true).into()),
                section(
                    "按钮尺寸",
                    row![
                        leak(button::Button::new("Large").with_size(button::ButtonSize::Large))
                            .view(t, Message::Noop),
                        leak(button::Button::new("Default")).view(t, Message::Noop),
                        leak(button::Button::new("Small").with_size(button::ButtonSize::Small))
                            .view(t, Message::Noop),
                    ]
                    .spacing(12)
                    .into()
                ),
            ]
            .spacing(16)
            .padding(Padding::from(16u16))
            .into()
        }
        "input" => column![
            section(
                "基础用法",
                at_width(
                    leak(input::Input::new().with_placeholder("Please input"))
                        .view(t, |_| { Message::Noop }),
                    180.0,
                )
            ),
            section(
                "禁用状态",
                at_width(
                    leak(
                        input::Input::new()
                            .with_placeholder("Please input")
                            .disabled(true)
                    )
                    .view(t, |_| Message::Noop),
                    180.0,
                )
            ),
            section(
                "可清空",
                at_width(
                    leak(
                        input::Input::new()
                            .with_placeholder("请输入内容")
                            .with_clearable(true)
                    )
                    .view(t, |_| Message::Noop),
                    180.0,
                )
            ),
        ]
        .spacing(16)
        .padding(Padding::from(16u16))
        .into(),
        "tag" => {
            let names = ["Default", "Primary", "Success", "Info", "Warning", "Danger"];
            let types = [
                tag::TagType::Default,
                tag::TagType::Primary,
                tag::TagType::Success,
                tag::TagType::Info,
                tag::TagType::Warning,
                tag::TagType::Danger,
            ];
            let row_of = |effect: tag::TagEffect| {
                let mut r = row![].spacing(12);
                for i in 0..6 {
                    r = r.push(
                        leak(
                            tag::Tag::new(names[i].to_string())
                                .with_type(types[i])
                                .with_effect(effect),
                        )
                        .view(t, || Message::Noop),
                    );
                }
                r
            };
            column![
                section("基础用法", row_of(tag::TagEffect::Light).into()),
                section("Dark 主题", row_of(tag::TagEffect::Dark).into()),
                section(
                    "可关闭",
                    row![
                        leak(tag::Tag::new("标签一").with_closable(true)).view(t, || Message::Noop),
                        leak(tag::Tag::new("标签二").with_closable(true)).view(t, || Message::Noop),
                    ]
                    .spacing(12)
                    .into()
                ),
            ]
            .spacing(16)
            .padding(Padding::from(16u16))
            .into()
        }
        "progress" => column![
            section(
                "线形进度条",
                at_width(
                    column![
                        leak(progress::Progress::new().with_percentage(50))
                            .view(t)
                            .map(|_| Message::Noop),
                        leak(
                            progress::Progress::new()
                                .with_percentage(70)
                                .with_status(progress::ProgressStatus::Success)
                        )
                        .view(t)
                        .map(|_| Message::Noop),
                        leak(
                            progress::Progress::new()
                                .with_percentage(90)
                                .with_status(progress::ProgressStatus::Warning)
                        )
                        .view(t)
                        .map(|_| Message::Noop),
                        leak(
                            progress::Progress::new()
                                .with_percentage(40)
                                .with_status(progress::ProgressStatus::Exception)
                        )
                        .view(t)
                        .map(|_| Message::Noop),
                    ]
                    .spacing(16)
                    .into(),
                    560.0,
                )
            ),
            section(
                "环形进度条",
                row![
                    leak(
                        progress::Progress::new()
                            .with_type(progress::ProgressType::Circle)
                            .with_percentage(50)
                    )
                    .view(t)
                    .map(|_| Message::Noop),
                    leak(
                        progress::Progress::new()
                            .with_type(progress::ProgressType::Circle)
                            .with_percentage(70)
                            .with_status(progress::ProgressStatus::Success)
                    )
                    .view(t)
                    .map(|_| Message::Noop),
                ]
                .spacing(16)
                .into()
            ),
        ]
        .spacing(16)
        .padding(Padding::from(16u16))
        .into(),
        "card" => at_width(
            leak(
                card::Card::new("今日销售额较昨日增长 12%，客单价 35.6 元。")
                    .with_header("经营概况"),
            )
            .view(t)
            .map(|_| Message::Noop),
            480.0,
        ),
        "switch" => row![
            leak(switch::Switch::new()).view(t, Message::Noop),
            leak(switch::Switch::with_value(true)).view(t, Message::Noop),
            leak(switch::Switch::new().with_disabled(true)).view(t, Message::Noop),
            leak(switch::Switch::with_value(true).with_disabled(true)).view(t, Message::Noop),
            leak(
                switch::Switch::with_value(true)
                    .with_active_text("按月付费")
                    .with_inactive_text("按日付费")
            )
            .view(t, Message::Noop),
        ]
        .spacing(16)
        .padding(Padding::from(16u16))
        .into(),
        "checkbox" => column![
            section(
                "基础用法",
                leak(checkbox::CheckboxGroup::new().with_value(vec!["a", "c"])).view(
                    t,
                    &[
                        ("a", "选项A"),
                        ("b", "选项B"),
                        ("c", "选项C"),
                        ("d", "选项D")
                    ],
                    |_| Message::Noop,
                )
            ),
            section(
                "禁用与选中",
                row![
                    leak(
                        checkbox::Checkbox::new("选中且不可选")
                            .with_checked(true)
                            .with_disabled(true)
                    )
                    .view(t, Message::Noop),
                    leak(checkbox::Checkbox::new("未选且不可选").with_disabled(true))
                        .view(t, Message::Noop),
                ]
                .spacing(16)
                .into()
            ),
        ]
        .spacing(16)
        .padding(Padding::from(16u16))
        .into(),
        "radio" => column![
            section(
                "基础用法",
                leak(radio::RadioGroup::new().with_value("a")).view(
                    t,
                    &[("a", "选项A"), ("b", "选项B"), ("c", "选项C")],
                    |_| Message::Noop,
                )
            ),
            section(
                "禁用与选中",
                row![
                    leak(
                        radio::Radio::new("选中且不可选")
                            .with_checked(true)
                            .with_disabled(true)
                    )
                    .view(t, Message::Noop),
                    leak(radio::Radio::new("未选且不可选").with_disabled(true))
                        .view(t, Message::Noop),
                ]
                .spacing(16)
                .into()
            ),
        ]
        .spacing(16)
        .padding(Padding::from(16u16))
        .into(),
        "slider" => column![
            section(
                "基础用法",
                at_width(
                    leak(
                        slider::Slider::new()
                            .with_min(0.0)
                            .with_max(100.0)
                            .with_value(30.0)
                    )
                    .view(t, |_| Message::Noop,),
                    480.0,
                )
            ),
            section(
                "范围选择",
                at_width(
                    leak(
                        slider::Slider::new()
                            .with_min(0.0)
                            .with_max(100.0)
                            .with_range(true)
                            .with_range_value(20.0, 70.0)
                    )
                    .view(t, |_| Message::Noop),
                    480.0,
                )
            ),
        ]
        .spacing(16)
        .padding(Padding::from(16u16))
        .into(),
        "rate" => column![
            section(
                "基础用法",
                leak(rate::Rate::new().with_value(4.0)).view(t, |_| Message::Noop)
            ),
            section(
                "允许半选",
                leak(rate::Rate::new().with_value(3.5).with_allow_half(true))
                    .view(t, |_| Message::Noop,)
            ),
        ]
        .spacing(16)
        .padding(Padding::from(16u16))
        .into(),
        "link" => {
            let names = ["Default", "Primary", "Success", "Info", "Warning", "Danger"];
            let types = [
                link::LinkType::Default,
                link::LinkType::Primary,
                link::LinkType::Success,
                link::LinkType::Info,
                link::LinkType::Warning,
                link::LinkType::Danger,
            ];
            let mut r1 = row![].spacing(16);
            let mut r2 = row![].spacing(16);
            for i in 0..6 {
                r1 = r1.push(
                    leak(
                        link::Link::new()
                            .with_text(names[i].to_string())
                            .with_type(types[i]),
                    )
                    .view(t, Message::Noop),
                );
                r2 = r2.push(
                    leak(
                        link::Link::new()
                            .with_text(names[i].to_string())
                            .with_type(types[i])
                            .with_disabled(true),
                    )
                    .view(t, Message::Noop),
                );
            }
            column![
                section("基础用法", r1.into()),
                section("禁用状态", r2.into()),
            ]
            .spacing(16)
            .padding(Padding::from(16u16))
            .into()
        }
        "divider" => leak(divider::Divider::new()).view(t).map(|_| Message::Noop),
        "empty" => leak(empty::Empty::new().with_description("暂无数据"))
            .view(t)
            .map(|_| Message::Noop),
        "alert" => {
            let al = |ty: alert::AlertType, title: &'static str, desc: &'static str| {
                leak(
                    alert::Alert::new()
                        .with_type(ty)
                        .with_title(title)
                        .with_description(desc)
                        .with_show_icon(true),
                )
                .view(t, || Message::Noop)
            };
            column![
                section(
                    "主题",
                    column![
                        al(alert::AlertType::Success, "成功提示", "成功状态的描述文字"),
                        al(alert::AlertType::Warning, "警告提示", "警告状态的描述文字"),
                        al(alert::AlertType::Info, "信息提示", "信息状态的描述文字"),
                        al(alert::AlertType::Error, "错误提示", "错误状态的描述文字"),
                    ]
                    .spacing(12)
                    .into()
                ),
                section(
                    "可关闭",
                    leak(
                        alert::Alert::new()
                            .with_type(alert::AlertType::Info)
                            .with_title("可关闭提示")
                            .with_description("点击右侧 × 关闭本条提示")
                            .with_show_icon(true)
                            .with_closable(true)
                    )
                    .view(t, || Message::Noop),
                ),
            ]
            .spacing(16)
            .padding(Padding::from(16u16))
            .into()
        }
        "text" => leak(text::Text::new().with_content("正文文字"))
            .view(t)
            .map(|_| Message::Noop),
        "statistic" => row![
            leak(
                statistic::Statistic::new(statistic::StatisticValue::Int(65321))
                    .with_title("今日订单")
            )
            .view(t)
            .map(|_| Message::Noop),
            leak(
                statistic::Statistic::new(statistic::StatisticValue::Float(35.6))
                    .with_title("客单价（元）")
                    .with_precision(1)
            )
            .view(t)
            .map(|_| Message::Noop),
            leak(
                statistic::Statistic::new(statistic::StatisticValue::Float(12.8))
                    .with_title("同比增长")
                    .with_suffix("%")
                    .with_precision(1)
            )
            .view(t)
            .map(|_| Message::Noop),
        ]
        .spacing(32)
        .padding(Padding::from(16u16))
        .into(),
        "skeleton" => leak(skeleton::Skeleton::new())
            .view(t)
            .map(|_| Message::Noop),
        "avatar" => row![
            leak(avatar::Avatar::new().with_text("U"))
                .view(t)
                .map(|_| Message::Noop),
            leak(
                avatar::Avatar::new()
                    .with_text("L")
                    .with_shape(avatar::AvatarShape::Square)
            )
            .view(t)
            .map(|_| Message::Noop),
            leak(
                avatar::Avatar::new()
                    .with_text("大")
                    .with_size(avatar::AvatarSize::Large)
            )
            .view(t)
            .map(|_| Message::Noop),
            leak(
                avatar::Avatar::new()
                    .with_text("小")
                    .with_size(avatar::AvatarSize::Small)
            )
            .view(t)
            .map(|_| Message::Noop),
        ]
        .spacing(16)
        .padding(Padding::from(16u16))
        .into(),
        "result_page" => leak(result_page::ResultPage::new().with_title("操作成功"))
            .view(t)
            .map(|_| Message::Noop),
        "pagination" => leak(pagination::Pagination::new(100, 10)).view(t, |_| Message::Noop),
        "collapse" => {
            let mut c = collapse::Collapse::new();
            c.add_item(
                collapse::CollapseItem::new("i1", "一致性 Consistency").with_content(
                    "与现实生活一致：与现实生活的流程、逻辑保持一致，遵循用户习惯的语言和概念。",
                ),
            );
            c.add_item(
                collapse::CollapseItem::new("i2", "反馈 Feedback")
                    .with_content("通过界面样式和交互动效让用户可以清晰的感知自己的操作。"),
            );
            c.add_item(
                collapse::CollapseItem::new("i3", "效率 Efficiency")
                    .with_content("简化用户操作流程，清晰明确的语言告知用户接下来要做的事情。"),
            );
            c.handle(collapse::CollapseMessage::Toggle("i1".into()));
            at_width(leak(c).view(t, |_| Message::Noop), 640.0)
        }
        "timeline" => {
            let tl = leak(timeline::Timeline::new());
            tl.handle(timeline::TimelineMessage::AddItem(
                timeline::TimelineItem::new("2026-10-01 09:00", "创建订单"),
            ));
            tl.handle(timeline::TimelineMessage::AddItem(
                timeline::TimelineItem::new("2026-10-02 10:30", "审核通过")
                    .with_type(timeline::TimelineItemType::Success),
            ));
            tl.handle(timeline::TimelineMessage::AddItem(
                timeline::TimelineItem::new("2026-10-03 14:20", "库存不足预警")
                    .with_type(timeline::TimelineItemType::Warning),
            ));
            tl.handle(timeline::TimelineMessage::AddItem(
                timeline::TimelineItem::new("2026-10-04 18:00", "超时未支付，订单关闭")
                    .with_type(timeline::TimelineItemType::Danger),
            ));
            at_width(tl.view(t).map(|_| Message::Noop), 560.0)
        }
        "select" => {
            let s = leak(
                select::Select::new()
                    .with_option(select::SelectOption::new("a", "选项A"))
                    .with_option(select::SelectOption::new("b", "选项B"))
                    .with_option(select::SelectOption::new("c", "选项C")),
            );
            at_width(s.view(t, |_| Message::Noop), 220.0)
        }
        "countdown" => leak(countdown::Countdown::new(60_000)).view(t, |_| Message::Noop),
        "input_number" => {
            let i = leak(
                input_number::InputNumber::new()
                    .with_min(1.0)
                    .with_max(10.0),
            );
            i.handle(input_number::InputNumberMessage::SetValue(3.0));
            at_width(
                i.view(t, |_| Message::Noop, Message::Noop, Message::Noop),
                180.0,
            )
        }
        "input_tag" => {
            let it = leak(input_tag::InputTag::new().with_placeholder("输入后回车"));
            it.handle(input_tag::InputTagMessage::SetTags(vec![
                "标签1".into(),
                "标签2".into(),
            ]));
            at_width(it.view(t, |_| Message::Noop), 280.0)
        }
        "steps" => {
            let st = leak(
                steps::Steps::new()
                    .with_step(steps::Step::new("步骤 1").with_description("填写订单信息"))
                    .with_step(steps::Step::new("步骤 2").with_description("确认订单"))
                    .with_step(steps::Step::new("步骤 3").with_description("在线支付"))
                    .with_step(steps::Step::new("步骤 4").with_description("完成")),
            );
            st.handle(steps::StepsMessage::JumpTo(2));
            at_width(st.view(t).map(|_| Message::Noop), 640.0)
        }
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
            at_width(a.view(t, |_| Message::Noop), 220.0)
        }
        "affix" => leak(affix::Affix::new()).view(t, text("固定内容").size(14).into()),
        "backtop" => leak(backtop::Backtop::new()).view(t, Message::Noop),
        "calendar" => leak(calendar::Calendar::new(2026, 10)).view(t, |_| Message::Noop),
        "carousel" => {
            let c = leak(
                carousel::Carousel::new()
                    .with_slide("第 1 页 — 新品上市")
                    .with_slide("第 2 页 — 限时折扣")
                    .with_slide("第 3 页 — 会员日活动")
                    .with_slide("第 4 页 — 满减优惠")
                    .with_autoplay(true)
                    .with_interval(3000),
            );
            at_width(c.view(t, || Message::Noop, || Message::Noop), 640.0)
        }
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
            at_width(cs.view(t, |_| Message::Noop), 220.0)
        }
        "color_picker" => leak(color_picker::ColorPicker::new())
            .view(t)
            .map(|_| Message::Noop),
        "customer_display" => leak(customer_display::CustomerDisplay::new())
            .view(t)
            .map(|_| Message::Noop),
        "descriptions" => leak(
            descriptions::Descriptions::new()
                .with_title("订单信息")
                .with_item(descriptions::DescriptionsItem::new(
                    "订单号",
                    "SO20261012001",
                ))
                .with_item(descriptions::DescriptionsItem::new("收货人", "张三"))
                .with_item(descriptions::DescriptionsItem::new("手机", "138****8888"))
                .with_item(descriptions::DescriptionsItem::new("状态", "已发货")),
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
        "page_header" => leak(
            page_header::PageHeader::new()
                .with_title("返回上一级")
                .with_content("门店详情"),
        )
        .view(t, || Message::Noop),
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
        "time_picker" => at_width(
            leak(time_picker::TimePicker::new()).view(t, |_| Message::Noop),
            200.0,
        ),
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
        "transfer" => {
            let tr = leak(
                transfer::Transfer::new(vec![
                    transfer::TransferItem::new("a", "有机蔬菜", transfer::Side::Left),
                    transfer::TransferItem::new("b", "新鲜水果", transfer::Side::Left),
                    transfer::TransferItem::new("c", "肉禽蛋品", transfer::Side::Left),
                    transfer::TransferItem::new("d", "海鲜水产", transfer::Side::Left),
                    transfer::TransferItem::new("e", "粮油副食", transfer::Side::Left),
                    transfer::TransferItem::new("f", "休闲零食", transfer::Side::Left),
                ])
                .with_titles("全部类目", "已选类目"),
            );
            tr.handle(transfer::TransferMessage::ToggleCheckLeft("a".into()));
            tr.handle(transfer::TransferMessage::ToggleCheckLeft("c".into()));
            tr.handle(transfer::TransferMessage::ToRight);
            tr.handle(transfer::TransferMessage::ToggleCheckLeft("a".into()));
            at_width(tr.view(t, |_| Message::Noop), 640.0)
        }
        "tree" => leak(tree::Tree::new().with_data(vec![
            tree::TreeNode::new("1", "一级 1").with_child(tree::TreeNode::new("1-1", "二级 1-1")
                .with_child(tree::TreeNode::new("1-1-1", "三级 1-1-1"))),
            tree::TreeNode::new("2", "一级 2").with_child(
                tree::TreeNode::new("2-1", "二级 2-1")
                    .with_child(tree::TreeNode::new("2-1-1", "三级 2-1-1"))
                    .with_child(tree::TreeNode::new("2-1-2", "三级 2-1-2")),
            ),
        ]))
        .view(t, |_| Message::Noop, |_| Message::Noop),
        "tree_select" => {
            let ts = leak(tree_select::TreeSelect::new().with_data(vec![
                    tree::TreeNode::new("1", "一级 1")
                        .with_child(tree::TreeNode::new("1-1", "二级 1-1")),
                ]));
            at_width(ts.view(t, |_| Message::Noop), 220.0)
        }
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
                    .with_item(form::FormItem::new("name", "活动名称"))
                    .with_item(form::FormItem::new("region", "活动区域"))
                    .with_item(form::FormItem::new("phone", "联系电话")),
            );
            at_width(
                f.view(t, |item| {
                    leak(input::Input::new().with_placeholder(format!("请输入{}", item.label())))
                        .view(t, |_| Message::Noop)
                }),
                560.0,
            )
        }
        "menu" => {
            let m = leak(menu::MenuState::new().with_mode(menu::MenuMode::Horizontal));
            m.register_item(menu::MenuItem::new("1", "首页"));
            m.register_item(
                menu::MenuItem::new("2", "订单管理").with_child(
                    menu::MenuItem::new("2-1", "订单列表")
                        .with_child(menu::MenuItem::new("2-1-1", "待支付")),
                ),
            );
            m.register_item(menu::MenuItem::new("3", "商品管理"));
            m.register_item(menu::MenuItem::new("4", "会员中心"));
            m.handle(menu::MenuMessage::Select("1".into()));
            at_width(m.view(t, |_| Message::Noop), 640.0)
        }
        "message_box" => {
            let mb = leak(message_box::MessageBox::new("提示", "操作成功"));
            mb.handle(message_box::MessageBoxMessage::Open);
            mb.view(t, |_| Message::Noop)
        }
        "notification" => column![
            leak(
                notification::Notification::new("成功通知", "这是一条成功的通知消息")
                    .with_type(notification::NotificationType::Success)
            )
            .view(t)
            .map(|_| Message::Noop),
            leak(
                notification::Notification::new("警告通知", "这是一条警告的通知消息")
                    .with_type(notification::NotificationType::Warning)
            )
            .view(t)
            .map(|_| Message::Noop),
            leak(
                notification::Notification::new("错误通知", "这是一条错误的通知消息")
                    .with_type(notification::NotificationType::Error)
            )
            .view(t)
            .map(|_| Message::Noop),
        ]
        .spacing(12)
        .padding(Padding::from(16u16))
        .into(),
        "payment" => {
            leak(payment::Payment::new(100.0)).view(t, |_| Message::Noop, || Message::Noop)
        }
        "table" => {
            let rows = vec![
                DemoRow {
                    id: "1".into(),
                    name: "有机番茄".into(),
                    price: "6.80".into(),
                    stock: "128".into(),
                },
                DemoRow {
                    id: "2".into(),
                    name: "黄瓜".into(),
                    price: "3.50".into(),
                    stock: "76".into(),
                },
                DemoRow {
                    id: "3".into(),
                    name: "本地土鸡蛋".into(),
                    price: "18.00".into(),
                    stock: "42".into(),
                },
                DemoRow {
                    id: "4".into(),
                    name: "五花猪肉".into(),
                    price: "26.50".into(),
                    stock: "18".into(),
                },
                DemoRow {
                    id: "5".into(),
                    name: "鲜活鲫鱼".into(),
                    price: "15.90".into(),
                    stock: "9".into(),
                },
            ];
            let tb = leak(
                table::Table::<DemoRow>::new()
                    .with_columns(vec![
                        table::TableColumn::new("name", "名称").with_width(160.0),
                        table::TableColumn::new("price", "单价（元）").with_width(120.0),
                        table::TableColumn::new("stock", "库存").with_width(100.0),
                    ])
                    .with_rows(rows)
                    .with_props(table::TableProps::new().with_border(true).with_stripe(true))
                    .with_field_extractor(demo_row_field),
            );
            at_width(tb.view(t, demo_row_field, |_| Message::Noop), 480.0)
        }
        "badge" => row![
            leak(
                badge::Badge::new(badge::BadgeValue::Number(12))
                    .with_max(99)
                    .with_position(badge::BadgePosition::TopRight)
            )
            .view(t)
            .map(|_| Message::Noop),
            leak(
                badge::Badge::new(badge::BadgeValue::Text("hot".into()))
                    .with_position(badge::BadgePosition::TopRight)
            )
            .view(t)
            .map(|_| Message::Noop),
            leak(
                badge::Badge::new(badge::BadgeValue::Number(200))
                    .with_max(99)
                    .with_position(badge::BadgePosition::TopRight)
            )
            .view(t)
            .map(|_| Message::Noop),
        ]
        .spacing(24)
        .padding(Padding::from(16u16))
        .into(),
        "breadcrumb" => leak(
            breadcrumb::Breadcrumb::new()
                .with_item(breadcrumb::BreadcrumbItem::new("首页"))
                .with_item(breadcrumb::BreadcrumbItem::new("组件"))
                .with_item(breadcrumb::BreadcrumbItem::new("面包屑"))
                .with_separator("/"),
        )
        .view(t, |_| Message::Noop),
        "date_picker" => at_width(
            leak(date_picker::DatePicker::new().with_placeholder("选择日期"))
                .view(t, |_| Message::Noop),
            200.0,
        ),
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
        "message" => column![
            leak(message::MessageItem::new(
                "成功消息",
                message::MessageType::Success
            ))
            .view(t)
            .map(|_| Message::Noop),
            leak(message::MessageItem::new(
                "警告消息",
                message::MessageType::Warning
            ))
            .view(t)
            .map(|_| Message::Noop),
            leak(message::MessageItem::new(
                "消息消息",
                message::MessageType::Info
            ))
            .view(t)
            .map(|_| Message::Noop),
            leak(message::MessageItem::new(
                "错误消息",
                message::MessageType::Error
            ))
            .view(t)
            .map(|_| Message::Noop),
        ]
        .spacing(12)
        .padding(Padding::from(16u16))
        .into(),
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
        "tabs" => {
            let tb = leak(
                tabs::Tabs::new()
                    .with_item(tabs::TabItem::new("a", "用户管理"))
                    .with_item(tabs::TabItem::new("b", "配置管理"))
                    .with_item(tabs::TabItem::new("c", "角色管理"))
                    .with_item(tabs::TabItem::new("d", "日志管理")),
            );
            tb.handle(tabs::TabsMessage::Select("b".into()));
            tb.view(t, |_| Message::Noop)
        }
        "upload" => {
            let up = leak(upload::Upload::new().with_multiple(true));
            up.handle(upload::UploadMessage::AddFile(upload::UploadFile::new(
                "美食封面.jpg",
                512_000,
            )));
            up.handle(upload::UploadMessage::AddFile(upload::UploadFile::new(
                "商品清单.xlsx",
                24_000,
            )));
            up.handle(upload::UploadMessage::AddFile(upload::UploadFile::new(
                "店铺logo.png",
                88_000,
            )));
            up.handle(upload::UploadMessage::UploadSuccess(0));
            up.handle(upload::UploadMessage::UploadSuccess(1));
            up.view(t, || Message::Noop)
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
        if s.selected.is_empty() || !ALL.contains(&s.selected.as_str()) {
            "HarUI 组件库".into()
        } else {
            format!("HarUI — {}", s.selected)
        }
    })
    // 注册字体：中文子集 + Arial 度量兼容的 Arimo（wasm 无系统字体回退）
    .font(include_bytes!("../fonts/NotoSansSC-HarUI-subset.otf").as_slice())
    .font(include_bytes!("../fonts/Arimo-Regular.ttf").as_slice())
    // 默认字体切为 Arimo（Arial 度量兼容），使拉丁/数字宽度与 Element Plus 对齐；
    // 中文仍由 Noto 子集回退渲染
    .default_font(Font::with_name("Arimo"))
    .run()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// GROUPS 必须不重不漏覆盖 ALL 的全部 71 个组件
    #[test]
    fn groups_cover_all_components_exactly() {
        let mut seen: Vec<&str> = Vec::new();
        for &(_, items) in GROUPS {
            for &id in items.iter() {
                assert!(ALL.contains(&id), "GROUPS 里的 {} 不在 ALL 中", id);
                assert!(!seen.contains(&id), "GROUPS 里 {} 重复", id);
                seen.push(id);
            }
        }
        assert_eq!(seen.len(), ALL.len(), "GROUPS 覆盖数 ≠ ALL（漏组件）");
    }

    /// 每个组件都要有中文名（总览卡片/侧栏展示用）
    #[test]
    fn every_component_has_label() {
        for &id in ALL {
            assert!(!label(id).is_empty(), "组件 {} 缺中文名", id);
        }
    }
}
