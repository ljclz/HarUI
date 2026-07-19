//! Payment 面板组件 — POS 支付方式选择与结算
//!
//! 支持 5 种支付方式（Cash/WeChat/Alipay/UnionPay/MemberBalance）。
//! 业务规则：
//! - 现金支付：实收 ≥ 总额，找零 = 实收 - 总额
//! - 非现金支付：实收自动等于总额，找零 0
//! - 总额变更后已确认状态重置为 Pending

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
