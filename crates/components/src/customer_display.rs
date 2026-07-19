//! CustomerDisplay 客显屏组件 — POS 专用
//!
//! 第二屏（客显屏）独立渲染：金额显示、收款方式、二维码、自定义信息。
//! 状态机：Idle / Welcome / ShowingAmount / ShowingQrCode / ShowingMessage / Success

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
        d.handle(CustomerDisplayMessage::ShowQrCode("url".to_string(), "WeChat".to_string()));
        d.handle(CustomerDisplayMessage::Reset);
        assert_eq!(d.amount(), 0.0);
        assert_eq!(d.payment_method(), None);
        assert_eq!(d.qr_code(), None);
        assert_eq!(d.custom_message(), None);
        assert_eq!(d.state(), CustomerDisplayState::Idle);
    }
}
