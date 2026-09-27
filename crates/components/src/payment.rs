//! Payment 面板组件 — POS 支付方式选择与结算
//!
//! 支持 5 种支付方式（Cash/WeChat/Alipay/UnionPay/MemberBalance）。
//! 业务规则：
//! - 现金支付：实收 ≥ 总额，找零 = 实收 - 总额
//! - 非现金支付：实收自动等于总额，找零 0
//! - 总额变更后已确认状态重置为 Pending

use har_ui_core::theme::Theme;
use iced::widget::{button, container, text};
use iced::{Color, Element, Length, Padding};

/// 支付方式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PaymentMethod {
    #[default]
    Cash,
    WeChat,
    Alipay,
    UnionPay,
    MemberBalance,
}

impl PaymentMethod {
    /// 是否为现金支付
    pub fn is_cash(self) -> bool {
        matches!(self, PaymentMethod::Cash)
    }
}

/// 支付状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PaymentState {
    #[default]
    Pending,
    Confirmed,
    Cancelled,
}

/// 支付消息
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PaymentMessage {
    /// 设置实收金额
    Received(f64),
    /// 切换支付方式
    SwitchMethod(PaymentMethod),
    /// 更新总额（外部购物车变化时）
    UpdateTotal(f64),
    /// 确认收款
    Confirm,
    /// 取消
    Cancel,
}

/// Payment 组件
#[derive(Debug, Clone)]
pub struct Payment {
    total: f64,
    method: PaymentMethod,
    received: f64,
    state: PaymentState,
}

impl Payment {
    pub fn new(total: f64) -> Self {
        Self {
            total,
            method: PaymentMethod::Cash,
            received: 0.0,
            state: PaymentState::Pending,
        }
    }

    pub fn with_method(mut self, m: PaymentMethod) -> Self {
        self.method = m;
        // 非现金支付自动设置实收 = 总额
        if !m.is_cash() {
            self.received = self.total;
        }
        self
    }

    pub fn total(&self) -> f64 {
        self.total
    }

    pub fn method(&self) -> PaymentMethod {
        self.method
    }

    pub fn received(&self) -> f64 {
        self.received
    }

    /// 找零 = max(0, 实收 - 总额)
    pub fn change(&self) -> f64 {
        (self.received - self.total).max(0.0)
    }

    pub fn state(&self) -> PaymentState {
        self.state
    }

    /// 重置为初始状态
    pub fn reset(&mut self) {
        self.method = PaymentMethod::Cash;
        self.received = 0.0;
        self.state = PaymentState::Pending;
    }

    /// 处理消息
    pub fn handle(&mut self, msg: PaymentMessage) {
        match msg {
            PaymentMessage::Received(v) => {
                if self.state != PaymentState::Pending {
                    return;
                }
                if v < 0.0 {
                    return; // 忽略负值
                }
                // 现金模式下接受任意实收；非现金模式锁定 received = total
                if self.method.is_cash() {
                    self.received = v;
                }
            }
            PaymentMessage::SwitchMethod(m) => {
                if self.state != PaymentState::Pending {
                    return;
                }
                self.method = m;
                if m.is_cash() {
                    self.received = 0.0;
                } else {
                    self.received = self.total;
                }
            }
            PaymentMessage::UpdateTotal(v) => {
                self.total = v;
                // 总额变更后状态重置
                self.state = PaymentState::Pending;
                // 非现金支付自动同步 received
                if !self.method.is_cash() {
                    self.received = self.total;
                }
            }
            PaymentMessage::Confirm => {
                if self.state != PaymentState::Pending {
                    return;
                }
                // 总额为 0 直接确认
                if self.total <= 0.0 {
                    self.state = PaymentState::Confirmed;
                    return;
                }
                // 现金支付需 received >= total
                if self.method.is_cash() {
                    if self.received + f64::EPSILON >= self.total {
                        self.state = PaymentState::Confirmed;
                    }
                } else {
                    // 非现金已自动 received = total
                    self.state = PaymentState::Confirmed;
                }
            }
            PaymentMessage::Cancel => {
                self.received = 0.0;
                self.state = PaymentState::Cancelled;
            }
        }
    }

