//! HarUI Screenshot Test — D.14 截图测试框架
//!
//! 渲染 12 个核心组件（与 showcase 相同），支持键盘快捷键与录制模式。
//! 真机像素级截图由 OS 工具完成（详见 docs/screenshot_test.md）。
//!
//! 快捷键：
//! - S: 打印当前画面截图清单条目（提示用 OS 工具截图）
//! - N: 切换下一个组件
//! - T: 切换主题（light / dark）
//! - R: 进入/退出录制模式（每 2 秒自动切换组件，遍历完 12 个后切换主题）
//! - Esc: 退出

use har_ui_components::button::{Button, ButtonType};
use har_ui_components::card::Card;
use har_ui_components::cascader::{Cascader, CascaderNode};
use har_ui_components::date_picker::DatePicker;
use har_ui_components::dialog::{Dialog, DialogMessage};
use har_ui_components::form::{Form, FormItem, FormMessage, FormRule, FormState};
use har_ui_components::input::{Input, InputMessage};
use har_ui_components::pagination::Pagination;
use har_ui_components::select::{Select, SelectOption};
use har_ui_components::table::{Table, TableColumn, TableProps, TableRow, VirtualScroll};
use har_ui_components::tabs::{TabItem, Tabs};
use har_ui_components::upload::Upload;
use har_ui_core::Theme;
use iced::keyboard::key::Named;
use iced::keyboard::{self, Key};
use iced::time::every;
use iced::widget::{Space, button, column, container, row, scrollable, text, text_input};
use iced::{Color, Element, Length, Padding, Subscription, Task};
use std::time::Duration;

const TABLE_ROW_HEIGHT: f32 = 30.0;
const TABLE_VIEWPORT_HEIGHT: f32 = 240.0;
const ADVANCE_INTERVAL_SECS: u64 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    Button,
    Input,
    Table,
    Dialog,
    Form,
    Select,
    Pagination,
    Card,
    Tabs,
    DatePicker,
    Cascader,
    Upload,
}

impl Category {
    const ALL: [Category; 12] = [
        Category::Button,
        Category::Input,
        Category::Table,
        Category::Dialog,
        Category::Form,
        Category::Select,
        Category::Pagination,
        Category::Card,
        Category::Tabs,
        Category::DatePicker,
        Category::Cascader,
        Category::Upload,
    ];

    fn label(self) -> &'static str {
        match self {
            Category::Button => "Button",
            Category::Input => "Input",
            Category::Table => "Table",
            Category::Dialog => "Dialog",
            Category::Form => "Form",
            Category::Select => "Select",
            Category::Pagination => "Pagination",
            Category::Card => "Card",
            Category::Tabs => "Tabs",
            Category::DatePicker => "DatePicker",
            Category::Cascader => "Cascader",
            Category::Upload => "Upload",
        }
    }

    fn next(self) -> Self {
        let idx = Self::ALL.iter().position(|c| *c == self).unwrap();
        Self::ALL[(idx + 1) % Self::ALL.len()]
    }
}

pub struct State {
    theme: Theme,
    category: Category,
    recording: bool,
    save_count: u32,
    type_buttons: Vec<Button>,
    text_input: Input,
    table: Table<TableRow>,
    dialog: Dialog,
    form: Form,
    select: Select,
    pagination: Pagination,
    card: Card,
    tabs: Tabs,
    picker: DatePicker,
    cascader: Cascader,
    upload: Upload,
}

