//! Tooltip 文字提示组件 — 参考 Element Plus `<el-tooltip>`。

use har_ui_components::tooltip::{
    Tooltip, TooltipEffect, TooltipMessage, TooltipPlacement, TooltipTrigger,
};

// ---------- 基础构造 ----------

#[test]
fn test_tooltip_default() {
    let t = Tooltip::new("提示内容");
    assert_eq!(t.content(), "提示内容");
    assert!(!t.visible());
    assert!(!t.disabled());
    assert!(t.show_arrow());
    assert!(t.enterable());
    assert_eq!(t.trigger(), TooltipTrigger::Hover);
    assert_eq!(t.effect(), TooltipEffect::Dark);
    assert_eq!(t.placement(), TooltipPlacement::Top);
    assert_eq!(t.hide_after(), None);
}

#[test]
fn test_tooltip_with_content() {
    let t = Tooltip::new("hello");
    assert_eq!(t.content(), "hello");
}

#[test]
fn test_tooltip_with_empty_content() {
    let t = Tooltip::new("");
    assert_eq!(t.content(), "");
}

#[test]
fn test_tooltip_with_placement() {
    use TooltipPlacement::*;
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
        let t = Tooltip::new("x").with_placement(*placement);
        assert_eq!(t.placement(), *placement);
    }
}

#[test]
fn test_tooltip_with_effect() {
    let t = Tooltip::new("x").with_effect(TooltipEffect::Light);
    assert_eq!(t.effect(), TooltipEffect::Light);
}

#[test]
fn test_tooltip_with_disabled() {
    let t = Tooltip::new("x").with_disabled(true);
    assert!(t.disabled());
}

#[test]
fn test_tooltip_with_show_arrow_false() {
    let t = Tooltip::new("x").with_show_arrow(false);
    assert!(!t.show_arrow());
}

#[test]
fn test_tooltip_with_enterable_false() {
    let t = Tooltip::new("x").with_enterable(false);
    assert!(!t.enterable());
}

#[test]
fn test_tooltip_with_hide_after() {
    let t = Tooltip::new("x").with_hide_after(3000);
    assert_eq!(t.hide_after(), Some(3000));
}

// ---------- 4 种 trigger 行为 ----------

#[test]
fn test_tooltip_hover_trigger_show_hide() {
    let mut t = Tooltip::new("x").with_trigger(TooltipTrigger::Hover);
    t.handle(TooltipMessage::MouseEnter);
    assert!(t.visible());
    t.handle(TooltipMessage::MouseLeave);
    assert!(!t.visible());
}

#[test]
fn test_tooltip_focus_trigger_show_hide() {
    let mut t = Tooltip::new("x").with_trigger(TooltipTrigger::Focus);
    t.handle(TooltipMessage::Focus);
    assert!(t.visible());
    t.handle(TooltipMessage::Blur);
    assert!(!t.visible());
}

#[test]
fn test_tooltip_click_trigger_show_hide() {
    let mut t = Tooltip::new("x").with_trigger(TooltipTrigger::Click);
    t.handle(TooltipMessage::Click);
    assert!(t.visible());
    t.handle(TooltipMessage::Click);
    assert!(!t.visible());
}

#[test]
fn test_tooltip_manual_trigger_show_hide() {
    let mut t = Tooltip::new("x").with_trigger(TooltipTrigger::Manual);
    t.handle(TooltipMessage::Show);
    assert!(t.visible());
    t.handle(TooltipMessage::Hide);
    assert!(!t.visible());
}

// ---------- trigger 隔离 ----------

#[test]
fn test_tooltip_hover_trigger_ignores_click() {
    let mut t = Tooltip::new("x").with_trigger(TooltipTrigger::Hover);
    t.handle(TooltipMessage::Click);
    assert!(!t.visible());
}

#[test]
fn test_tooltip_manual_trigger_ignores_hover() {
    let mut t = Tooltip::new("x").with_trigger(TooltipTrigger::Manual);
    t.handle(TooltipMessage::MouseEnter);
    assert!(!t.visible());
}

// ---------- disabled 阻塞 ----------

