//! HarUI Showcase — comprehensive component gallery

use har_ui_components::button::{Button, ButtonType};
use har_ui_components::card::Card;
use har_ui_components::cascader::{Cascader, CascaderMessage, CascaderNode};
use har_ui_components::date_picker::{DatePicker, DatePickerMessage, SimpleDate};
use har_ui_components::dialog::{Dialog, DialogMessage};
use har_ui_components::form::{Form, FormItem, FormMessage, FormRule, FormState};
use har_ui_components::input::{Input, InputMessage};
use har_ui_components::pagination::{Pagination, PaginationMessage};
use har_ui_components::select::{Select, SelectMessage, SelectOption};
use har_ui_components::table::{
    Table, TableColumn, TableMessage, TableProps, TableRow, VirtualScroll,
};
use har_ui_components::tabs::{TabItem, Tabs, TabsMessage};
use har_ui_components::upload::{Upload, UploadFile, UploadMessage};
use har_ui_core::Theme;
use iced::widget::{Space, button, column, container, row, scrollable, text, text_input};
use iced::{Color, Element, Length, Padding, Task};

const TABLE_ROW_HEIGHT: f32 = 30.0;
const TABLE_VIEWPORT_HEIGHT: f32 = 240.0;

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
}

pub struct State {
    theme: Theme,
    category: Category,
    type_buttons: Vec<Button>,
    click_count: u32,
    text_input: Input,
    table: Table<TableRow>,
    dialog: Dialog,
    dialog_open_count: u32,
    form: Form,
    select: Select,
    pagination: Pagination,
    card: Card,
    tabs: Tabs,
    picker: DatePicker,
    cascader: Cascader,
    upload: Upload,
    upload_count: u32,
}

#[derive(Debug, Clone)]
pub enum Message {
    SelectCategory(Category),
    ToggleTheme,
    Noop,
    ButtonClicked,
    InputChanged(String),
    TableMsg(TableMessage),
    OpenDialog,
    CloseDialog,
    SubmitForm,
    ResetForm,
    FormMsg(FormMessage),
    SelectChoose(String),
    PageJump(i64),
    TabSelected(String),
    DatePick(String),
    CascaderPath(Vec<String>),
    UploadTrigger,
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::SelectCategory(c) => {
            state.category = c;
            Task::none()
        }
        Message::ToggleTheme => {
            state.theme = if state.theme.is_dark {
                Theme::element_light()
            } else {
                Theme::element_dark()
            };
            Task::none()
        }
        Message::Noop => Task::none(),
        Message::ButtonClicked => {
            state.click_count += 1;
            Task::none()
        }
        Message::InputChanged(s) => {
            state.text_input.handle(InputMessage::Clear);
            for ch in s.chars() {
                state.text_input.handle(InputMessage::Char(ch));
            }
            Task::none()
        }
        Message::TableMsg(msg) => {
            state.table.handle(msg);
            Task::none()
        }
        Message::OpenDialog => {
            state.dialog.handle(DialogMessage::Open);
            state.dialog_open_count += 1;
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
        Message::SelectChoose(s) => {
            match s.as_str() {
                "__trigger__" => state.select.handle(SelectMessage::Open),
                "__clear__" => state.select.handle(SelectMessage::Clear),
                s if s.starts_with("__query:") => {
                    let q = s.strip_prefix("__query:").unwrap_or("");
                    state.select.handle(SelectMessage::Query(q.to_string()));
                }
                other => state
                    .select
                    .handle(SelectMessage::Choose(other.to_string())),
            }
            Task::none()
        }
        Message::PageJump(p) => {
            match p {
                -1 => state.pagination.handle(PaginationMessage::Prev),
                -2 => state.pagination.handle(PaginationMessage::Next),
                other => state.pagination.handle(PaginationMessage::JumpTo(other)),
            }
            Task::none()
        }
        Message::TabSelected(id) => {
            state.tabs.handle(TabsMessage::Select(id));
            Task::none()
        }
        Message::DatePick(s) => {
            let msg = match s.as_str() {
                "__toggle__" => {
                    if state.picker.visible() {
                        DatePickerMessage::Close
                    } else {
                        DatePickerMessage::Open
                    }
                }
                "__prev_month__" => DatePickerMessage::PrevMonth,
                "__next_month__" => DatePickerMessage::NextMonth,
                "__prev_year__" => DatePickerMessage::PrevYear,
                "__next_year__" => DatePickerMessage::NextYear,
                _ => {
                    let parts: Vec<&str> = s.split('-').collect();
                    if parts.len() != 3 {
                        return Task::none();
                    }
                    let year = match parts[0].parse::<i32>() {
                        Ok(v) => v,
                        Err(_) => return Task::none(),
                    };
                    let month = match parts[1].parse::<u32>() {
                        Ok(v) => v,
                        Err(_) => return Task::none(),
                    };
                    let day = match parts[2].parse::<u32>() {
                        Ok(v) => v,
                        Err(_) => return Task::none(),
                    };
                    DatePickerMessage::SelectDate(SimpleDate::new(year, month, day))
                }
            };
            state.picker.handle(msg);
            Task::none()
        }
        Message::CascaderPath(path) => {
            if path.as_slice() == ["__toggle__"] {
                state.cascader.handle(CascaderMessage::TogglePanel);
            } else if let Some(last) = path.last() {
                state.cascader.handle(CascaderMessage::Select(last.clone()));
            }
            Task::none()
        }
        Message::UploadTrigger => {
            state.upload_count += 1;
            let file = UploadFile::new(
                format!("showcase_file_{}.txt", state.upload_count),
                1024 * (state.upload_count as u64),
            );
            state.upload.handle(UploadMessage::AddFile(file));
            Task::none()
        }
    }
}

