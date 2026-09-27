//! MessageBox 消息框组件 — 参考 Element Plus `ElMessageBox`。
//! 支持：alert/confirm/prompt 三种模式、4 种类型、按钮配置、close_on_click_modal、center、prompt 输入。

use har_ui_core::theme::Theme;
use iced::widget::{button, container, text};
use iced::{Color, Element, Length, Padding};

/// MessageBox 类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MessageBoxType {
    Success,
    Warning,
    #[default]
    Info,
    Error,
}

impl MessageBoxType {
    pub fn as_str(self) -> &'static str {
        match self {
            MessageBoxType::Success => "success",
            MessageBoxType::Warning => "warning",
            MessageBoxType::Info => "info",
            MessageBoxType::Error => "error",
        }
    }
}

/// MessageBox 用户动作
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageBoxAction {
    Confirm,
    Cancel,
    Close,
}

/// MessageBox 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageBoxMessage {
    /// 打开对话框
    Open,
    /// 关闭对话框（点击 X 或 Esc）
    Close,
    /// 点击确认
    Confirm,
    /// 点击取消
    Cancel,
    /// 点击遮罩层
    ClickModal,
    /// prompt 模式输入值
    Input(String),
}

/// MessageBox 组件
#[derive(Debug, Clone)]
pub struct MessageBox {
    /// 标题
    title: String,
    /// 消息内容
    message: String,
    /// 类型
    msg_type: MessageBoxType,
    /// 是否可见
    visible: bool,
    /// 是否显示确认按钮
    show_confirm: bool,
    /// 是否显示取消按钮
    show_cancel: bool,
    /// 是否显示关闭按钮
    show_close: bool,
    /// 是否居中显示
    center: bool,
    /// 是否点击遮罩关闭
    close_on_click_modal: bool,
    /// 确认按钮文本
    confirm_text: String,
    /// 取消按钮文本
    cancel_text: String,
    /// 是否为 prompt 模式
    is_prompt: bool,
    /// prompt 输入值
    input_value: String,
    /// 上次用户动作（用于回调判断）
    last_action: Option<MessageBoxAction>,
    /// 确认时携带的输入值（仅 prompt 模式有效）
    confirmed_input: Option<String>,
}

impl MessageBox {
    pub fn new(title: &str, message: &str) -> Self {
        Self {
            title: title.to_string(),
            message: message.to_string(),
            msg_type: MessageBoxType::Info,
            visible: false,
            show_confirm: true,
            show_cancel: false,
            show_close: true,
            center: false,
            close_on_click_modal: false,
            confirm_text: "确认".to_string(),
            cancel_text: "取消".to_string(),
            is_prompt: false,
            input_value: String::new(),
            last_action: None,
            confirmed_input: None,
        }
    }

    /// 创建 prompt 模式对话框
    pub fn prompt(title: &str, message: &str) -> Self {
        let mut mb = Self::new(title, message);
        mb.is_prompt = true;
        mb.show_cancel = true;
        mb
    }

    // ---------- Builder ----------

    pub fn with_type(mut self, t: MessageBoxType) -> Self {
        self.msg_type = t;
        self
    }

    pub fn with_show_confirm(mut self, s: bool) -> Self {
        self.show_confirm = s;
        self
    }

    pub fn with_show_cancel(mut self, s: bool) -> Self {
        self.show_cancel = s;
        self
    }

    pub fn with_show_close(mut self, s: bool) -> Self {
        self.show_close = s;
        self
    }

    pub fn with_center(mut self, c: bool) -> Self {
        self.center = c;
        self
    }

    pub fn with_close_on_click_modal(mut self, c: bool) -> Self {
        self.close_on_click_modal = c;
        self
    }

    pub fn with_confirm_text(mut self, t: &str) -> Self {
        self.confirm_text = t.to_string();
        self
    }

    pub fn with_cancel_text(mut self, t: &str) -> Self {
        self.cancel_text = t.to_string();
        self
    }