    /// 渲染 Payment 为 iced::Element
    ///
    /// # 参数
    /// - `theme`: HarUI 主题引用
    /// - `on_pay`: 点击支付方式按钮时发出消息，参数为支付方式名称
    ///   （"Cash" / "WeChat" / "Alipay" / "UnionPay" / "MemberBalance"）
    /// - `on_cancel`: 点击取消按钮时发出消息
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        on_pay: impl Fn(String) -> Message + 'a,
        on_cancel: impl Fn() -> Message + 'a,
    ) -> Element<'a, Message> {
        let primary = Color::from(theme.primary.base);
        let danger = Color::from(theme.danger.base);
        let text_primary = Color::from(theme.neutral.text_primary);
        let text_secondary = Color::from(theme.neutral.text_secondary);
        let text_placeholder = Color::from(theme.neutral.text_placeholder);
        let border_lighter = Color::from(theme.neutral.border_lighter);

        let mut col_children: Vec<Element<'a, Message>> = Vec::new();

        // 总额
        let total_label = text("总额".to_string()).color(text_secondary).size(12.0);
        let total_value = text(format!("¥ {:.2}", self.total))
            .color(primary)
            .size(28.0);
        let total_row = iced::widget::Row::new()
            .push(total_label)
            .push(iced::widget::Space::with_width(Length::Fixed(8.0)))
            .push(total_value)
            .align_y(iced::Alignment::Center);
        col_children.push(
            container(total_row)
                .width(Length::Fill)
                .padding(Padding::from([8u16, 12u16]))
                .into(),
        );

        // 实收 + 找零（仅现金支付时显示）
        if self.method.is_cash() && self.received > 0.0 {
            let received_row = iced::widget::Row::new()
                .push(text("实收".to_string()).color(text_secondary).size(12.0))
                .push(iced::widget::Space::with_width(Length::Fixed(8.0)))
                .push(
                    text(format!("¥ {:.2}", self.received))
                        .color(text_primary)
                        .size(16.0),
                )
                .align_y(iced::Alignment::Center);
            let change_row = iced::widget::Row::new()
                .push(text("找零".to_string()).color(text_secondary).size(12.0))
                .push(iced::widget::Space::with_width(Length::Fixed(8.0)))
                .push(
                    text(format!("¥ {:.2}", self.change()))
                        .color(primary)
                        .size(16.0),
                )
                .align_y(iced::Alignment::Center);
            col_children.push(
                container(received_row)
                    .width(Length::Fill)
                    .padding(Padding::from([4u16, 12u16]))
                    .into(),
            );
            col_children.push(
                container(change_row)
                    .width(Length::Fill)
                    .padding(Padding::from([4u16, 12u16]))
                    .into(),
            );
        }

        // 支付方式按钮
        let methods: [(PaymentMethod, &str); 5] = [
            (PaymentMethod::Cash, "现金"),
            (PaymentMethod::WeChat, "微信"),
            (PaymentMethod::Alipay, "支付宝"),
            (PaymentMethod::UnionPay, "银联"),
            (PaymentMethod::MemberBalance, "会员余额"),
        ];
        let mut method_row = iced::widget::Row::new().spacing(4);
        for (method, label) in methods {
            let method_name = format!("{:?}", method);
            let is_current = method == self.method;
            let btn_color = if is_current { primary } else { text_secondary };
            let btn = button(text(label.to_string()).color(btn_color).size(14.0))
                .padding(Padding::from([8u16, 12u16]))
                .on_press(on_pay(method_name))
                .style(move |_t, _status| iced::widget::button::Style {
                    background: if is_current {
                        Some(iced::Background::Color(Color { a: 0.1, ..primary }))
                    } else {
                        None
                    },
                    text_color: btn_color,
                    border: iced::Border {
                        color: if is_current { primary } else { border_lighter },
                        width: if is_current { 2.0 } else { 1.0 },
                        radius: iced::border::radius(4.0),
                    },
                    shadow: iced::Shadow::default(),
                });
            method_row = method_row.push(btn);
        }
        col_children.push(
            container(method_row)
                .width(Length::Fill)
                .padding(Padding::from([8u16, 12u16]))
                .into(),
        );

        // 状态文本
        let state_text = match self.state {
            PaymentState::Pending => text("待支付".to_string())
                .color(text_placeholder)
                .size(12.0),
            PaymentState::Confirmed => text("已确认".to_string()).color(primary).size(12.0),
            PaymentState::Cancelled => text("已取消".to_string()).color(danger).size(12.0),
        };
        col_children.push(
            container(state_text)
                .width(Length::Fill)
                .padding(Padding::from([4u16, 12u16]))
                .into(),
        );

        // 取消按钮
        let cancel_btn = button(text("取消".to_string()).color(danger).size(14.0))
            .padding(Padding::from([8u16, 16u16]))
            .on_press(on_cancel())
            .style(move |_t, _status| iced::widget::button::Style {
                background: None,
                text_color: danger,
                border: iced::Border {
                    color: danger,
                    width: 1.0,
                    radius: iced::border::radius(4.0),
                },
                shadow: iced::Shadow::default(),
            });
        col_children.push(
            container(cancel_btn)
                .width(Length::Fill)
                .padding(Padding::from([8u16, 12u16]))
                .into(),
        );

        let col = iced::widget::Column::with_children(col_children).spacing(0);
        container(col)
            .width(Length::Fill)
            .style(move |_t| iced::widget::container::Style {
                text_color: None,
                background: None,
                border: iced::Border {
                    color: border_lighter,
                    width: 1.0,
                    radius: iced::border::radius(4.0),
                },
                shadow: iced::Shadow::default(),
            })
            .into()
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_payment_method_is_cash() {
        assert!(PaymentMethod::Cash.is_cash());
        assert!(!PaymentMethod::WeChat.is_cash());
        assert!(!PaymentMethod::Alipay.is_cash());
        assert!(!PaymentMethod::UnionPay.is_cash());
        assert!(!PaymentMethod::MemberBalance.is_cash());
    }

    #[test]
    fn test_payment_change_calculation() {
        let mut p = Payment::new(100.0);
        p.handle(PaymentMessage::Received(150.0));
        assert!((p.change() - 50.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_payment_state_transitions() {
        let mut p = Payment::new(10.0);
        p.handle(PaymentMessage::Received(10.0));
        p.handle(PaymentMessage::Confirm);
        assert_eq!(p.state(), PaymentState::Confirmed);
    }
}
