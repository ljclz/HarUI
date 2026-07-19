//! CustomerDisplay 客显屏组件 — POS 专用
//!
//! 第二屏（客显屏）独立渲染：
//! - 金额显示（大字体）
//! - 收款方式
//! - 二维码（扫码支付）
//! - 自定义信息

use har_ui_components::customer_display::{
    CustomerDisplay, CustomerDisplayMessage, CustomerDisplayState,
};

// ---------- 基础构造 ----------

#[test]
fn test_customer_display_default() {
    let d = CustomerDisplay::new();
    assert_eq!(d.amount(), 0.0);
    assert_eq!(d.payment_method(), None);
    assert_eq!(d.qr_code(), None);
    assert_eq!(d.custom_message(), None);
    assert_eq!(d.state(), CustomerDisplayState::Idle);
}

// ---------- 显示金额 ----------

#[test]
fn test_customer_display_show_amount() {
    let mut d = CustomerDisplay::new();
    d.handle(CustomerDisplayMessage::ShowAmount(88.50));
    assert!((d.amount() - 88.50).abs() < f64::EPSILON);
    assert_eq!(d.state(), CustomerDisplayState::ShowingAmount);
}

#[test]
fn test_customer_display_show_amount_zero() {
    let mut d = CustomerDisplay::new();
    d.handle(CustomerDisplayMessage::ShowAmount(0.0));
    assert!((d.amount() - 0.0).abs() < f64::EPSILON);
    assert_eq!(d.state(), CustomerDisplayState::ShowingAmount);
}

// ---------- 显示二维码 ----------

#[test]
fn test_customer_display_show_qr_code() {
    let mut d = CustomerDisplay::new();
    d.handle(CustomerDisplayMessage::ShowQrCode(
        "weixin://wxpay/bizpayurl?pr=ABC".to_string(),
        "微信支付".to_string(),
    ));
    assert_eq!(d.qr_code(), Some(&"weixin://wxpay/bizpayurl?pr=ABC".to_string()));
    assert_eq!(d.payment_method(), Some(&"微信支付".to_string()));
    assert_eq!(d.state(), CustomerDisplayState::ShowingQrCode);
}

#[test]
fn test_customer_display_show_qr_code_alipay() {
    let mut d = CustomerDisplay::new();
    d.handle(CustomerDisplayMessage::ShowQrCode(
        "https://qr.alipay.com/ABC".to_string(),
        "支付宝".to_string(),
    ));
    assert_eq!(d.payment_method(), Some(&"支付宝".to_string()));
}

// ---------- 自定义信息 ----------

#[test]
fn test_customer_display_show_custom_message() {
    let mut d = CustomerDisplay::new();
    d.handle(CustomerDisplayMessage::ShowCustomMessage("欢迎光临".to_string()));
    assert_eq!(d.custom_message(), Some(&"欢迎光临".to_string()));
    assert_eq!(d.state(), CustomerDisplayState::ShowingMessage);
}

// ---------- 重置 ----------

#[test]
fn test_customer_display_reset() {
    let mut d = CustomerDisplay::new();
    d.handle(CustomerDisplayMessage::ShowAmount(100.0));
    d.handle(CustomerDisplayMessage::ShowQrCode("url".to_string(), "WeChat".to_string()));
    d.handle(CustomerDisplayMessage::ShowCustomMessage("Hello".to_string()));
    d.handle(CustomerDisplayMessage::Reset);
    assert_eq!(d.amount(), 0.0);
    assert_eq!(d.payment_method(), None);
    assert_eq!(d.qr_code(), None);
    assert_eq!(d.custom_message(), None);
    assert_eq!(d.state(), CustomerDisplayState::Idle);
}

// ---------- 显示支付成功 ----------

#[test]
fn test_customer_display_show_success() {
    let mut d = CustomerDisplay::new();
    d.handle(CustomerDisplayMessage::ShowAmount(100.0));
    d.handle(CustomerDisplayMessage::ShowSuccess);
    assert_eq!(d.state(), CustomerDisplayState::Success);
}

#[test]
fn test_customer_display_show_success_clears_qr() {
    let mut d = CustomerDisplay::new();
    d.handle(CustomerDisplayMessage::ShowQrCode("url".to_string(), "WeChat".to_string()));
    d.handle(CustomerDisplayMessage::ShowSuccess);
    // 支付成功后清除二维码
    assert_eq!(d.qr_code(), None);
}

// ---------- 显示欢迎页 ----------

#[test]
fn test_customer_display_show_welcome() {
    let mut d = CustomerDisplay::new();
    d.handle(CustomerDisplayMessage::ShowWelcome);
    assert_eq!(d.state(), CustomerDisplayState::Welcome);
}

// ---------- 状态转换 ----------

#[test]
fn test_customer_display_state_transitions() {
    let mut d = CustomerDisplay::new();
    assert_eq!(d.state(), CustomerDisplayState::Idle);

    d.handle(CustomerDisplayMessage::ShowWelcome);
    assert_eq!(d.state(), CustomerDisplayState::Welcome);

    d.handle(CustomerDisplayMessage::ShowAmount(50.0));
    assert_eq!(d.state(), CustomerDisplayState::ShowingAmount);

    d.handle(CustomerDisplayMessage::ShowQrCode("url".to_string(), "WeChat".to_string()));
    assert_eq!(d.state(), CustomerDisplayState::ShowingQrCode);

    d.handle(CustomerDisplayMessage::ShowSuccess);
    assert_eq!(d.state(), CustomerDisplayState::Success);

    d.handle(CustomerDisplayMessage::ShowWelcome);
    assert_eq!(d.state(), CustomerDisplayState::Welcome);
}

// ---------- 金额格式化 ----------

#[test]
fn test_customer_display_formatted_amount() {
    let mut d = CustomerDisplay::new();
    d.handle(CustomerDisplayMessage::ShowAmount(1234.5));
    // 应保留两位小数
    assert_eq!(d.formatted_amount(), "1234.50");
}

#[test]
fn test_customer_display_formatted_amount_zero() {
    let d = CustomerDisplay::new();
    assert_eq!(d.formatted_amount(), "0.00");
}

#[test]
fn test_customer_display_formatted_amount_large() {
    let mut d = CustomerDisplay::new();
    d.handle(CustomerDisplayMessage::ShowAmount(999999.99));
    assert_eq!(d.formatted_amount(), "999999.99");
}