#[derive(Debug, Clone)]
pub enum Message {
    NextComponent,
    ToggleTheme,
    ToggleRecording,
    SaveScreenshot,
    AutoAdvance,
    Exit,
    Noop,
    InputChanged(String),
    OpenDialog,
    CloseDialog,
    SubmitForm,
    ResetForm,
    FormMsg(FormMessage),
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::NextComponent => {
            state.category = state.category.next();
            Task::none()
        }
        Message::ToggleTheme => {
            state.theme = next_theme(&state.theme);
            Task::none()
        }
        Message::ToggleRecording => {
            state.recording = !state.recording;
            Task::none()
        }
        Message::SaveScreenshot => {
            state.save_count += 1;
            println!(
                "[screenshot] {}_{} (count: {}) — capture window with OS screenshot tool",
                state.category.label(),
                theme_label(&state.theme),
                state.save_count
            );
            Task::none()
        }
        Message::AutoAdvance => {
            let next = state.category.next();
            if next == Category::ALL[0] {
                state.theme = next_theme(&state.theme);
            }
            state.category = next;
            Task::none()
        }
        Message::Exit => iced::exit(),
        Message::Noop => Task::none(),
        Message::InputChanged(s) => {
            state.text_input.handle(InputMessage::Clear);
            for ch in s.chars() {
                state.text_input.handle(InputMessage::Char(ch));
            }
            Task::none()
        }
        Message::OpenDialog => {
            state.dialog.handle(DialogMessage::Open);
            Task::none()
        }
        Message::CloseDialog => {
            state.dialog.handle(DialogMessage::Close);
            Task::none()
        }
        Message::SubmitForm => {
            state.form.handle(FormMessage::Validate);
            Task::none()
        }
        Message::ResetForm => {
            state.form.handle(FormMessage::Reset);
            Task::none()
        }
        Message::FormMsg(msg) => {
            state.form.handle(msg);
            Task::none()
        }
    }
}

fn subscription(state: &State) -> Subscription<Message> {
    let mut subs: Vec<Subscription<Message>> = Vec::new();

    if state.recording {
        let timer = every(Duration::from_secs(ADVANCE_INTERVAL_SECS)).map(|_| Message::AutoAdvance);
        subs.push(timer);
    }

    // 0.14 移除 on_key_press，改用 keyboard::listen 过滤按键按下事件
    let keys = keyboard::listen().map(|event| match event {
        keyboard::Event::KeyPressed { key, .. } => match key {
            Key::Character(c) => match c.to_lowercase().as_str() {
                "s" => Message::SaveScreenshot,
                "n" => Message::NextComponent,
                "t" => Message::ToggleTheme,
                "r" => Message::ToggleRecording,
                _ => Message::Noop,
            },
            Key::Named(Named::Escape) => Message::Exit,
            _ => Message::Noop,
        },
        _ => Message::Noop,
    });
    subs.push(keys);

    Subscription::batch(subs)
}

fn view(state: &State) -> Element<'_, Message> {
    let theme = &state.theme;
    let header = build_header(theme, state.category, state.recording, state.save_count);
    let body = build_body(state);
    let footer = build_footer(theme, state.theme.is_dark);

    let layout = column![header, body, footer]
        .spacing(0)
        .width(Length::Fill)
        .height(Length::Fill);

    container(layout)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_t| container::Style {
            background: Some(iced::Background::Color(Color::from(theme.neutral.bg_page))),
            text_color: Some(Color::from(theme.neutral.text_primary)),
            ..container::Style::default()
        })
        .into()
}

fn build_header(
    theme: &Theme,
    category: Category,
    recording: bool,
    save_count: u32,
) -> Element<'_, Message> {
    let title = text("HarUI Screenshot Test").size(20).color(Color::WHITE);
    let subtitle = text(format!(
        "{} | {} | saves: {} | {}",
        category.label(),
        theme_label(theme),
        save_count,
        if recording { "REC" } else { "IDLE" }
    ))
    .size(13)
    .color(Color::WHITE);
    let col = column![title, subtitle].spacing(2);
    container(col)
        .width(Length::Fill)
        .height(Length::Fixed(56.0))
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .padding(Padding::from([8u16, 16u16]))
        .style(move |_t| container::Style {
            background: Some(iced::Background::Color(Color::from(theme.primary.base))),
            text_color: Some(Color::WHITE),
            ..container::Style::default()
        })
        .into()
}

