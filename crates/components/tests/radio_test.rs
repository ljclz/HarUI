//! Radio 单选框组件 — 参考 Element Plus `<el-radio>` 与 `<el-radio-group>`。

use har_ui_components::radio::{Radio, RadioGroup, RadioMessage, RadioSize};

// ---------- 单个 Radio 基础 ----------

#[test]
fn test_radio_default_unchecked() {
    let r = Radio::new("apple");
    assert!(!r.checked());
    assert_eq!(r.label(), "apple");
    assert!(!r.disabled());
    assert_eq!(r.size(), RadioSize::Default);
    assert!(!r.border());
}

#[test]
fn test_radio_with_checked() {
    let r = Radio::new("apple").with_checked(true);
    assert!(r.checked());
}

#[test]
fn test_radio_with_disabled() {
    let r = Radio::new("apple").with_disabled(true);
    assert!(r.disabled());
}

#[test]
fn test_radio_with_size() {
    let r = Radio::new("apple").with_size(RadioSize::Large);
    assert_eq!(r.size(), RadioSize::Large);
}

#[test]
fn test_radio_with_border() {
    let r = Radio::new("apple").with_border(true);
    assert!(r.border());
}

// ---------- 单个 Radio 切换 ----------

#[test]
fn test_radio_set_checked() {
    let mut r = Radio::new("apple");
    r.handle(RadioMessage::SetChecked(true));
    assert!(r.checked());
}

#[test]
fn test_radio_disabled_blocks_set_checked() {
    let mut r = Radio::new("apple").with_disabled(true);
    r.handle(RadioMessage::SetChecked(true));
    assert!(!r.checked());
}

#[test]
fn test_radio_set_checked_off() {
    let mut r = Radio::new("apple").with_checked(true);
    r.handle(RadioMessage::SetChecked(false));
    assert!(!r.checked());
}

// ---------- RadioGroup 基础 ----------

#[test]
fn test_radio_group_default_empty() {
    let g = RadioGroup::new();
    assert_eq!(g.value(), None);
    assert!(!g.disabled());
    assert_eq!(g.size(), RadioSize::Default);
}

#[test]
fn test_radio_group_with_initial_value() {
    let g = RadioGroup::new().with_value("apple");
    assert_eq!(g.value(), Some("apple"));
}

#[test]
fn test_radio_group_with_disabled() {
    let g = RadioGroup::new().with_disabled(true);
    assert!(g.disabled());
}

#[test]
fn test_radio_group_with_size() {
    let g = RadioGroup::new().with_size(RadioSize::Small);
    assert_eq!(g.size(), RadioSize::Small);
}

// ---------- RadioGroup 互斥选择 ----------

#[test]
fn test_radio_group_set_value_replaces_previous() {
    let mut g = RadioGroup::new().with_value("apple");
    g.handle(RadioMessage::SetValue("banana".to_string()));
    // 互斥：只能保留一个值
    assert_eq!(g.value(), Some("banana"));
}

#[test]
fn test_radio_group_set_value_from_empty() {
    let mut g = RadioGroup::new();
    g.handle(RadioMessage::SetValue("apple".to_string()));
    assert_eq!(g.value(), Some("apple"));
}

#[test]
fn test_radio_group_clear_value() {
    let mut g = RadioGroup::new().with_value("apple");
    g.handle(RadioMessage::Clear);
    assert_eq!(g.value(), None);
}

// ---------- RadioGroup disabled ----------

#[test]
fn test_radio_group_disabled_blocks_set_value() {
    let mut g = RadioGroup::new().with_disabled(true);
    g.handle(RadioMessage::SetValue("apple".to_string()));
    assert_eq!(g.value(), None, "disabled 状态下不能修改");
}

#[test]
fn test_radio_group_disabled_blocks_clear() {
    let mut g = RadioGroup::new().with_value("apple").with_disabled(true);
    g.handle(RadioMessage::Clear);
    assert_eq!(g.value(), Some("apple"));
}

// ---------- RadioGroup 选项管理 ----------

#[test]
fn test_radio_group_is_checked() {
    let g = RadioGroup::new().with_value("apple");
    assert!(g.is_checked("apple"));
    assert!(!g.is_checked("banana"));
}

#[test]
fn test_radio_group_is_checked_empty() {
    let g = RadioGroup::new();
    assert!(!g.is_checked("apple"));
}

// ---------- 边界 ----------

#[test]
fn test_radio_group_set_empty_string_value() {
    let mut g = RadioGroup::new();
    g.handle(RadioMessage::SetValue(String::new()));
    assert_eq!(g.value(), Some(""));
}

#[test]
fn test_radio_group_toggle_option_convenience() {
    // 便捷方法：toggle 一个选项
    let mut g = RadioGroup::new();
    g.handle(RadioMessage::ToggleOption("apple".to_string()));
    assert_eq!(g.value(), Some("apple"));
    // 再次 toggle 同一选项 → 清空（参考 Element Plus 行为）
    g.handle(RadioMessage::ToggleOption("apple".to_string()));
    assert_eq!(g.value(), None);
}
