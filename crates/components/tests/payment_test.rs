//! Payment 面板组件 — POS 支付方式选择与结算
//!
//! 支持 5 种支付方式：现金、微信、支付宝、银联、会员余额。
//! 提供：实收金额、找零计算、收款确认/取消。
//! 业务规则：实收金额 ≥ 总额（现金支付时）；找零 = 实收 - 总额。
//! 其他支付方式（非现金）要求实收 = 总额。

use har_ui_components::payment::{Payment, PaymentMessage, PaymentMethod, PaymentState};

// ---------- 基础构造 ----------

#[test]
fn test_payment_default() {
    let p = Payment::new(100.00);
    assert!((p.total() - 100.0).abs() < f64::EPSILON);
    assert_eq!(p.method(), PaymentMethod::Cash);
    assert!((p.received() - 0.0).abs() < f64::EPSILON);
    assert!((p.change() - 0.0).abs() < f64::EPSILON);
    assert_eq!(p.state(), PaymentState::Pending);
}

#[test]
fn test_payment_with_method() {
    let p = Payment::new(50.0).with_method(PaymentMethod::WeChat);
    assert_eq!(p.method(), PaymentMethod::WeChat);
}

#[test]
fn test_payment_methods_enum() {
    let methods = [
        PaymentMethod::Cash,
        PaymentMethod::WeChat,
        PaymentMethod::Alipay,
        PaymentMethod::UnionPay,
        PaymentMethod::MemberBalance,
    ];
    assert_eq!(methods.len(), 5);
}

// ---------- 现金支付 ----------

#[test]
fn test_payment_cash_exact_amount() {
    let mut p = Payment::new(100.0);
    p.handle(PaymentMessage::Received(100.0));
    assert!((p.received() - 100.0).abs() < f64::EPSILON);
    assert!((p.change() - 0.0).abs() < f64::EPSILON);
}

#[test]
fn test_payment_cash_with_change() {
    // 收 150，找零 50
    let mut p = Payment::new(100.0);
    p.handle(PaymentMessage::Received(150.0));
    assert!((p.change() - 50.0).abs() < f64::EPSILON);
}

#[test]
fn test_payment_cash_insufficient_cannot_confirm() {
    // 收 80，不足以支付 100，无法确认
    let mut p = Payment::new(100.0);
    p.handle(PaymentMessage::Received(80.0));
    p.handle(PaymentMessage::Confirm);
    assert_eq!(p.state(), PaymentState::Pending); // 未确认
}

#[test]
fn test_payment_cash_sufficient_can_confirm() {
    let mut p = Payment::new(100.0);
    p.handle(PaymentMessage::Received(100.0));
    p.handle(PaymentMessage::Confirm);
    assert_eq!(p.state(), PaymentState::Confirmed);
}

// ---------- 非现金支付 ----------

#[test]
fn test_payment_wechat_auto_received() {
    // 切换到微信支付后，实收自动等于总额，找零 0
    let mut p = Payment::new(88.88);
    p.handle(PaymentMessage::SwitchMethod(PaymentMethod::WeChat));
    assert!((p.received() - 88.88).abs() < f64::EPSILON);
    assert!((p.change() - 0.0).abs() < f64::EPSILON);
}

#[test]
fn test_payment_alipay_auto_received() {
    let mut p = Payment::new(50.0);
    p.handle(PaymentMessage::SwitchMethod(PaymentMethod::Alipay));
    assert!((p.received() - 50.0).abs() < f64::EPSILON);
}

#[test]
fn test_payment_unionpay_auto_received() {
    let mut p = Payment::new(200.0);
    p.handle(PaymentMessage::SwitchMethod(PaymentMethod::UnionPay));
    assert!((p.received() - 200.0).abs() < f64::EPSILON);
}

#[test]
fn test_payment_member_balance_auto_received() {
    let mut p = Payment::new(30.0);
    p.handle(PaymentMessage::SwitchMethod(PaymentMethod::MemberBalance));
    assert!((p.received() - 30.0).abs() < f64::EPSILON);
}

#[test]
fn test_payment_noncash_can_confirm_immediately() {
    let mut p = Payment::new(100.0);
    p.handle(PaymentMessage::SwitchMethod(PaymentMethod::WeChat));
    p.handle(PaymentMessage::Confirm);
    assert_eq!(p.state(), PaymentState::Confirmed);
}

// ---------- 切回现金重置 ----------

#[test]
fn test_payment_switch_back_to_cash_resets_received() {
    let mut p = Payment::new(100.0);
    p.handle(PaymentMessage::SwitchMethod(PaymentMethod::WeChat));
    p.handle(PaymentMessage::SwitchMethod(PaymentMethod::Cash));
    assert!((p.received() - 0.0).abs() < f64::EPSILON);
    assert!((p.change() - 0.0).abs() < f64::EPSILON);
}

// ---------- 取消 ----------

#[test]
fn test_payment_cancel() {
    let mut p = Payment::new(100.0);
    p.handle(PaymentMessage::Received(150.0));
    p.handle(PaymentMessage::Cancel);
    assert_eq!(p.state(), PaymentState::Cancelled);
    assert!((p.received() - 0.0).abs() < f64::EPSILON);
}

// ---------- 总额更新 ----------

#[test]
fn test_payment_update_total_recalculates_change() {
    let mut p = Payment::new(100.0);
    p.handle(PaymentMessage::Received(150.0));
    // 更新总额为 120，找零应变为 30
    p.handle(PaymentMessage::UpdateTotal(120.0));
    assert!((p.total() - 120.0).abs() < f64::EPSILON);
    assert!((p.change() - 30.0).abs() < f64::EPSILON);
}

#[test]
fn test_payment_update_total_invalidates_confirm() {
    // 总额变更后，原本已确认的状态需重置为 Pending
    let mut p = Payment::new(100.0);
    p.handle(PaymentMessage::Received(100.0));
    p.handle(PaymentMessage::Confirm);
    assert_eq!(p.state(), PaymentState::Confirmed);
    p.handle(PaymentMessage::UpdateTotal(120.0));
    assert_eq!(p.state(), PaymentState::Pending);
}

// ---------- 边界 ----------

#[test]
fn test_payment_zero_total() {
    // 总额为 0 时直接确认
    let mut p = Payment::new(0.0);
    p.handle(PaymentMessage::Confirm);
    assert_eq!(p.state(), PaymentState::Confirmed);
}

#[test]
fn test_payment_negative_received_ignored() {
    let mut p = Payment::new(100.0);
    p.handle(PaymentMessage::Received(-50.0));
    assert!((p.received() - 0.0).abs() < f64::EPSILON);
}

// ---------- 重置 ----------

#[test]
fn test_payment_reset() {
    let mut p = Payment::new(100.0);
    p.handle(PaymentMessage::Received(150.0));
    p.handle(PaymentMessage::Confirm);
    p.reset();
    assert_eq!(p.state(), PaymentState::Pending);
    assert!((p.received() - 0.0).abs() < f64::EPSILON);
    assert_eq!(p.method(), PaymentMethod::Cash);
}