fn build_body(state: &State) -> Element<'_, Message> {
    let theme = &state.theme;
    let title = text(state.category.label())
        .size(22)
        .color(Color::from(theme.neutral.text_primary));
    let content: Element<'_, Message> = match state.category {
        Category::Button => build_button_panel(state),
        Category::Input => build_input_panel(state),
        Category::Table => build_table_panel(state),
        Category::Dialog => build_dialog_panel(state),
        Category::Form => build_form_panel(state),
        Category::Select => build_select_panel(state),
        Category::Pagination => build_pagination_panel(state),
        Category::Card => build_card_panel(state),
        Category::Tabs => build_tabs_panel(state),
        Category::DatePicker => build_date_picker_panel(state),
        Category::Cascader => build_cascader_panel(state),
        Category::Upload => build_upload_panel(state),
    };
    let col = column![title, content]
        .spacing(16)
        .padding(Padding::from(24u16));
    let scroll = scrollable(col).height(Length::Fill);
    container(scroll)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_t| container::Style {
            background: Some(iced::Background::Color(Color::from(theme.neutral.bg_page))),
            ..container::Style::default()
        })
        .into()
}

fn build_footer(theme: &Theme, is_dark: bool) -> Element<'_, Message> {
    let hint = text("[S] save  [N] next  [T] theme  [R] record  [Esc] exit")
        .size(12)
        .color(Color::from(theme.neutral.text_secondary));
    let toggle = button(if is_dark { "Light" } else { "Dark" })
        .on_press(Message::ToggleTheme)
        .padding(Padding::from([4u16, 12u16]));
    let bar = row![hint, Space::new().width(Length::Fill), toggle]
        .spacing(8)
        .align_y(iced::Alignment::Center)
        .padding(Padding::from([8u16, 16u16]));
    container(bar)
        .width(Length::Fill)
        .height(Length::Fixed(36.0))
        .style(move |_t| container::Style {
            background: Some(iced::Background::Color(Color::from(theme.neutral.bg_base))),
            border: iced::Border {
                color: Color::from(theme.neutral.border_lighter),
                width: 1.0,
                radius: iced::border::radius(0.0),
            },
            ..container::Style::default()
        })
        .into()
}

fn build_button_panel(state: &State) -> Element<'_, Message> {
    let theme = &state.theme;
    let mut type_row = iced::widget::Row::new()
        .spacing(8)
        .align_y(iced::Alignment::Center);
    for btn in &state.type_buttons {
        type_row = type_row.push(btn.view(theme, Message::Noop));
    }
    column![text("Button types").size(16), type_row]
        .spacing(12)
        .into()
}

fn build_input_panel(state: &State) -> Element<'_, Message> {
    let theme = &state.theme;
    let input_elem = state.text_input.view(theme, Message::InputChanged);
    let value_text = text(format!("Value: {}", state.text_input.value())).size(14);
    column![text("Text input").size(16), input_elem, value_text]
        .spacing(12)
        .into()
}

fn build_table_panel(state: &State) -> Element<'_, Message> {
    let theme = &state.theme;
    let (vstart, vend) = state.table.visible_range();
    let total = state.table.rows().len();
    let info = text(format!(
        "Total: {} | Visible: [{}, {})",
        total, vstart, vend
    ))
    .size(14);
    let table_elem = state.table.view(
        theme,
        |row, prop| row.get(prop).cloned().unwrap_or_default(),
        |_| Message::Noop,
    );
    let table_container = container(table_elem).height(Length::Fixed(TABLE_VIEWPORT_HEIGHT));
    column![text("Virtual scroll table").size(16), info, table_container]
        .spacing(12)
        .into()
}

fn build_dialog_panel(state: &State) -> Element<'_, Message> {
    let theme = &state.theme;
    let trigger = button("Open Dialog")
        .on_press(Message::OpenDialog)
        .padding(Padding::from([8u16, 16u16]));
    let dialog_elem = state.dialog.view(theme, Message::CloseDialog);
    column![text("Dialog").size(16), trigger, dialog_elem]
        .spacing(12)
        .into()
}

