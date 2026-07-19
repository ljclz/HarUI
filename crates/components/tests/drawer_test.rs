//! Drawer 抽屉组件 — 参考 Element Plus `<el-drawer>`。

use har_ui_components::drawer::{Drawer, DrawerDirection, DrawerMessage, DrawerState};

// ---------- 基础构造 ----------

#[test]
fn test_drawer_default() {
    let d = Drawer::new();
    assert!(!d.visible());
    assert_eq!(d.state(), DrawerState::Closed);
    assert_eq!(d.title(), None);
    assert_eq!(d.direction(), DrawerDirection::Rtl);
    assert_eq!(d.size(), "30%");
    assert!(d.show_close());
    assert!(d.modal());
    assert!(d.close_on_click_modal());
    assert!(d.close_on_press_escape());
    assert!(!d.destroy_on_close());
}

#[test]
fn test_drawer_with_title() {
    let d = Drawer::new().with_title("设置");
    assert_eq!(d.title(), Some("设置"));
}

#[test]
fn test_drawer_with_size() {
    let d = Drawer::new().with_size("50%");
    assert_eq!(d.size(), "50%");
}

#[test]
fn test_drawer_with_size_px() {
    let d = Drawer::new().with_size("400px");
    assert_eq!(d.size(), "400px");
}

#[test]
fn test_drawer_with_show_close_false() {
    let d = Drawer::new().with_show_close(false);
    assert!(!d.show_close());
}

#[test]
fn test_drawer_with_modal_false() {
    let d = Drawer::new().with_modal(false);
    assert!(!d.modal());
}

#[test]
fn test_drawer_with_close_on_click_modal_false() {
    let d = Drawer::new().with_close_on_click_modal(false);
    assert!(!d.close_on_click_modal());
}

#[test]
fn test_drawer_with_close_on_press_escape_false() {
    let d = Drawer::new().with_close_on_press_escape(false);
    assert!(!d.close_on_press_escape());
}

#[test]
fn test_drawer_with_destroy_on_close() {
    let d = Drawer::new().with_destroy_on_close(true);
    assert!(d.destroy_on_close());
}

// ---------- 4 种 direction ----------

#[test]
fn test_drawer_all_directions() {
    use DrawerDirection::*;
    let cases = [Rtl, Ltr, Ttb, Btt];
    for dir in cases.iter() {
        let d = Drawer::new().with_direction(*dir);
        assert_eq!(d.direction(), *dir);
    }
}

#[test]
fn test_drawer_direction_as_str() {
    use DrawerDirection::*;
    assert_eq!(Rtl.as_str(), "rtl");
    assert_eq!(Ltr.as_str(), "ltr");
    assert_eq!(Ttb.as_str(), "ttb");
    assert_eq!(Btt.as_str(), "btt");
}

// ---------- 状态机：Closed → Opening → Open → Closing → Closed ----------

#[test]
fn test_drawer_open_transitions_to_opening() {
    let mut d = Drawer::new();
    assert_eq!(d.state(), DrawerState::Closed);
    d.handle(DrawerMessage::Open);
    assert_eq!(d.state(), DrawerState::Opening);
}

#[test]
fn test_drawer_finish_opening_transitions_to_open() {
    let mut d = Drawer::new();
    d.handle(DrawerMessage::Open);
    d.handle(DrawerMessage::AnimationEnd);
    assert_eq!(d.state(), DrawerState::Open);
    assert!(d.visible());
}

#[test]
fn test_drawer_close_transitions_to_closing() {
    let mut d = Drawer::new();
    d.handle(DrawerMessage::Open);
    d.handle(DrawerMessage::AnimationEnd);
    d.handle(DrawerMessage::Close);
    assert_eq!(d.state(), DrawerState::Closing);
}

#[test]
fn test_drawer_finish_closing_transitions_to_closed() {
    let mut d = Drawer::new();
    d.handle(DrawerMessage::Open);
    d.handle(DrawerMessage::AnimationEnd);
    d.handle(DrawerMessage::Close);
    d.handle(DrawerMessage::AnimationEnd);
    assert_eq!(d.state(), DrawerState::Closed);
    assert!(!d.visible());
}

#[test]
fn test_drawer_full_open_close_cycle() {
    let mut d = Drawer::new();
    // Closed → Opening → Open → Closing → Closed
    assert_eq!(d.state(), DrawerState::Closed);
    d.handle(DrawerMessage::Open);
    assert_eq!(d.state(), DrawerState::Opening);
    d.handle(DrawerMessage::AnimationEnd);
    assert_eq!(d.state(), DrawerState::Open);
    d.handle(DrawerMessage::Close);
    assert_eq!(d.state(), DrawerState::Closing);
    d.handle(DrawerMessage::AnimationEnd);
    assert_eq!(d.state(), DrawerState::Closed);
}

