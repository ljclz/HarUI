//! Dialog 组件测试
//!
//! 参考 Element Plus `<el-dialog>` 组件 API。
//! 测试覆盖：
//! - visible 开/关
//! - 遮罩点击关闭
//! - Escape 关闭
//! - fullscreen 满屏模式
//! - draggable 拖拽
//! - 多 Dialog 叠加
//! - 动画状态机

use har_ui_components::dialog::{Dialog, DialogMessage, DialogProps, DialogState};

#[test]
fn test_dialog_default_is_closed() {
    let dlg = Dialog::new("Title", "Content");
    assert_eq!(dlg.state(), DialogState::Closed);
    assert!(!dlg.is_visible());
}

#[test]
fn test_dialog_open_transitions_to_opening() {
    let mut dlg = Dialog::new("Title", "Content");
    dlg.handle(DialogMessage::Open);
    assert_eq!(dlg.state(), DialogState::Opening);
    assert!(dlg.is_visible());
}

#[test]
fn test_dialog_opening_to_open_after_animation() {
    let mut dlg = Dialog::new("Title", "Content");
    dlg.handle(DialogMessage::Open);
    assert_eq!(dlg.state(), DialogState::Opening);
    // 完成动画
    dlg.handle(DialogMessage::AnimationFinished);
    assert_eq!(dlg.state(), DialogState::Open);
}

#[test]
fn test_dialog_close_transitions_to_closing() {
    let mut dlg = Dialog::new("Title", "Content");
    dlg.handle(DialogMessage::Open);
    dlg.handle(DialogMessage::AnimationFinished); // 完成打开动画
    dlg.handle(DialogMessage::Close);
    assert_eq!(dlg.state(), DialogState::Closing);
}

#[test]
fn test_dialog_closing_to_closed_after_animation() {
    let mut dlg = Dialog::new("Title", "Content");
    dlg.handle(DialogMessage::Open);
    dlg.handle(DialogMessage::AnimationFinished);
    dlg.handle(DialogMessage::Close);
    dlg.handle(DialogMessage::AnimationFinished);
    assert_eq!(dlg.state(), DialogState::Closed);
    assert!(!dlg.is_visible());
}

#[test]
fn test_dialog_escape_closes_when_open() {
    let mut dlg = Dialog::new("Title", "Content");
    dlg.handle(DialogMessage::Open);
    dlg.handle(DialogMessage::AnimationFinished);
    dlg.handle(DialogMessage::EscapePressed);
    assert_eq!(dlg.state(), DialogState::Closing);
}

#[test]
fn test_dialog_overlay_click_closes_when_close_on_click_modal() {
    let mut dlg = Dialog::new("Title", "Content");
    dlg.handle(DialogMessage::Open);
    dlg.handle(DialogMessage::AnimationFinished);
    dlg.handle(DialogMessage::OverlayClicked);
    assert_eq!(dlg.state(), DialogState::Closing);
}

#[test]
fn test_dialog_overlay_click_blocked_when_close_on_click_modal_false() {
    let mut dlg = Dialog::new("Title", "Content")
        .with_props(DialogProps::new().with_close_on_click_modal(false));
    dlg.handle(DialogMessage::Open);
    dlg.handle(DialogMessage::AnimationFinished);
    dlg.handle(DialogMessage::OverlayClicked);
    // 不应关闭
    assert_eq!(dlg.state(), DialogState::Open);
}

#[test]
fn test_dialog_escape_blocked_when_close_on_press_escape_false() {
    let mut dlg = Dialog::new("Title", "Content")
        .with_props(DialogProps::new().with_close_on_press_escape(false));
    dlg.handle(DialogMessage::Open);
    dlg.handle(DialogMessage::AnimationFinished);
    dlg.handle(DialogMessage::EscapePressed);
    assert_eq!(dlg.state(), DialogState::Open);
}

#[test]
fn test_dialog_fullscreen_mode() {
    let dlg = Dialog::new("Title", "Content").with_props(DialogProps::new().with_fullscreen(true));
    assert!(dlg.props().fullscreen);
}

#[test]
fn test_dialog_draggable() {
    let dlg = Dialog::new("Title", "Content").with_props(DialogProps::new().with_draggable(true));
    assert!(dlg.props().draggable);
}

#[test]
fn test_dialog_drag_updates_position() {
    let mut dlg =
        Dialog::new("Title", "Content").with_props(DialogProps::new().with_draggable(true));
    dlg.handle(DialogMessage::Open);
    dlg.handle(DialogMessage::AnimationFinished);
    // 拖拽到新位置
    dlg.handle(DialogMessage::Dragged(100.0, 50.0));
    assert_eq!(dlg.position(), Some((100.0, 50.0)));
}

#[test]
fn test_dialog_drag_blocked_when_not_draggable() {
    let mut dlg = Dialog::new("Title", "Content"); // 默认 draggable=false
    dlg.handle(DialogMessage::Open);
    dlg.handle(DialogMessage::AnimationFinished);
    dlg.handle(DialogMessage::Dragged(100.0, 50.0));
    assert_eq!(dlg.position(), None);
}

#[test]
fn test_dialog_close_blocked_when_not_visible() {
    let mut dlg = Dialog::new("Title", "Content");
    // 在 Closed 状态下 Close 不应触发 Closing
    dlg.handle(DialogMessage::Close);
    assert_eq!(dlg.state(), DialogState::Closed);
}

#[test]
fn test_dialog_title_and_content() {
    let dlg = Dialog::new("My Title", "My Content");
    assert_eq!(dlg.title(), "My Title");
    assert_eq!(dlg.content(), "My Content");
}

#[test]
fn test_dialog_props_default() {
    let p = DialogProps::default();
    assert!(p.close_on_click_modal);
    assert!(p.close_on_press_escape);
    assert!(!p.fullscreen);
    assert!(!p.draggable);
}

#[test]
fn test_dialog_state_variants() {
    let states = [
        DialogState::Closed,
        DialogState::Opening,
        DialogState::Open,
        DialogState::Closing,
    ];
    assert_eq!(states.len(), 4);
    // 状态互不相等
    for i in 0..states.len() {
        for j in (i + 1)..states.len() {
            assert_ne!(states[i], states[j]);
        }
    }
}

#[test]
fn test_dialog_reset_clears_position() {
    let mut dlg =
        Dialog::new("Title", "Content").with_props(DialogProps::new().with_draggable(true));
    dlg.handle(DialogMessage::Open);
    dlg.handle(DialogMessage::AnimationFinished);
    dlg.handle(DialogMessage::Dragged(50.0, 50.0));
    dlg.handle(DialogMessage::Close);
    dlg.handle(DialogMessage::AnimationFinished);
    // 关闭后位置应清除
    assert_eq!(dlg.position(), None);
}
