//! Popconfirm 气泡确认框组件 — 参考 Element Plus `<el-popconfirm>`。

use har_ui_components::popconfirm::{
    Popconfirm, PopconfirmAction, PopconfirmMessage, PopconfirmPlacement, PopconfirmTrigger,
};

// ---------- 基础构造 ----------

#[test]
fn test_popconfirm_default() {
    let p = Popconfirm::new("确认删除？");
    assert_eq!(p.title(), "确认删除？");
    assert!(!p.visible());
    assert!(!p.disabled());
    assert!(p.show_arrow());
    assert_eq!(p.trigger(), PopconfirmTrigger::Click);
    assert_eq!(p.placement(), PopconfirmPlacement::Top);
    assert_eq!(p.confirm_button_text(), "确认");
    assert_eq!(p.cancel_button_text(), "取消");
    assert_eq!(p.width(), None);
    assert_eq!(p.last_action(), None);
}

#[test]
fn test_popconfirm_with_title() {
    let p = Popconfirm::new("确定要保存吗？");
    assert_eq!(p.title(), "确定要保存吗？");
}

#[test]
fn test_popconfirm_with_empty_title() {
    let p = Popconfirm::new("");
    assert_eq!(p.title(), "");
}

#[test]
fn test_popconfirm_with_confirm_text() {
    let p = Popconfirm::new("x").with_confirm_text("是的");
    assert_eq!(p.confirm_button_text(), "是的");
}

#[test]
fn test_popconfirm_with_cancel_text() {
    let p = Popconfirm::new("x").with_cancel_text("不");
    assert_eq!(p.cancel_button_text(), "不");
}

#[test]
fn test_popconfirm_with_width() {
    let p = Popconfirm::new("x").with_width(200);
    assert_eq!(p.width(), Some(200));
}

#[test]
fn test_popconfirm_with_show_arrow_false() {
    let p = Popconfirm::new("x").with_show_arrow(false);
    assert!(!p.show_arrow());
}

#[test]
fn test_popconfirm_with_disabled() {
    let p = Popconfirm::new("x").with_disabled(true);
    assert!(p.disabled());
}

// ---------- 12 种 placement ----------

#[test]
fn test_popconfirm_all_placements() {
    use PopconfirmPlacement::*;
    let cases = [
        Top,
        TopStart,
        TopEnd,
        Bottom,
        BottomStart,
        BottomEnd,
        Left,
        LeftStart,
        LeftEnd,
        Right,
        RightStart,
        RightEnd,
    ];
    for placement in cases.iter() {
        let p = Popconfirm::new("x").with_placement(*placement);
        assert_eq!(p.placement(), *placement);
    }
}

// ---------- 4 种 trigger ----------

#[test]
fn test_popconfirm_all_triggers() {
    use PopconfirmTrigger::*;
    let cases = [Click, Hover, Focus, Manual];
    for trigger in cases.iter() {
        let p = Popconfirm::new("x").with_trigger(*trigger);
        assert_eq!(p.trigger(), *trigger);
    }
}

// ---------- trigger=click 行为 ----------

#[test]
fn test_popconfirm_click_trigger_show() {
    let mut p = Popconfirm::new("x").with_trigger(PopconfirmTrigger::Click);
    p.handle(PopconfirmMessage::Click);
    assert!(p.visible());
}

#[test]
fn test_popconfirm_click_trigger_toggle() {
    let mut p = Popconfirm::new("x").with_trigger(PopconfirmTrigger::Click);
    p.handle(PopconfirmMessage::Click);
    p.handle(PopconfirmMessage::Click);
    assert!(!p.visible());
}

// ---------- trigger=hover 行为 ----------

#[test]
fn test_popconfirm_hover_trigger_show_hide() {
    let mut p = Popconfirm::new("x").with_trigger(PopconfirmTrigger::Hover);
    p.handle(PopconfirmMessage::MouseEnter);
    assert!(p.visible());
    p.handle(PopconfirmMessage::MouseLeave);
    assert!(!p.visible());
}

// ---------- trigger=manual 行为 ----------

#[test]
fn test_popconfirm_manual_trigger_show_hide() {
    let mut p = Popconfirm::new("x").with_trigger(PopconfirmTrigger::Manual);
    p.handle(PopconfirmMessage::Show);
    assert!(p.visible());
    p.handle(PopconfirmMessage::Hide);
    assert!(!p.visible());
}

