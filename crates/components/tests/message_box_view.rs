//! MessageBox view() 测试 — TDD RED 阶段

use har_ui_components::message_box::{
    MessageBox, MessageBoxMessage, MessageBoxType,
};
use har_ui_core::theme::Theme;

#[test]
fn test_message_box_view_hidden_renders_empty() {
    let theme = Theme::element_light();
    let mb = MessageBox::new("Title", "Body");
    assert!(!mb.visible());
    let _element = mb.view(&theme, |_| ());
}

#[test]
fn test_message_box_view_opened_renders() {
    let theme = Theme::element_light();
    let mut mb = MessageBox::new("Title", "Body");
    mb.handle(MessageBoxMessage::Open);
    let _element = mb.view(&theme, |_| ());
}

#[test]
fn test_message_box_view_success_type_renders() {
    let theme = Theme::element_light();
    let mut mb = MessageBox::new("Title", "Body").with_type(MessageBoxType::Success);
    mb.handle(MessageBoxMessage::Open);
    let _element = mb.view(&theme, |_| ());
}

#[test]
fn test_message_box_view_warning_type_renders() {
    let theme = Theme::element_light();
    let mut mb = MessageBox::new("Title", "Body").with_type(MessageBoxType::Warning);
    mb.handle(MessageBoxMessage::Open);
    let _element = mb.view(&theme, |_| ());
}

#[test]
fn test_message_box_view_error_type_renders() {
    let theme = Theme::element_light();
    let mut mb = MessageBox::new("Title", "Body").with_type(MessageBoxType::Error);
    mb.handle(MessageBoxMessage::Open);
    let _element = mb.view(&theme, |_| ());
}

#[test]
fn test_message_box_view_with_cancel_button_renders() {
    let theme = Theme::element_light();
    let mut mb = MessageBox::new("Title", "Body").with_show_cancel(true);
    mb.handle(MessageBoxMessage::Open);
    let _element = mb.view(&theme, |_| ());
}

#[test]
fn test_message_box_view_without_close_button_renders() {
    let theme = Theme::element_light();
    let mut mb = MessageBox::new("Title", "Body").with_show_close(false);
    mb.handle(MessageBoxMessage::Open);
    let _element = mb.view(&theme, |_| ());
}

#[test]
fn test_message_box_view_without_confirm_button_renders() {
    let theme = Theme::element_light();
    let mut mb = MessageBox::new("Title", "Body").with_show_confirm(false);
    mb.handle(MessageBoxMessage::Open);
    let _element = mb.view(&theme, |_| ());
}

#[test]
fn test_message_box_view_prompt_mode_renders() {
    let theme = Theme::element_light();
    let mut mb = MessageBox::prompt("Title", "Body");
    mb.handle(MessageBoxMessage::Open);
    mb.handle(MessageBoxMessage::Input("hello".to_string()));
    let _element = mb.view(&theme, |_| ());
}

#[test]
fn test_message_box_view_custom_button_text_renders() {
    let theme = Theme::element_light();
    let mut mb = MessageBox::new("Title", "Body")
        .with_confirm_text("OK")
        .with_show_cancel(true)
        .with_cancel_text("NO");
    mb.handle(MessageBoxMessage::Open);
    let _element = mb.view(&theme, |_| ());
}

#[test]
fn test_message_box_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let mut mb = MessageBox::new("Title", "Body");
    mb.handle(MessageBoxMessage::Open);
    let _element = mb.view(&theme, |_| ());
}
