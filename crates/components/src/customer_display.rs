//! CustomerDisplay 客显屏组件 — POS 专用
//!
//! 第二屏（客显屏）独立渲染：金额显示、收款方式、二维码、自定义信息。
//! 状态机：Idle / Welcome / ShowingAmount / ShowingQrCode / ShowingMessage / Success

use har_ui_core::theme::Theme;
use iced::widget::{container, text};
use iced::{Color, Element, Length, Padding};

/// 客显屏状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CustomerDisplayState {
    #[default]
    Idle,
    Welcome,
    ShowingAmount,
    ShowingQrCode,
    ShowingMessage,
    Success,
}

/// 客显屏消息
#[derive(Debug, Clone, PartialEq)]
pub enum CustomerDisplayMessage {
    ShowAmount(f64),
    ShowQrCode(String, String), // (二维码内容, 支付方式名称)
    ShowCustomMessage(String),
    ShowSuccess,
    ShowWelcome,
    Reset,
}

/// CustomerDisplay 组件
#[derive(Debug, Clone, Default)]
pub struct CustomerDisplay {
    amount: f64,
    payment_method: Option<String>,
    qr_code: Option<String>,
    custom_message: Option<String>,
    state: CustomerDisplayState,
}

impl CustomerDisplay {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }

    pub fn payment_method(&self) -> Option<&String> {
        self.payment_method.as_ref()
    }

    pub fn qr_code(&self) -> Option<&String> {
        self.qr_code.as_ref()
    }

    pub fn custom_message(&self) -> Option<&String> {
        self.custom_message.as_ref()
    }

    pub fn state(&self) -> CustomerDisplayState {
        self.state
    }

    /// 格式化金额为 "1234.56" 字符串
    pub fn formatted_amount(&self) -> String {
        format!("{:.2}", self.amount)
    }

    /// 处理消息
    pub fn handle(&mut self, msg: CustomerDisplayMessage) {
        match msg {
            CustomerDisplayMessage::ShowAmount(v) => {
                self.amount = v;
                self.custom_message = None;
                self.qr_code = None;
                self.payment_method = None;
                self.state = CustomerDisplayState::ShowingAmount;
            }
            CustomerDisplayMessage::ShowQrCode(qr, method) => {
                self.qr_code = Some(qr);
                self.payment_method = Some(method);
                self.custom_message = None;
                self.state = CustomerDisplayState::ShowingQrCode;
            }
            CustomerDisplayMessage::ShowCustomMessage(msg) => {
                self.custom_message = Some(msg);
                self.state = CustomerDisplayState::ShowingMessage;
            }
            CustomerDisplayMessage::ShowSuccess => {
                self.qr_code = None;
                self.state = CustomerDisplayState::Success;
            }
            CustomerDisplayMessage::ShowWelcome => {
                self.amount = 0.0;
                self.payment_method = None;
                self.qr_code = None;
                self.custom_message = None;
                self.state = CustomerDisplayState::Welcome;
            }
            CustomerDisplayMessage::Reset => {
                self.amount = 0.0;
                self.payment_method = None;
                self.qr_code = None;
                self.custom_message = None;
                self.state = CustomerDisplayState::Idle;
            }
        }
    }

    /// 渲染 CustomerDisplay 为 iced::Element
    ///
    /// # 参数
    /// - `theme`: HarUI 主题引用
    pub fn view<'a>(&'a self, theme: &'a Theme) -> Element<'a, ()> {
        let primary = Color::from(theme.primary.base);
        let success = Color::from(theme.success.base);
        let warning = Color::from(theme.warning.base);
        let text_primary = Color::from(theme.neutral.text_primary);
        let text_secondary = Color::from(theme.neutral.text_secondary);
        let text_placeholder = Color::from(theme.neutral.text_placeholder);
        let border_lighter = Color::from(theme.neutral.border_lighter);

        let (state_label, state_color) = match self.state {
            CustomerDisplayState::Idle => ("待机", text_secondary),
            CustomerDisplayState::Welcome => ("欢迎光临", primary),
            CustomerDisplayState::ShowingAmount => ("应收", warning),
            CustomerDisplayState::ShowingQrCode => ("扫码支付", primary),
            CustomerDisplayState::ShowingMessage => ("提示", text_secondary),
            CustomerDisplayState::Success => ("完成", success),
        };

        let mut col_children: Vec<Element<'a, ()>> = Vec::new();

        // 状态标签
        col_children.push(
            container(text(state_label.to_string()).color(state_color).size(14.0))
                .width(Length::Fill)
                .padding(Padding::from([4u16, 8u16]))
                .into(),
        );

        // 主内容区
        let body_text = match self.state {
            CustomerDisplayState::Idle => text("等待交易...".to_string())
                .color(text_placeholder)
                .size(20.0),
            CustomerDisplayState::Welcome => text("欢迎光临".to_string()).color(primary).size(32.0),
            CustomerDisplayState::ShowingAmount => text(format!("¥ {}", self.formatted_amount()))
                .color(warning)
                .size(40.0),
            CustomerDisplayState::ShowingQrCode => {
                let method = self.payment_method.clone().unwrap_or_default();
                text(format!("扫码支付 · {}", method))
                    .color(primary)
                    .size(24.0)
            }
            CustomerDisplayState::ShowingMessage => {
                let msg = self.custom_message.clone().unwrap_or_default();
                text(msg).color(text_primary).size(20.0)
            }
            CustomerDisplayState::Success => text("支付成功".to_string()).color(success).size(32.0),
        };
        let body = container(body_text)
            .width(Length::Fill)
            .padding(Padding::from([20u16, 8u16]));
        col_children.push(body.into());

        // 二维码内容（若存在）
        if let Some(qr) = &self.qr_code {
            col_children.push(
                container(text(qr.clone()).color(text_secondary).size(12.0))
                    .width(Length::Fill)
                    .padding(Padding::from([4u16, 8u16]))
                    .into(),
            );
        }

        let col = iced::widget::Column::with_children(col_children).spacing(0);
        container(col)
            .width(Length::Fill)
            .height(Length::Fixed(200.0))
            .padding(Padding::from(8u16))
            .style(move |_t| iced::widget::container::Style {
                text_color: None,
                background: Some(iced::Background::Color(Color {
                    a: 0.6,
                    ..Color::from(theme.neutral.border_extra_light)
                })),
                border: iced::Border {
                    color: border_lighter,
                    width: 1.0,
                    radius: iced::border::radius(4.0),
                },
                shadow: iced::Shadow::default(),
                snap: false,
            })
            .into()
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_default_state_idle() {
        let d = CustomerDisplay::new();
        assert_eq!(d.state(), CustomerDisplayState::Idle);
    }

    #[test]
    fn test_formatted_amount() {
        let mut d = CustomerDisplay::new();
        d.handle(CustomerDisplayMessage::ShowAmount(88.5));
        assert_eq!(d.formatted_amount(), "88.50");
    }

    #[test]
    fn test_reset_clears_all_fields() {
        let mut d = CustomerDisplay::new();
        d.handle(CustomerDisplayMessage::ShowAmount(50.0));
        d.handle(CustomerDisplayMessage::ShowQrCode(
            "url".to_string(),
            "WeChat".to_string(),
        ));
        d.handle(CustomerDisplayMessage::Reset);
        assert_eq!(d.amount(), 0.0);
        assert_eq!(d.payment_method(), None);
        assert_eq!(d.qr_code(), None);
        assert_eq!(d.custom_message(), None);
        assert_eq!(d.state(), CustomerDisplayState::Idle);
    }
}