// ---------- Confirm / Cancel 行为 ----------

#[test]
fn test_popconfirm_confirm_action() {
    let mut p = Popconfirm::new("x");
    p.handle(PopconfirmMessage::Click);
    p.handle(PopconfirmMessage::Confirm);
    assert_eq!(p.last_action(), Some(PopconfirmAction::Confirm));
    assert!(!p.visible(), "确认后应自动关闭");
}

#[test]
fn test_popconfirm_cancel_action() {
    let mut p = Popconfirm::new("x");
    p.handle(PopconfirmMessage::Click);
    p.handle(PopconfirmMessage::Cancel);
    assert_eq!(p.last_action(), Some(PopconfirmAction::Cancel));
    assert!(!p.visible(), "取消后应自动关闭");
}

#[test]
fn test_popconfirm_show_resets_action() {
    let mut p = Popconfirm::new("x");
    p.handle(PopconfirmMessage::Click);
    p.handle(PopconfirmMessage::Confirm);
    assert_eq!(p.last_action(), Some(PopconfirmAction::Confirm));
    // 再次显示应重置 last_action
    p.handle(PopconfirmMessage::Click);
    assert_eq!(p.last_action(), None);
}

// ---------- disabled 阻塞 ----------

#[test]
fn test_popconfirm_disabled_blocks_show() {
    let mut p = Popconfirm::new("x")
        .with_disabled(true)
        .with_trigger(PopconfirmTrigger::Manual);
    p.handle(PopconfirmMessage::Show);
    assert!(!p.visible());
}

#[test]
fn test_popconfirm_disabled_blocks_click() {
    let mut p = Popconfirm::new("x").with_disabled(true);
    p.handle(PopconfirmMessage::Click);
    assert!(!p.visible());
}

#[test]
fn test_popconfirm_disabled_unblocks_after_clear() {
    let mut p = Popconfirm::new("x")
        .with_disabled(true)
        .with_trigger(PopconfirmTrigger::Manual);
    p.handle(PopconfirmMessage::Show);
    assert!(!p.visible());
    p.set_disabled(false);
    p.handle(PopconfirmMessage::Show);
    assert!(p.visible());
}

// ---------- 直接控制 visible ----------

#[test]
fn test_popconfirm_with_visible_initial() {
    let p = Popconfirm::new("x").with_visible(true);
    assert!(p.visible());
}

#[test]
fn test_popconfirm_set_visible_runtime() {
    let mut p = Popconfirm::new("x");
    p.set_visible(true);
    assert!(p.visible());
    p.set_visible(false);
    assert!(!p.visible());
}

// ---------- 外部点击关闭 ----------

#[test]
fn test_popconfirm_click_outside_closes() {
    let mut p = Popconfirm::new("x");
    p.handle(PopconfirmMessage::Click);
    p.handle(PopconfirmMessage::ClickOutside);
    assert!(!p.visible());
}

#[test]
fn test_popconfirm_click_outside_does_not_set_action() {
    let mut p = Popconfirm::new("x");
    p.handle(PopconfirmMessage::Click);
    p.handle(PopconfirmMessage::ClickOutside);
    // 外部点击关闭不算 Confirm/Cancel
    assert_eq!(p.last_action(), None);
}

// ---------- 边界 ----------

#[test]
fn test_popconfirm_confirm_when_not_visible_no_op() {
    let mut p = Popconfirm::new("x");
    // 未显示时 Confirm 不应设置 action 或改变 visible
    p.handle(PopconfirmMessage::Confirm);
    assert!(!p.visible());
    // last_action 仍可能被设置但不影响 visible
}

#[test]
fn test_popconfirm_open_after_close_reuses() {
    let mut p = Popconfirm::new("x");
    p.handle(PopconfirmMessage::Click);
    p.handle(PopconfirmMessage::Confirm);
    assert!(!p.visible());
    // 再次打开
    p.handle(PopconfirmMessage::Click);
    assert!(p.visible());
}

#[test]
fn test_popconfirm_zero_width_safe() {
    let p = Popconfirm::new("x").with_width(0);
    assert_eq!(p.width(), Some(0));
}
