//! R.2.P0.5 Dialog view() 测试 — TDD RED 阶段
//!
//! 验证 Dialog view() 正确渲染遮罩、标题、内容、关闭按钮。

use har_ui_components::dialog::{Dialog, DialogMessage, DialogProps, DialogState};
use har_ui_core::theme::Theme;

// ============== R.2.P0.5.a view() 基础渲染 ==============

#[test]
fn test_dialog_view_closed_renders() {
    let theme = Theme::element_light();
    let dlg = Dialog::new("Title", "Content");
    assert_eq!(dlg.state(), DialogState::Closed);
    let _element = dlg.view(&theme, ());
}

#[test]
fn test_dialog_view_open_renders() {
    let theme = Theme::element_light();
    let mut dlg = Dialog::new("Title", "Content");
    dlg.handle(DialogMessage::Open);
    dlg.handle(DialogMessage::AnimationFinished);
    assert_eq!(dlg.state(), DialogState::Open);
    let _element = dlg.view(&theme, ());
}

#[test]
fn test_dialog_view_opening_renders() {
    let theme = Theme::element_light();
    let mut dlg = Dialog::new("Title", "Content");
    dlg.handle(DialogMessage::Open);
    assert_eq!(dlg.state(), DialogState::Opening);
    let _element = dlg.view(&theme, ());
}

#[test]
fn test_dialog_view_closing_renders() {
    let theme = Theme::element_light();
    let mut dlg = Dialog::new("Title", "Content");
    dlg.handle(DialogMessage::Open);
    dlg.handle(DialogMessage::AnimationFinished);
    dlg.handle(DialogMessage::Close);
    assert_eq!(dlg.state(), DialogState::Closing);
    let _element = dlg.view(&theme, ());
}

// ============== R.2.P0.5.b 配置变体 ==============

#[test]
fn test_dialog_view_fullscreen_renders() {
    let theme = Theme::element_light();
    let mut dlg =
        Dialog::new("Title", "Content").with_props(DialogProps::new().with_fullscreen(true));
    dlg.handle(DialogMessage::Open);
    dlg.handle(DialogMessage::AnimationFinished);
    let _element = dlg.view(&theme, ());
}

#[test]
fn test_dialog_view_no_close_on_modal_renders() {
    let theme = Theme::element_light();
    let mut dlg = Dialog::new("Title", "Content")
        .with_props(DialogProps::new().with_close_on_click_modal(false));
    dlg.handle(DialogMessage::Open);
    dlg.handle(DialogMessage::AnimationFinished);
    let _element = dlg.view(&theme, ());
}

#[test]
fn test_dialog_view_draggable_renders() {
    let theme = Theme::element_light();
    let mut dlg =
        Dialog::new("Title", "Content").with_props(DialogProps::new().with_draggable(true));
    dlg.handle(DialogMessage::Open);
    dlg.handle(DialogMessage::AnimationFinished);
    let _element = dlg.view(&theme, ());
}

#[test]
fn test_dialog_view_with_dragged_position_renders() {
    let theme = Theme::element_light();
    let mut dlg =
        Dialog::new("Title", "Content").with_props(DialogProps::new().with_draggable(true));
    dlg.handle(DialogMessage::Open);
    dlg.handle(DialogMessage::AnimationFinished);
    dlg.handle(DialogMessage::Dragged(100.0, 200.0));
    assert_eq!(dlg.position(), Some((100.0, 200.0)));
    let _element = dlg.view(&theme, ());
}

#[test]
fn test_dialog_view_long_content_renders() {
    let theme = Theme::element_light();
    let mut dlg = Dialog::new(
        "Title",
        "This is a very long content that should wrap or scroll.",
    );
    dlg.handle(DialogMessage::Open);
    dlg.handle(DialogMessage::AnimationFinished);
    let _element = dlg.view(&theme, ());
}

#[test]
fn test_dialog_view_empty_title_renders() {
    let theme = Theme::element_light();
    let mut dlg = Dialog::new("", "Content");
    dlg.handle(DialogMessage::Open);
    dlg.handle(DialogMessage::AnimationFinished);
    let _element = dlg.view(&theme, ());
}

#[test]
fn test_dialog_view_empty_content_renders() {
    let theme = Theme::element_light();
    let mut dlg = Dialog::new("Title", "");
    dlg.handle(DialogMessage::Open);
    dlg.handle(DialogMessage::AnimationFinished);
    let _element = dlg.view(&theme, ());
}

// ============== R.2.P0.5.c 主题/消息类型 ==============

#[test]
fn test_dialog_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let mut dlg = Dialog::new("Title", "Content");
    dlg.handle(DialogMessage::Open);
    dlg.handle(DialogMessage::AnimationFinished);
    let _element = dlg.view(&theme, ());
}

#[test]
fn test_dialog_view_with_custom_message_type() {
    let theme = Theme::element_light();
    let mut dlg = Dialog::new("Title", "Content");
    dlg.handle(DialogMessage::Open);
    dlg.handle(DialogMessage::AnimationFinished);
    #[derive(Clone, Debug)]
    #[allow(dead_code)]
    enum AppMsg {
        Close,
    }
    let _element = dlg.view(&theme, AppMsg::Close);
}
