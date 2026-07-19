//! Message view() 测试 — TDD RED 阶段

use har_ui_components::message::{MessageItem, MessageType};
use har_ui_core::theme::Theme;

#[test]
fn test_message_view_default_info_renders() {
    let theme = Theme::element_light();
    let m = MessageItem::new("hello", MessageType::Info);
    let _element = m.view(&theme);
}

#[test]
fn test_message_view_success_type_renders() {
    let theme = Theme::element_light();
    let m = MessageItem::new("done", MessageType::Success);
    let _element = m.view(&theme);
}

#[test]
fn test_message_view_warning_type_renders() {
    let theme = Theme::element_light();
    let m = MessageItem::new("warn", MessageType::Warning);
    let _element = m.view(&theme);
}

#[test]
fn test_message_view_error_type_renders() {
    let theme = Theme::element_light();
    let m = MessageItem::new("boom", MessageType::Error);
    let _element = m.view(&theme);
}

#[test]
fn test_message_view_closed_renders_empty() {
    let theme = Theme::element_light();
    let mut m = MessageItem::new("hi", MessageType::Info).with_duration(100);
    // 触发 tick 使其关闭
    assert!(m.tick(100));
    assert!(m.closed());
    let _element = m.view(&theme);
}

#[test]
fn test_message_view_long_text_renders() {
    let theme = Theme::element_light();
    let long = "A".repeat(200);
    let m = MessageItem::new(long, MessageType::Info);
    let _element = m.view(&theme);
}

#[test]
fn test_message_view_zero_duration_renders() {
    let theme = Theme::element_light();
    let m = MessageItem::new("sticky", MessageType::Info).with_duration(0);
    let _element = m.view(&theme);
}

#[test]
fn test_message_view_empty_text_renders() {
    let theme = Theme::element_light();
    let m = MessageItem::new("", MessageType::Info);
    let _element = m.view(&theme);
}

#[test]
fn test_message_view_not_closed_after_tick_below_duration_renders() {
    let theme = Theme::element_light();
    let mut m = MessageItem::new("hi", MessageType::Info).with_duration(1000);
    assert!(!m.tick(500));
    assert!(!m.closed());
    let _element = m.view(&theme);
}

#[test]
fn test_message_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let m = MessageItem::new("hi", MessageType::Success);
    let _element = m.view(&theme);
}
