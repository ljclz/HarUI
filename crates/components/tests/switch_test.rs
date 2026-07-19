//! Switch 开关组件 — 参考 Element Plus `<el-switch>`。

use har_ui_components::switch::{Switch, SwitchMessage, SwitchValue};

// ---------- 基础构造 ----------

#[test]
fn test_switch_default_is_off() {
    let s = Switch::new();
    assert!(!s.is_on());
    assert!(!s.disabled());
    assert!(!s.loading());
}

#[test]
fn test_switch_new_with_value_on() {
    let s = Switch::with_value(true);
    assert!(s.is_on());
}

#[test]
fn test_switch_width_default() {
    let s = Switch::new();
    // Element Plus 默认宽度 40px
    assert_eq!(s.width(), 40);
}

#[test]
fn test_switch_with_width() {
    let s = Switch::new().with_width(60);
    assert_eq!(s.width(), 60);
}

// ---------- 开/关切换 ----------

#[test]
fn test_switch_toggle_off_to_on() {
    let mut s = Switch::new();
    assert!(!s.is_on());
    s.handle(SwitchMessage::Toggle);
    assert!(s.is_on());
}

#[test]
fn test_switch_toggle_on_to_off() {
    let mut s = Switch::with_value(true);
    assert!(s.is_on());
    s.handle(SwitchMessage::Toggle);
    assert!(!s.is_on());
}

#[test]
fn test_switch_multiple_toggle_stays_correct() {
    let mut s = Switch::new();
    for i in 0..10 {
        s.handle(SwitchMessage::Toggle);
        // 奇数次后为开，偶数次后为关
        let expected = i % 2 == 0;
        assert_eq!(s.is_on(), expected, "toggle #{}", i + 1);
    }
}

#[test]
fn test_switch_set_value_directly() {
    let mut s = Switch::new();
    s.handle(SwitchMessage::SetValue(true));
    assert!(s.is_on());
    s.handle(SwitchMessage::SetValue(false));
    assert!(!s.is_on());
}

// ---------- disabled ----------

#[test]
fn test_switch_disabled_blocks_toggle() {
    let mut s = Switch::new().with_disabled(true);
    s.handle(SwitchMessage::Toggle);
    assert!(!s.is_on(), "disabled 状态下不能切换");
}

#[test]
fn test_switch_disabled_blocks_set_value() {
    let mut s = Switch::with_value(true).with_disabled(true);
    s.handle(SwitchMessage::SetValue(false));
    assert!(s.is_on(), "disabled 状态下不能直接设置值");
}

#[test]
fn test_switch_with_disabled_getter() {
    let s = Switch::new().with_disabled(true);
    assert!(s.disabled());
}

// ---------- loading ----------

#[test]
fn test_switch_loading_blocks_toggle() {
    let mut s = Switch::new().with_loading(true);
    s.handle(SwitchMessage::Toggle);
    assert!(!s.is_on(), "loading 状态下不能切换");
}

#[test]
fn test_switch_with_loading_getter() {
    let s = Switch::new().with_loading(true);
    assert!(s.loading());
}

#[test]
fn test_switch_loading_unblocks_after_clear() {
    let mut s = Switch::new().with_loading(true);
    s.handle(SwitchMessage::Toggle);
    assert!(!s.is_on());
    // 清除 loading 后可切换
    s.set_loading(false);
    s.handle(SwitchMessage::Toggle);
    assert!(s.is_on());
}

// ---------- active-color / inactive-color ----------

#[test]
fn test_switch_default_colors() {
    let s = Switch::new();
    // 默认激活色为 primary 蓝色 #409EFF
    assert_eq!(s.active_color(), "#409EFF");
    // 默认未激活色为灰色 #C0CCDA
    assert_eq!(s.inactive_color(), "#C0CCDA");
}

#[test]
fn test_switch_with_active_color() {
    let s = Switch::new().with_active_color("#13CE66");
    assert_eq!(s.active_color(), "#13CE66");
}

#[test]
fn test_switch_with_inactive_color() {
    let s = Switch::new().with_inactive_color("#FF4949");
    assert_eq!(s.inactive_color(), "#FF4949");
}

// ---------- 文本描述 ----------

#[test]
fn test_switch_default_no_text() {
    let s = Switch::new();
    assert_eq!(s.active_text(), None);
    assert_eq!(s.inactive_text(), None);
}

#[test]
fn test_switch_with_active_text() {
    let s = Switch::new().with_active_text("启用");
    assert_eq!(s.active_text(), Some("启用"));
}

#[test]
fn test_switch_with_inactive_text() {
    let s = Switch::new().with_inactive_text("停用");
    assert_eq!(s.inactive_text(), Some("停用"));
}

#[test]
fn test_switch_current_text_off() {
    let s = Switch::new()
        .with_active_text("启用")
        .with_inactive_text("停用");
    assert_eq!(s.current_text(), Some("停用"));
}

#[test]
fn test_switch_current_text_on() {
    let s = Switch::with_value(true)
        .with_active_text("启用")
        .with_inactive_text("停用");
    assert_eq!(s.current_text(), Some("启用"));
}

#[test]
fn test_switch_current_text_none_when_no_text() {
    let s = Switch::new();
    assert_eq!(s.current_text(), None);
    let s2 = Switch::with_value(true);
    assert_eq!(s2.current_text(), None);
}

// ---------- active-value / inactive-value（自定义值） ----------

#[test]
fn test_switch_default_bool_values() {
    let s = Switch::new();
    assert_eq!(s.value(), SwitchValue::Bool(false));
    let s2 = Switch::with_value(true);
    assert_eq!(s2.value(), SwitchValue::Bool(true));
}

#[test]
fn test_switch_custom_active_value_number() {
    // 用 1/0 作为自定义激活/未激活值
    let s = Switch::new()
        .with_active_value(SwitchValue::Int(1))
        .with_inactive_value(SwitchValue::Int(0));
    assert_eq!(s.value(), SwitchValue::Int(0));
    assert!(!s.is_on());
}

#[test]
fn test_switch_custom_active_value_toggle() {
    let mut s = Switch::new()
        .with_active_value(SwitchValue::Int(1))
        .with_inactive_value(SwitchValue::Int(0));
    s.handle(SwitchMessage::Toggle);
    assert_eq!(s.value(), SwitchValue::Int(1));
    assert!(s.is_on());
}

// ---------- 边界 ----------

#[test]
fn test_switch_disabled_and_loading_combined() {
    // 同时 disabled 和 loading 时仍然不能切换
    let mut s = Switch::new().with_disabled(true).with_loading(true);
    s.handle(SwitchMessage::Toggle);
    assert!(!s.is_on());
}

#[test]
fn test_switch_set_disabled_at_runtime() {
    let mut s = Switch::new();
    // 切换前动态禁用
    s.set_disabled(true);
    s.handle(SwitchMessage::Toggle);
    assert!(!s.is_on());
    // 解除禁用后可切换
    s.set_disabled(false);
    s.handle(SwitchMessage::Toggle);
    assert!(s.is_on());
}