    // ---------- Getter ----------

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn msg_type(&self) -> MessageBoxType {
        self.msg_type
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn show_confirm_button(&self) -> bool {
        self.show_confirm
    }

    pub fn show_cancel_button(&self) -> bool {
        self.show_cancel
    }

    pub fn show_close(&self) -> bool {
        self.show_close
    }

    pub fn center(&self) -> bool {
        self.center
    }

    pub fn close_on_click_modal(&self) -> bool {
        self.close_on_click_modal
    }

    pub fn confirm_button_text(&self) -> &str {
        &self.confirm_text
    }

    pub fn cancel_button_text(&self) -> &str {
        &self.cancel_text
    }

    pub fn is_prompt(&self) -> bool {
        self.is_prompt
    }

    pub fn input_value(&self) -> &str {
        &self.input_value
    }

    pub fn last_action(&self) -> Option<MessageBoxAction> {
        self.last_action
    }

    /// 确认动作携带的输入值（仅 prompt 模式 + Confirm 后有效）
    pub fn confirmed_input(&self) -> Option<&str> {
        self.confirmed_input.as_deref()
    }

    // ---------- 消息处理 ----------

    pub fn handle(&mut self, msg: MessageBoxMessage) {
        match msg {
            MessageBoxMessage::Open => {
                self.visible = true;
                self.last_action = None;
                // 重新打开时清空 prompt 输入
                if self.is_prompt {
                    self.input_value.clear();
                    self.confirmed_input = None;
                }
            }
            MessageBoxMessage::Close => {
                self.visible = false;
                self.last_action = Some(MessageBoxAction::Close);
            }
            MessageBoxMessage::Confirm => {
                if self.is_prompt {
                    self.confirmed_input = Some(self.input_value.clone());
                }
                self.visible = false;
                self.last_action = Some(MessageBoxAction::Confirm);
            }
            MessageBoxMessage::Cancel => {
                self.confirmed_input = None;
                self.visible = false;
                self.last_action = Some(MessageBoxAction::Cancel);
            }
            MessageBoxMessage::ClickModal => {
                if self.close_on_click_modal {
                    self.visible = false;
                    self.last_action = Some(MessageBoxAction::Close);
                }
            }
            MessageBoxMessage::Input(v) => {
                if self.is_prompt {
                    self.input_value = v;
                }
            }
        }
    }

    /// 渲染 MessageBox 为 iced::Element
    ///
    /// - `visible == false` 时返回空容器
    /// - 否则渲染遮罩 + 标题 + 内容 + 按钮组（按 show_confirm/show_close/show_cancel）
    /// - on_action 参数为按钮 key：`"confirm"` / `"cancel"` / `"close"`
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        on_action: impl Fn(String) -> Message + 'a,
    ) -> Element<'a, Message> {
        if !self.visible {
            return container(text("")).into();
        }

        let title_color = Color::from(theme.neutral.text_primary);
        let content_color = Color::from(theme.neutral.text_regular);
        let bg = Color::from(theme.neutral.bg_overlay);
        let border_color = Color::from(theme.neutral.border_lighter);
        let primary = Color::from(theme.primary.base);
        let mask = Color {
            a: 0.5,
            ..Color::BLACK
        };

        // 标题栏
        let title_text = text(self.title.clone()).color(title_color).size(16.0);
        let mut header_row_children: Vec<Element<'a, Message>> = Vec::new();
        header_row_children.push(title_text.into());
        header_row_children.push(iced::widget::Space::with_width(Length::Fill).into());
        if self.show_close {
            let close_msg = on_action("close".to_string());
            let close_btn = button(text("×").color(content_color).size(16.0))
                .padding(Padding::from([2u16, 8u16]))
                .on_press(close_msg)
                .style(move |_t, _status| iced::widget::button::Style {
                    background: None,
                    text_color: content_color,
                    border: iced::Border::default(),
                    shadow: iced::Shadow::default(),
                });
            header_row_children.push(close_btn.into());
        }
        let header = iced::widget::Row::with_children(header_row_children)
            .align_y(iced::Alignment::Center)
            .spacing(0);

        // 内容
        let content_text = text(self.message.clone()).color(content_color).size(14.0);

        let mut body_children: Vec<Element<'a, Message>> = Vec::new();
        body_children.push(content_text.into());
        if self.is_prompt {
            // prompt 输入框（只读展示当前输入值）
            let input_box = container(
                text(self.input_value.clone())
                    .color(content_color)
                    .size(14.0),
            )
            .width(Length::Fill)
            .padding(Padding::from([8u16, 12u16]))
            .style(move |_t| iced::widget::container::Style {
                text_color: None,
                background: Some(iced::Background::Color(Color::from(theme.neutral.bg_base))),
                border: iced::Border {
                    color: border_color,
                    width: 1.0,
                    radius: iced::border::radius(4.0),
                },
                shadow: iced::Shadow::default(),
            });
            body_children.push(iced::widget::Space::with_height(Length::Fixed(8.0)).into());
            body_children.push(input_box.into());
        }
        let body = iced::widget::Column::with_children(body_children).spacing(0);

