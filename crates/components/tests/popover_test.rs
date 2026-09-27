//! Popover 气泡组件 — 参考 Element Plus `<el-popover>`。

use har_ui_components::popover::{Popover, PopoverMessage, PopoverPlacement, PopoverTrigger};

// ---------- 基础构造 ----------

#[test]
fn test_popover_default() {
    let p = Popover::new();
    assert!(!p.visible());
    assert!(!p.disabled());
    assert!(p.show_arrow());
    assert_eq!(p.trigger(), PopoverTrigger::Click);
    assert_eq!(p.placement(), PopoverPlacement::Bottom);
    assert_eq!(p.title(), None);
    assert_eq!(p.content(), None);
    assert_eq!(p.width(), None);
}

#[test]
fn test_popover_with_title() {
    let p = Popover::new().with_title("标题");
    assert_eq!(p.title(), Some("标题"));
}

#[test]
fn test_popover_with_content() {
    let p = Popover::new().with_content("内容");
    assert_eq!(p.content(), Some("内容"));
}

#[test]
fn test_popover_with_width() {
    let p = Popover::new().with_width(200);
    assert_eq!(p.width(), Some(200));
}

#[test]
fn test_popover_with_show_arrow_false() {
    let p = Popover::new().with_show_arrow(false);
    assert!(!p.show_arrow());
}

#[test]
fn test_popover_with_disabled() {
    let p = Popover::new().with_disabled(true);
    assert!(p.disabled());
}

// ---------- 12 种 placement ----------

#[test]
fn test_popover_all_placements() {
    use PopoverPlacement::*;
    let cases = [
        (Top, "top"),
        (TopStart, "top-start"),
        (TopEnd, "top-end"),
        (Bottom, "bottom"),
        (BottomStart, "bottom-start"),
        (BottomEnd, "bottom-end"),
        (Left, "left"),
        (LeftStart, "left-start"),
        (LeftEnd, "left-end"),
        (Right, "right"),
        (RightStart, "right-start"),
        (RightEnd, "right-end"),
    ];
    for (placement, label) in cases.iter() {
        let p = Popover::new().with_placement(*placement);
        assert_eq!(p.placement(), *placement, "placement: {}", label);
    }
}

#[test]
fn test_popover_placement_as_str() {
    use PopoverPlacement::*;
    assert_eq!(Top.as_str(), "top");
    assert_eq!(TopStart.as_str(), "top-start");
    assert_eq!(BottomEnd.as_str(), "bottom-end");
    assert_eq!(LeftStart.as_str(), "left-start");
    assert_eq!(RightEnd.as_str(), "right-end");
}

// ---------- 4 种 trigger ----------

#[test]
fn test_popover_all_triggers() {
    use PopoverTrigger::*;
    let cases = [Click, Hover, Focus, Manual];
    for trigger in cases.iter() {
        let p = Popover::new().with_trigger(*trigger);
        assert_eq!(p.trigger(), *trigger);
    }
}

// ---------- trigger=click 行为 ----------

#[test]
fn test_popover_click_trigger_show_hide() {
    let mut p = Popover::new().with_trigger(PopoverTrigger::Click);
    assert!(!p.visible());
    p.handle(PopoverMessage::Click);
    assert!(p.visible());
    // 再次点击隐藏
    p.handle(PopoverMessage::Click);
    assert!(!p.visible());
}

#[test]
fn test_popover_click_trigger_ignored_for_hover() {
    let mut p = Popover::new().with_trigger(PopoverTrigger::Hover);
    p.handle(PopoverMessage::Click);
    // Hover trigger 不响应 Click
    assert!(!p.visible());
}

// ---------- trigger=hover 行为 ----------

#[test]
fn test_popover_hover_trigger_enter_leave() {
    let mut p = Popover::new().with_trigger(PopoverTrigger::Hover);
    p.handle(PopoverMessage::MouseEnter);
    assert!(p.visible());
    p.handle(PopoverMessage::MouseLeave);
    assert!(!p.visible());
}

#[test]
fn test_popover_hover_trigger_ignored_for_click() {
    let mut p = Popover::new().with_trigger(PopoverTrigger::Click);
    p.handle(PopoverMessage::MouseEnter);
    // Click trigger 不响应 MouseEnter
    assert!(!p.visible());
}

// ---------- trigger=focus 行为 ----------

#[test]
fn test_popover_focus_trigger_show_hide() {
    let mut p = Popover::new().with_trigger(PopoverTrigger::Focus);
    p.handle(PopoverMessage::Focus);
    assert!(p.visible());
    p.handle(PopoverMessage::Blur);
    assert!(!p.visible());
}

// ---------- trigger=manual 行为 ----------

#[test]
fn test_popover_manual_trigger_show_hide() {
    let mut p = Popover::new().with_trigger(PopoverTrigger::Manual);
    p.handle(PopoverMessage::Show);
    assert!(p.visible());
    p.handle(PopoverMessage::Hide);
    assert!(!p.visible());
}

#[test]
fn test_popover_manual_trigger_ignores_click() {
    let mut p = Popover::new().with_trigger(PopoverTrigger::Manual);
    p.handle(PopoverMessage::Click);
    // Manual 模式不响应 Click，需要显式 Show/Hide
    assert!(!p.visible());
}

// ---------- disabled 阻塞 ----------

#[test]
fn test_popover_disabled_blocks_show() {
    let mut p = Popover::new()
        .with_disabled(true)
        .with_trigger(PopoverTrigger::Manual);
    p.handle(PopoverMessage::Show);
    assert!(!p.visible(), "disabled 状态下不能显示");
}

#[test]
fn test_popover_disabled_blocks_click() {
    let mut p = Popover::new()
        .with_disabled(true)
        .with_trigger(PopoverTrigger::Click);
    p.handle(PopoverMessage::Click);
    assert!(!p.visible());
}

#[test]
fn test_popover_disabled_unblocks_after_clear() {
    let mut p = Popover::new()
        .with_disabled(true)
        .with_trigger(PopoverTrigger::Manual);
    p.handle(PopoverMessage::Show);
    assert!(!p.visible());
    p.set_disabled(false);
    p.handle(PopoverMessage::Show);
    assert!(p.visible());
}

// ---------- 外部点击关闭 ----------

#[test]
fn test_popover_click_outside_closes() {
    let mut p = Popover::new().with_trigger(PopoverTrigger::Click);
    p.handle(PopoverMessage::Click);
    assert!(p.visible());
    p.handle(PopoverMessage::ClickOutside);
    assert!(!p.visible());
}

#[test]
fn test_popover_click_outside_ignored_for_manual() {
    let mut p = Popover::new()
        .with_trigger(PopoverTrigger::Manual)
        .with_visible(true);
    p.handle(PopoverMessage::ClickOutside);
    // Manual 模式下外部点击不关闭
    assert!(p.visible());
}

// ---------- 直接设置 visible ----------

#[test]
fn test_popover_with_visible_initial() {
    let p = Popover::new().with_visible(true);
    assert!(p.visible());
}

#[test]
fn test_popover_set_visible_runtime() {
    let mut p = Popover::new();
    p.set_visible(true);
    assert!(p.visible());
    p.set_visible(false);
    assert!(!p.visible());
}

// ---------- 边界 ----------

#[test]
fn test_popover_empty_title_content_safe() {
    let p = Popover::new().with_title("").with_content("");
    assert_eq!(p.title(), Some(""));
    assert_eq!(p.content(), Some(""));
}

#[test]
fn test_popover_zero_width_safe() {
    let p = Popover::new().with_width(0);
    assert_eq!(p.width(), Some(0));
}