fn build_form_panel(state: &State) -> Element<'_, Message> {
    let theme = &state.theme;
    let form = &state.form;
    let form_view = form.view(theme, move |item| {
        let field = item.field();
        let val = form.value_of(field).map(|s| s.as_str()).unwrap_or("");
        let field_owned = field.to_string();
        let placeholder = match field {
            "username" => "Enter username",
            "password" => "Enter password",
            _ => "",
        };
        let secure = field == "password";
        let ti = text_input(placeholder, val)
            .secure(secure)
            .on_input(move |s| Message::FormMsg(FormMessage::SetValue(field_owned.clone(), s)));
        ti.into()
    });
    let submit_btn = button("Submit")
        .on_press(Message::SubmitForm)
        .padding(Padding::from([8u16, 16u16]));
    let reset_btn = button("Reset")
        .on_press(Message::ResetForm)
        .padding(Padding::from([8u16, 16u16]));
    let actions = row![submit_btn, reset_btn].spacing(8);
    let state_label = match state.form.state() {
        FormState::Idle => "Idle",
        FormState::Validating => "Validating",
        FormState::Passed => "Passed",
        FormState::Failed => "Failed",
    };
    column![
        text("Form").size(16),
        form_view,
        actions,
        text(format!("State: {}", state_label)).size(14)
    ]
    .spacing(12)
    .into()
}

fn build_select_panel(state: &State) -> Element<'_, Message> {
    let theme = &state.theme;
    let select_elem = state.select.view(theme, |_| Message::Noop);
    let current = state
        .select
        .display_label()
        .unwrap_or_else(|| "(none)".to_string());
    column![
        text("Select").size(16),
        select_elem,
        text(format!("Current: {}", current)).size(14)
    ]
    .spacing(12)
    .into()
}

fn build_pagination_panel(state: &State) -> Element<'_, Message> {
    let theme = &state.theme;
    let pagination_elem = state.pagination.view(theme, |_| Message::Noop);
    let info = text(format!(
        "Page {} of {}",
        state.pagination.current_page(),
        state.pagination.total_pages()
    ))
    .size(14);
    column![text("Pagination").size(16), info, pagination_elem]
        .spacing(12)
        .into()
}

fn build_card_panel(state: &State) -> Element<'_, Message> {
    let theme = &state.theme;
    let card_elem = state.card.view(theme).map(|_| Message::Noop);
    column![text("Card").size(16), card_elem].spacing(12).into()
}

fn build_tabs_panel(state: &State) -> Element<'_, Message> {
    let theme = &state.theme;
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
        .unwrap_or("(none)");
    let tabs_view = state.tabs.view(theme, |_| Message::Noop);
    column![
        text("Tabs").size(16),
        text(format!("Active: {}", active_label)).size(14),
        tabs_view
    ]
    .spacing(12)
    .into()
}

fn build_date_picker_panel(state: &State) -> Element<'_, Message> {
    let selected = state.picker.formatted_value();
    let display = if selected.is_empty() {
        "(none)".to_string()
    } else {
        format!("Selected: {}", selected)
    };
    let picker_view = state.picker.view(&state.theme, |_| Message::Noop);
    column![
        text("DatePicker").size(16),
        text(display).size(14),
        picker_view
    ]
    .spacing(12)
    .into()
}

fn build_cascader_panel(state: &State) -> Element<'_, Message> {
    let path = state.cascader.selected_path();
    let path_display = if path.is_empty() {
        "(none)".to_string()
    } else {
        let mut labels: Vec<String> = Vec::new();
        let mut current: &[CascaderNode] = state.cascader.options();
        for value in path {
            if let Some(node) = current.iter().find(|n| n.value() == value.as_str()) {
                labels.push(node.label().to_string());
                current = node.children();
            } else {
                break;
            }
        }
        labels.join(" / ")
    };
    let cascader_view = state.cascader.view(&state.theme, |_| Message::Noop);
    column![
        text("Cascader").size(16),
        text(format!("Path: {}", path_display)).size(14),
        cascader_view
    ]
    .spacing(12)
    .into()
}

fn build_upload_panel(state: &State) -> Element<'_, Message> {
    let count = state.upload.file_list().len();
    let upload_view = state.upload.view(&state.theme, || Message::Noop);
    column![
        text("Upload").size(16),
        text(format!("Files: {}", count)).size(14),
        upload_view
    ]
    .spacing(12)
    .into()
}

