//! Payment view() 测试 — TDD RED 阶段

use har_ui_components::payment::{Payment, PaymentMessage, PaymentMethod};
use har_ui_core::theme::Theme;

#[test]
fn test_payment_view_default_renders() {
    let theme = Theme::element_light();
    let p = Payment::new(100.0);
    let _element = p.view(&theme, |_| (), || ());
}

#[test]
fn test_payment_view_zero_total_renders() {
    let theme = Theme::element_light();
    let p = Payment::new(0.0);
    let _element = p.view(&theme, |_| (), || ());
}

#[test]
fn test_payment_view_cash_method_renders() {
    let theme = Theme::element_light();
    let p = Payment::new(100.0).with_method(PaymentMethod::Cash);
    let _element = p.view(&theme, |_| (), || ());
}

#[test]
fn test_payment_view_wechat_method_renders() {
    let theme = Theme::element_light();
    let p = Payment::new(100.0).with_method(PaymentMethod::WeChat);
    let _element = p.view(&theme, |_| (), || ());
}

#[test]
fn test_payment_view_alipay_method_renders() {
    let theme = Theme::element_light();
    let p = Payment::new(100.0).with_method(PaymentMethod::Alipay);
    let _element = p.view(&theme, |_| (), || ());
}

#[test]
fn test_payment_view_unionpay_method_renders() {
    let theme = Theme::element_light();
    let p = Payment::new(100.0).with_method(PaymentMethod::UnionPay);
    let _element = p.view(&theme, |_| (), || ());
}

#[test]
fn test_payment_view_member_balance_renders() {
    let theme = Theme::element_light();
    let p = Payment::new(100.0).with_method(PaymentMethod::MemberBalance);
    let _element = p.view(&theme, |_| (), || ());
}

#[test]
fn test_payment_view_with_received_renders() {
    let theme = Theme::element_light();
    let mut p = Payment::new(100.0);
    p.handle(PaymentMessage::Received(150.0));
    let _element = p.view(&theme, |_| (), || ());
}

#[test]
fn test_payment_view_confirmed_renders() {
    let theme = Theme::element_light();
    let mut p = Payment::new(100.0);
    p.handle(PaymentMessage::Received(100.0));
    p.handle(PaymentMessage::Confirm);
    let _element = p.view(&theme, |_| (), || ());
}

#[test]
fn test_payment_view_cancelled_renders() {
    let theme = Theme::element_light();
    let mut p = Payment::new(100.0);
    p.handle(PaymentMessage::Cancel);
    let _element = p.view(&theme, |_| (), || ());
}

#[test]
fn test_payment_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let p = Payment::new(99.99).with_method(PaymentMethod::WeChat);
    let _element = p.view(&theme, |_| (), || ());
}

#[test]
fn test_payment_view_custom_message_type() {
    let theme = Theme::element_light();
    let p = Payment::new(50.0);
    #[derive(Clone, Debug)]
    #[allow(dead_code)]
    enum AppMsg {
        Pay(String),
        Cancel,
    }
    let _element = p.view(&theme, |s| AppMsg::Pay(s), || AppMsg::Cancel);
}