// ---------- 可见性 ----------

#[test]
fn test_drawer_visible_during_opening_and_open() {
    let mut d = Drawer::new();
    d.handle(DrawerMessage::Open);
    assert!(d.visible(), "Opening 期间应可见");
    d.handle(DrawerMessage::AnimationEnd);
    assert!(d.visible(), "Open 状态应可见");
}

#[test]
fn test_drawer_visible_during_closing_but_not_closed() {
    let mut d = Drawer::new();
    d.handle(DrawerMessage::Open);
    d.handle(DrawerMessage::AnimationEnd);
    d.handle(DrawerMessage::Close);
    assert!(d.visible(), "Closing 期间应可见（动画未结束）");
    d.handle(DrawerMessage::AnimationEnd);
    assert!(!d.visible(), "Closed 后不可见");
}

// ---------- close_on_click_modal ----------

#[test]
fn test_drawer_click_modal_closes_when_enabled() {
    let mut d = Drawer::new();
    d.handle(DrawerMessage::Open);
    d.handle(DrawerMessage::AnimationEnd);
    d.handle(DrawerMessage::ClickModal);
    assert_eq!(d.state(), DrawerState::Closing);
}

#[test]
fn test_drawer_click_modal_ignored_when_disabled() {
    let mut d = Drawer::new().with_close_on_click_modal(false);
    d.handle(DrawerMessage::Open);
    d.handle(DrawerMessage::AnimationEnd);
    d.handle(DrawerMessage::ClickModal);
    assert_eq!(d.state(), DrawerState::Open, "禁用点击遮罩关闭时应保持打开");
}

#[test]
fn test_drawer_click_modal_ignored_when_no_modal() {
    let mut d = Drawer::new().with_modal(false);
    d.handle(DrawerMessage::Open);
    d.handle(DrawerMessage::AnimationEnd);
    d.handle(DrawerMessage::ClickModal);
    assert_eq!(d.state(), DrawerState::Open);
}

// ---------- close_on_press_escape ----------

#[test]
fn test_drawer_escape_closes_when_enabled() {
    let mut d = Drawer::new();
    d.handle(DrawerMessage::Open);
    d.handle(DrawerMessage::AnimationEnd);
    d.handle(DrawerMessage::PressEscape);
    assert_eq!(d.state(), DrawerState::Closing);
}

#[test]
fn test_drawer_escape_ignored_when_disabled() {
    let mut d = Drawer::new().with_close_on_press_escape(false);
    d.handle(DrawerMessage::Open);
    d.handle(DrawerMessage::AnimationEnd);
    d.handle(DrawerMessage::PressEscape);
    assert_eq!(d.state(), DrawerState::Open);
}

// ---------- close 按钮 ----------

#[test]
fn test_drawer_close_button_closes() {
    let mut d = Drawer::new();
    d.handle(DrawerMessage::Open);
    d.handle(DrawerMessage::AnimationEnd);
    d.handle(DrawerMessage::ClickClose);
    assert_eq!(d.state(), DrawerState::Closing);
}

// ---------- 边界：状态非法转换 ----------

#[test]
fn test_drawer_open_when_already_opening_no_op() {
    let mut d = Drawer::new();
    d.handle(DrawerMessage::Open);
    d.handle(DrawerMessage::Open); // 重复 Open 不应改变状态
    assert_eq!(d.state(), DrawerState::Opening);
}

#[test]
fn test_drawer_close_when_already_closed_no_op() {
    let mut d = Drawer::new();
    d.handle(DrawerMessage::Close); // 已 Closed 时 Close 不应改变状态
    assert_eq!(d.state(), DrawerState::Closed);
}

#[test]
fn test_drawer_animation_end_when_closed_no_op() {
    let mut d = Drawer::new();
    d.handle(DrawerMessage::AnimationEnd); // Closed 状态下 AnimationEnd 不应改变状态
    assert_eq!(d.state(), DrawerState::Closed);
}

#[test]
fn test_drawer_open_after_close_reuses() {
    let mut d = Drawer::new();
    d.handle(DrawerMessage::Open);
    d.handle(DrawerMessage::AnimationEnd);
    d.handle(DrawerMessage::Close);
    d.handle(DrawerMessage::AnimationEnd);
    // 重新打开
    d.handle(DrawerMessage::Open);
    assert_eq!(d.state(), DrawerState::Opening);
    d.handle(DrawerMessage::AnimationEnd);
    assert_eq!(d.state(), DrawerState::Open);
    assert!(d.visible());
}
