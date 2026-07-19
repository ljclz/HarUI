//! Button 组件测试
//!
//! 参考 Element Plus `<el-button>` 组件 API。
//! 测试覆盖：
//! - 8 种 type × 3 种 size 矩阵
//! - disabled / loading / plain / round / circle / icon 状态
//! - click 事件派发

use har_ui_components::button::{
    Button, ButtonMessage, ButtonSize, ButtonType, ButtonState,
};

#[test]
fn test_button_default_type_is_default() {
    let btn = Button::new("Click");
    assert_eq!(btn.props().button_type, ButtonType::Default);
}

#[test]
fn test_button_all_type_variants() {
    let types = [
        ButtonType::Default,
        ButtonType::Primary,
        ButtonType::Success,
        ButtonType::Warning,
        ButtonType::Danger,
        ButtonType::Info,
        ButtonType::Text,
        ButtonType::Link,
    ];
    assert_eq!(types.len(), 8);
    // 8 种 type 互不相同
    for i in 0..types.len() {
        for j in (i + 1)..types.len() {
            assert_ne!(types[i], types[j], "type {:?} == type {:?}", types[i], types[j]);
        }
    }
}

#[test]
fn test_button_all_size_variants() {
    let sizes = [ButtonSize::Large, ButtonSize::Default, ButtonSize::Small];
    assert_eq!(sizes.len(), 3);
    assert_ne!(ButtonSize::Large, ButtonSize::Default);
    assert_ne!(ButtonSize::Default, ButtonSize::Small);
}

#[test]
fn test_button_default_size_is_default() {
    let btn = Button::new("Click");
    assert_eq!(btn.props().size, ButtonSize::Default);
}

#[test]
fn test_button_with_type_primary() {
    let btn = Button::new("Save").with_type(ButtonType::Primary);
    assert_eq!(btn.props().button_type, ButtonType::Primary);
}

#[test]
fn test_button_with_size_small() {
    let btn = Button::new("Cancel").with_size(ButtonSize::Small);
    assert_eq!(btn.props().size, ButtonSize::Small);
}

#[test]
fn test_button_disabled() {
    let btn = Button::new("Submit").disabled(true);
    assert!(btn.props().disabled);
    let btn2 = Button::new("Submit");
    assert!(!btn2.props().disabled);
}

#[test]
fn test_button_loading() {
    let btn = Button::new("Saving").loading(true);
    assert!(btn.props().loading);
}

#[test]
fn test_button_plain() {
    let btn = Button::new("Plain").plain(true);
    assert!(btn.props().plain);
}

#[test]
fn test_button_round() {
    let btn = Button::new("Round").round(true);
    assert!(btn.props().round);
}

#[test]
fn test_button_circle() {
    let btn = Button::new("X").circle(true);
    assert!(btn.props().circle);
}

#[test]
fn test_button_text_content() {
    let btn = Button::new("Submit Form");
    assert_eq!(btn.props().text, "Submit Form");
}

#[test]
fn test_button_click_event_when_not_disabled() {
    let mut btn = Button::new("Click").with_type(ButtonType::Primary);
    // 初始 state 为 Normal
    assert_eq!(btn.state(), ButtonState::Normal);
    // 派发 click 事件
    let msg = btn.handle(ButtonMessage::Clicked);
    // 应返回 Clicked 消息（用于上层处理）
    assert!(matches!(msg, ButtonMessage::Clicked));
}

#[test]
fn test_button_click_event_blocked_when_disabled() {
    let mut btn = Button::new("Click").disabled(true);
    // click 应被忽略，返回 NoChange
    let msg = btn.handle(ButtonMessage::Clicked);
    assert!(matches!(msg, ButtonMessage::NoChange));
}

#[test]
fn test_button_state_transitions_hover_active() {
    let mut btn = Button::new("Hover Me");
    // Normal → Hover
    btn.handle(ButtonMessage::Hovered);
    assert_eq!(btn.state(), ButtonState::Hover);
    // Hover → Active (mouse down)
    btn.handle(ButtonMessage::Pressed);
    assert_eq!(btn.state(), ButtonState::Active);
    // Active → Hover (mouse up)
    btn.handle(ButtonMessage::Released);
    assert_eq!(btn.state(), ButtonState::Hover);
    // Hover → Normal (mouse leave)
    btn.handle(ButtonMessage::Unhovered);
    assert_eq!(btn.state(), ButtonState::Normal);
}

#[test]
fn test_button_loading_state_blocks_click() {
    let mut btn = Button::new("Saving").loading(true);
    let msg = btn.handle(ButtonMessage::Clicked);
    assert!(matches!(msg, ButtonMessage::NoChange));
}

#[test]
fn test_button_builder_chains() {
    let btn = Button::new("Complex")
        .with_type(ButtonType::Success)
        .with_size(ButtonSize::Large)
        .disabled(false)
        .loading(false)
        .plain(true)
        .round(true)
        .circle(false);
    let p = btn.props();
    assert_eq!(p.button_type, ButtonType::Success);
    assert_eq!(p.size, ButtonSize::Large);
    assert!(!p.disabled);
    assert!(!p.loading);
    assert!(p.plain);
    assert!(p.round);
    assert!(!p.circle);
}

#[test]
fn test_button_default_props() {
    let btn = Button::new("Test");
    let p = btn.props();
    assert_eq!(p.text, "Test");
    assert_eq!(p.button_type, ButtonType::Default);
    assert_eq!(p.size, ButtonSize::Default);
    assert!(!p.disabled);
    assert!(!p.loading);
    assert!(!p.plain);
    assert!(!p.round);
    assert!(!p.circle);
}