#[test]
fn test_tooltip_disabled_blocks_hover() {
    let mut t = Tooltip::new("x").with_disabled(true);
    t.handle(TooltipMessage::MouseEnter);
    assert!(!t.visible());
}

#[test]
fn test_tooltip_disabled_blocks_manual_show() {
    let mut t = Tooltip::new("x")
        .with_disabled(true)
        .with_trigger(TooltipTrigger::Manual);
    t.handle(TooltipMessage::Show);
    assert!(!t.visible());
}

#[test]
fn test_tooltip_disabled_unblocks_after_clear() {
    let mut t = Tooltip::new("x").with_disabled(true);
    t.handle(TooltipMessage::MouseEnter);
    assert!(!t.visible());
    t.set_disabled(false);
    t.handle(TooltipMessage::MouseEnter);
    assert!(t.visible());
}

// ---------- hide_after 自动关闭（用 tick 模拟时间推进） ----------

#[test]
fn test_tooltip_hide_after_auto_close() {
    let mut t = Tooltip::new("x").with_hide_after(3000);
    t.handle(TooltipMessage::MouseEnter);
    assert!(t.visible());
    // 推进 2999ms 还应可见
    t.tick(2999);
    assert!(t.visible());
    // 推进到 3000ms 应自动关闭
    t.tick(1);
    assert!(!t.visible());
}

#[test]
fn test_tooltip_hide_after_not_set_never_auto_close() {
    let mut t = Tooltip::new("x");
    t.handle(TooltipMessage::MouseEnter);
    assert!(t.visible());
    // 没有 hide_after，再大的 tick 也不关闭
    t.tick(1000000);
    assert!(t.visible());
}

#[test]
fn test_tooltip_hide_after_resets_on_show() {
    let mut t = Tooltip::new("x").with_hide_after(1000);
    t.handle(TooltipMessage::MouseEnter);
    t.tick(500);
    // 重新触发显示应重置计时器
    t.handle(TooltipMessage::MouseLeave);
    t.handle(TooltipMessage::MouseEnter);
    t.tick(900);
    assert!(t.visible(), "重新显示后计时器应重置");
}

// ---------- enterable 阻止关闭 ----------

#[test]
fn test_tooltip_enterable_blocks_leave() {
    // enterable=true 时，鼠标进入气泡不算 MouseLeave
    let mut t = Tooltip::new("x").with_enterable(true);
    t.handle(TooltipMessage::MouseEnter);
    assert!(t.visible());
    t.handle(TooltipMessage::EnterTooltip);
    t.handle(TooltipMessage::MouseLeave);
    // 鼠标在气泡内时不关闭
    assert!(t.visible());
    // 离开气泡后才关闭
    t.handle(TooltipMessage::LeaveTooltip);
    assert!(!t.visible());
}

#[test]
fn test_tooltip_not_enterable_leaves_immediately() {
    let mut t = Tooltip::new("x").with_enterable(false);
    t.handle(TooltipMessage::MouseEnter);
    t.handle(TooltipMessage::MouseLeave);
    // 非 enterable 时立即关闭
    assert!(!t.visible());
}

// ---------- 直接控制 visible ----------

#[test]
fn test_tooltip_with_visible_initial() {
    let t = Tooltip::new("x").with_visible(true);
    assert!(t.visible());
}

#[test]
fn test_tooltip_set_visible_runtime() {
    let mut t = Tooltip::new("x");
    t.set_visible(true);
    assert!(t.visible());
}

// ---------- 边界 ----------

#[test]
fn test_tooltip_hide_after_zero_immediate_close() {
    let mut t = Tooltip::new("x").with_hide_after(0);
    t.handle(TooltipMessage::MouseEnter);
    assert!(t.visible());
    // hide_after=0 表示立即关闭（任意 tick 后）
    t.tick(1);
    assert!(!t.visible());
}

#[test]
fn test_tooltip_disabled_does_not_block_set_visible() {
    // set_visible 是直接控制，不受 disabled 影响
    let mut t = Tooltip::new("x").with_disabled(true);
    t.set_visible(true);
    assert!(t.visible());
}