fn build_cascader_options() -> Vec<CascaderNode> {
    vec![
        CascaderNode::new("guangdong", "Guangdong").with_children(vec![
            CascaderNode::new("guangzhou", "Guangzhou").with_children(vec![
                CascaderNode::new("tianhe", "Tianhe"),
                CascaderNode::new("yuexiu", "Yuexiu"),
            ]),
            CascaderNode::new("shenzhen", "Shenzhen").with_children(vec![
                CascaderNode::new("nanshan", "Nanshan"),
                CascaderNode::new("futian", "Futian"),
            ]),
        ]),
        CascaderNode::new("zhejiang", "Zhejiang").with_children(vec![
            CascaderNode::new("hangzhou", "Hangzhou").with_children(vec![
                CascaderNode::new("xihu", "Xihu"),
                CascaderNode::new("binjiang", "Binjiang"),
            ]),
        ]),
    ]
}

fn build_table() -> Table<TableRow> {
    let columns = vec![
        TableColumn::new("id", "ID").with_width(80.0),
        TableColumn::new("name", "Name").with_width(200.0),
        TableColumn::new("value", "Value").with_width(120.0),
    ];
    let rows: Vec<TableRow> = (0..200)
        .map(|i| {
            let mut row = TableRow::new();
            row.insert("id".to_string(), i.to_string());
            row.insert("name".to_string(), format!("Item {}", i));
            row.insert("value".to_string(), format!("{}", i * 10));
            row
        })
        .collect();
    Table::new()
        .with_columns(columns)
        .with_rows(rows)
        .with_props(TableProps::new().with_stripe(true).with_border(true))
        .with_virtual_scroll(VirtualScroll::new(TABLE_ROW_HEIGHT, TABLE_VIEWPORT_HEIGHT))
}

fn next_theme(theme: &Theme) -> Theme {
    if theme.is_dark {
        Theme::element_light()
    } else {
        Theme::element_dark()
    }
}

fn theme_label(theme: &Theme) -> &'static str {
    if theme.is_dark { "dark" } else { "light" }
}

fn main() -> iced::Result {
    iced::application(
        || {
            let type_buttons = vec![
                Button::new("Default"),
                Button::new("Primary").with_type(ButtonType::Primary),
                Button::new("Success").with_type(ButtonType::Success),
                Button::new("Warning").with_type(ButtonType::Warning),
                Button::new("Danger").with_type(ButtonType::Danger),
                Button::new("Info").with_type(ButtonType::Info),
            ];
            let text_input = Input::new()
                .with_placeholder("Enter text")
                .with_clearable(true);
            let table = build_table();
            let dialog = Dialog::new("Confirm Action", "Are you sure you want to proceed?");
            let form = Form::new()
                .with_label_width(100)
                .with_item(
                    FormItem::new("username", "Username").with_rule(
                        FormRule::new("username")
                            .required(true)
                            .with_message("Username is required"),
                    ),
                )
                .with_item(
                    FormItem::new("password", "Password").with_rule(
                        FormRule::new("password")
                            .required(true)
                            .with_message("Password is required"),
                    ),
                );
            let select = Select::new()
                .with_clearable(true)
                .with_filterable(true)
                .with_option(SelectOption::new("beijing", "Beijing"))
                .with_option(SelectOption::new("shanghai", "Shanghai"))
                .with_option(SelectOption::new("guangzhou", "Guangzhou"))
                .with_option(SelectOption::new("shenzhen", "Shenzhen"))
                .with_option(SelectOption::new("hangzhou", "Hangzhou"));
            let pagination = Pagination::new(100, 10);
            let card = Card::new("Screenshot card body content.")
                .with_header("Card Header")
                .with_footer("Card Footer");
            let tabs = Tabs::new()
                .with_item(TabItem::new("user", "User"))
                .with_item(TabItem::new("role", "Role"))
                .with_item(TabItem::new("perm", "Permission"));
            let picker = DatePicker::new();
            let cascader = Cascader::new().with_options(build_cascader_options());
            let upload = Upload::new().with_multiple(true).with_limit(5);
            let state = State {
                theme: Theme::element_light(),
                category: Category::Button,
                recording: false,
                save_count: 0,
                type_buttons,
                text_input,
                table,
                dialog,
                form,
                select,
                pagination,
                card,
                tabs,
                picker,
                cascader,
                upload,
            };
            (state, Task::none())
        },
        update,
        view,
    )
    .title(|_state: &State| String::from("HarUI Screenshot Test"))
    .window_size(iced::Size::new(1024.0, 768.0))
    .subscription(subscription)
    .run()
}
