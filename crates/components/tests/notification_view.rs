//! Notification view() 测试 — TDD RED 阶段

use har_ui_components::notification::{
    Notification, NotificationMessage, NotificationPosition, NotificationType,
};
use har_ui_core::theme::Theme;

#[test]
fn test_notification_view_default_renders() {
    let theme = Theme::element_light();
    let n = Notification::new("Title", "Body");
    let _element = n.view(&theme);
}

#[test]
fn test_notification_view_success_type_renders() {
    let theme = Theme::element_light();
    let n = Notification::new("Title", "Body").with_type(NotificationType::Success);
    let _element = n.view(&theme);
}

#[test]
fn test_notification_view_warning_type_renders() {
    let theme = Theme::element_light();
    let n = Notification::new("Title", "Body").with_type(NotificationType::Warning);
    let _element = n.view(&theme);
}

#[test]
fn test_notification_view_error_type_renders() {
    let theme = Theme::element_light();
    let n = Notification::new("Title", "Body").with_type(NotificationType::Error);
    let _element = n.view(&theme);
}

#[test]
fn test_notification_view_info_type_renders() {
    let theme = Theme::element_light();
    let n = Notification::new("Title", "Body").with_type(NotificationType::Info);
    let _element = n.view(&theme);
}

#[test]
fn test_notification_view_without_close_button_renders() {
    let theme = Theme::element_light();
    let n = Notification::new("Title", "Body").with_show_close(false);
    let _element = n.view(&theme);
}

#[test]
fn test_notification_view_closed_renders_empty() {
    let theme = Theme::element_light();
    let mut n = Notification::new("Title", "Body");
    n.handle(NotificationMessage::Close);
    assert!(n.closed());
    let _element = n.view(&theme);
}

#[test]
fn test_notification_view_with_offset_renders() {
    let theme = Theme::element_light();
    let n = Notification::new("Title", "Body").with_offset(50);
    let _element = n.view(&theme);
}

#[test]
fn test_notification_view_top_left_position_renders() {
    let theme = Theme::element_light();
    let n = Notification::new("Title", "Body").with_position(NotificationPosition::TopLeft);
    let _element = n.view(&theme);
}

#[test]
fn test_notification_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let n = Notification::new("Title", "Body").with_type(NotificationType::Success);
    let _element = n.view(&theme);
}