fn view(state: &State) -> Element<'_, Message> {
    let theme = &state.theme;
    let top_bar = build_top_bar(theme);
    let bottom_bar = build_bottom_bar(theme, state.category, state.theme.is_dark);
    let nav = build_nav(theme, state.category);
    let content = build_content(state);
    let middle = row![nav, content]
        .spacing(0)
        .width(Length::Fill)
        .height(Length::Fill);
    let layout = column![top_bar, middle, bottom_bar]
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

fn build_top_bar(theme: &Theme) -> Element<'_, Message> {
    let title = text("HarUI Showcase").size(20).color(Color::WHITE);
    container(title)
        .width(Length::Fill)
        .height(Length::Fixed(48.0))
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(move |_t| container::Style {
            background: Some(iced::Background::Color(Color::from(theme.primary.base))),
            text_color: Some(Color::WHITE),
            ..container::Style::default()
        })
        .into()
}

fn build_bottom_bar(theme: &Theme, category: Category, is_dark: bool) -> Element<'_, Message> {
    let status = text(format!("Category: {}", category.label()))
        .size(14)
        .color(Color::from(theme.neutral.text_regular));
    let toggle_label = if is_dark { "Light Theme" } else { "Dark Theme" };
    let toggle_btn = button(toggle_label)
        .on_press(Message::ToggleTheme)
        .padding(Padding::from([4u16, 12u16]));
    let bar = row![status, Space::with_width(Length::Fill), toggle_btn]
        .spacing(8)
        .align_y(iced::Alignment::Center)
        .padding(Padding::from([8u16, 16u16]));
    container(bar)
        .width(Length::Fill)
        .height(Length::Fixed(40.0))
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

fn build_nav(theme: &Theme, current: Category) -> Element<'_, Message> {
    let mut col = iced::widget::Column::new()
        .spacing(2)
        .padding(Padding::from([8u16, 4u16]));
    for cat in Category::ALL.iter() {
        let label = cat.label();
        let selected = *cat == current;
        let nav_btn = button(text(label).size(14))
            .on_press(Message::SelectCategory(*cat))
            .padding(Padding::from([6u16, 12u16]))
            .style(move |_t, _status| button::Style {
                background: if selected {
                    Some(iced::Background::Color(Color::from(theme.primary.base)))
                } else {
                    None
                },
                text_color: if selected {
                    Color::WHITE
                } else {
                    Color::from(theme.neutral.text_regular)
                },
                border: iced::Border::default(),
                shadow: iced::Shadow::default(),
            });
        col = col.push(nav_btn);
    }
    let scroll = scrollable(col).height(Length::Fill);
    container(scroll)
        .width(Length::Fixed(160.0))
        .height(Length::Fill)
        .style(move |_t| container::Style {
            background: Some(iced::Background::Color(Color::from(
                theme.neutral.bg_overlay,
            ))),
            border: iced::Border {
                color: Color::from(theme.neutral.border_lighter),
                width: 1.0,
                radius: iced::border::radius(0.0),
            },
            ..container::Style::default()
        })
        .into()
}

fn build_content(state: &State) -> Element<'_, Message> {
    let theme = &state.theme;
    let title = text(state.category.label())
        .size(22)
        .color(Color::from(theme.neutral.text_primary));
    let body: Element<'_, Message> = match state.category {
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
    let col = column![title, body]
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

fn build_button_panel(state: &State) -> Element<'_, Message> {
    let theme = &state.theme;
    let mut type_row = iced::widget::Row::new()
        .spacing(8)
        .align_y(iced::Alignment::Center);
    for btn in &state.type_buttons {
        type_row = type_row.push(btn.view(theme, Message::ButtonClicked));
    }
    let info = text(format!("Click count: {}", state.click_count)).size(14);
    column![text("Button types").size(16), type_row, info]
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
        |idx| Message::TableMsg(TableMessage::RowClicked(idx)),
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
    let status = text(format!("Open count: {}", state.dialog_open_count)).size(14);
    let dialog_elem = state.dialog.view(theme, Message::CloseDialog);
    column![text("Dialog").size(16), trigger, status, dialog_elem]
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
    let select_elem = state.select.view(theme, Message::SelectChoose);
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
    let pagination_elem = state.pagination.view(theme, Message::PageJump);
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
    let tabs_view = state.tabs.view(theme, Message::TabSelected);
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
    let picker_view = state.picker.view(&state.theme, Message::DatePick);
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
    let cascader_view = state.cascader.view(&state.theme, Message::CascaderPath);
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
    let upload_view = state.upload.view(&state.theme, || Message::UploadTrigger);
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

fn main() -> iced::Result {
    iced::application("HarUI Showcase", update, view)
        .window_size(iced::Size::new(1024.0, 700.0))
        .run_with(|| {
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
            let card = Card::new("Showcase card body content.")
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
                type_buttons,
                click_count: 0,
                text_input,
                table,
                dialog,
                dialog_open_count: 0,
                form,
                select,
                pagination,
                card,
                tabs,
                picker,
                cascader,
                upload,
                upload_count: 0,
            };
            (state, Task::none())
        })
}