        // 按钮组
        let mut btn_row_children: Vec<Element<'a, Message>> = Vec::new();
        btn_row_children.push(iced::widget::Space::with_width(Length::Fill).into());
        if self.show_cancel {
            let cancel_msg = on_action("cancel".to_string());
            let cancel_btn = button(
                text(self.cancel_text.clone())
                    .color(content_color)
                    .size(14.0),
            )
            .padding(Padding::from([8u16, 16u16]))
            .on_press(cancel_msg)
            .style(move |_t, _status| iced::widget::button::Style {
                background: Some(iced::Background::Color(Color::from(theme.neutral.bg_base))),
                text_color: content_color,
                border: iced::Border {
                    color: border_color,
                    width: 1.0,
                    radius: iced::border::radius(4.0),
                },
                shadow: iced::Shadow::default(),
            });
            btn_row_children.push(cancel_btn.into());
            btn_row_children.push(iced::widget::Space::with_width(Length::Fixed(8.0)).into());
        }
        if self.show_confirm {
            let confirm_msg = on_action("confirm".to_string());
            let confirm_btn = button(
                text(self.confirm_text.clone())
                    .color(Color::WHITE)
                    .size(14.0),
            )
            .padding(Padding::from([8u16, 16u16]))
            .on_press(confirm_msg)
            .style(move |_t, _status| iced::widget::button::Style {
                background: Some(iced::Background::Color(primary)),
                text_color: Color::WHITE,
                border: iced::Border {
                    color: primary,
                    width: 1.0,
                    radius: iced::border::radius(4.0),
                },
                shadow: iced::Shadow::default(),
            });
            btn_row_children.push(confirm_btn.into());
        }
        let btn_row = iced::widget::Row::with_children(btn_row_children)
            .align_y(iced::Alignment::Center)
            .spacing(0);

        // 对话框主体
        let dialog_inner = iced::widget::Column::new()
            .push(header)
            .push(iced::widget::Space::with_height(Length::Fixed(12.0)))
            .push(body)
            .push(iced::widget::Space::with_height(Length::Fixed(16.0)))
            .push(btn_row);
        let dialog_box = container(dialog_inner)
            .max_width(420.0)
            .padding(Padding::from(16u16))
            .style(move |_t| iced::widget::container::Style {
                text_color: Some(title_color),
                background: Some(iced::Background::Color(bg)),
                border: iced::Border {
                    color: border_color,
                    width: 1.0,
                    radius: iced::border::radius(4.0),
                },
                shadow: iced::Shadow {
                    color: Color {
                        a: 0.3,
                        ..Color::BLACK
                    },
                    offset: iced::Vector::new(0.0, 4.0),
                    blur_radius: 16.0,
                },
            });

        let centered = container(dialog_box)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill);

        container(centered)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_t| iced::widget::container::Style {
                text_color: None,
                background: Some(iced::Background::Color(mask)),
                border: iced::Border::default(),
                shadow: iced::Shadow::default(),
            })
            .into()
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_mb_internal_default() {
        let mb = MessageBox::new("t", "m");
        assert_eq!(mb.title, "t");
        assert_eq!(mb.message, "m");
        assert_eq!(mb.msg_type, MessageBoxType::Info);
        assert!(mb.show_confirm);
        assert!(!mb.show_cancel);
        assert!(!mb.is_prompt);
    }

    #[test]
    fn test_mb_internal_prompt_constructor() {
        let mb = MessageBox::prompt("t", "m");
        assert!(mb.is_prompt);
        assert!(mb.show_cancel);
    }

    #[test]
    fn test_mb_internal_open_resets_state() {
        let mut mb = MessageBox::prompt("t", "m");
        mb.handle(MessageBoxMessage::Open);
        mb.handle(MessageBoxMessage::Input("测试".to_string()));
        mb.handle(MessageBoxMessage::Cancel);
        mb.handle(MessageBoxMessage::Open);
        assert!(mb.input_value.is_empty());
        assert!(mb.confirmed_input.is_none());
        assert!(mb.last_action.is_none());
    }
}
