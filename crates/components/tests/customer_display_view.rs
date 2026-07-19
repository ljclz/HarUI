//! CustomerDisplay view() 测试 — TDD RED 阶段

use har_ui_components::customer_display::{CustomerDisplay, CustomerDisplayMessage};
use har_ui_core::theme::Theme;

#[test]
fn test_customer_display_view_default_renders() {
    let theme = Theme::element_light();
    let d = CustomerDisplay::new();
    let _element = d.view(&theme);
}

#[test]
fn test_customer_display_view_amount_renders() {
    let theme = Theme::element_light();
    let mut d = CustomerDisplay::new();
    d.handle(CustomerDisplayMessage::ShowAmount(128.50));
    let _element = d.view(&theme);
}

#[test]
fn test_customer_display_view_welcome_renders() {
    let theme = Theme::element_light();
    let mut d = CustomerDisplay::new();
    d.handle(CustomerDisplayMessage::ShowWelcome);
    let _element = d.view(&theme);
}

#[test]
fn test_customer_display_view_qr_code_renders() {
    let theme = Theme::element_light();
    let mut d = CustomerDisplay::new();
    d.handle(CustomerDisplayMessage::ShowQrCode(
        "https://example.com/pay".to_string(),
        "WeChat".to_string(),
    ));
    let _element = d.view(&theme);
}

#[test]
fn test_customer_display_view_message_renders() {
    let theme = Theme::element_light();
    let mut d = CustomerDisplay::new();
    d.handle(CustomerDisplayMessage::ShowCustomMessage("请扫描商品条码".to_string()));
    let _element = d.view(&theme);
}

#[test]
fn test_customer_display_view_success_renders() {
    let theme = Theme::element_light();
    let mut d = CustomerDisplay::new();
    d.handle(CustomerDisplayMessage::ShowAmount(50.0));
    d.handle(CustomerDisplayMessage::ShowSuccess);
    let _element = d.view(&theme);
}

#[test]
fn test_customer_display_view_reset_renders() {
    let theme = Theme::element_light();
    let mut d = CustomerDisplay::new();
    d.handle(CustomerDisplayMessage::ShowAmount(50.0));
    d.handle(CustomerDisplayMessage::Reset);
    let _element = d.view(&theme);
}

#[test]
fn test_customer_display_view_large_amount_renders() {
    let theme = Theme::element_light();
    let mut d = CustomerDisplay::new();
    d.handle(CustomerDisplayMessage::ShowAmount(9999999.99));
    let _element = d.view(&theme);
}

#[test]
fn test_customer_display_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let mut d = CustomerDisplay::new();
    d.handle(CustomerDisplayMessage::ShowAmount(88.88));
    d.handle(CustomerDisplayMessage::ShowSuccess);
    let _element = d.view(&theme);
}
