//! Checkbox 多选框组件 — 参考 Element Plus `<el-checkbox>` 与 `<el-checkbox-group>`。

use har_ui_components::checkbox::{
    Checkbox, CheckboxGroup, CheckboxMessage, CheckboxSize,
};

// ---------- 单个 Checkbox 基础 ----------

#[test]
fn test_checkbox_default_unchecked() {
    let c = Checkbox::new("apple");
    assert!(!c.checked());
    assert_eq!(c.label(), "apple");
    assert!(!c.disabled());
    assert_eq!(c.size(), CheckboxSize::Default);
    assert!(!c.border());
    assert!(!c.indeterminate());
}

#[test]
fn test_checkbox_with_checked() {
    let c = Checkbox::new("apple").with_checked(true);
    assert!(c.checked());
}

#[test]
fn test_checkbox_with_disabled() {
    let c = Checkbox::new("apple").with_disabled(true);
    assert!(c.disabled());
}

#[test]
fn test_checkbox_with_size() {
    let c = Checkbox::new("apple").with_size(CheckboxSize::Large);
    assert_eq!(c.size(), CheckboxSize::Large);
}

#[test]
fn test_checkbox_with_border() {
    let c = Checkbox::new("apple").with_border(true);
    assert!(c.border());
}

// ---------- 单个 Checkbox 切换 ----------

#[test]
fn test_checkbox_toggle_unchecked_to_checked() {
    let mut c = Checkbox::new("apple");
    c.handle(CheckboxMessage::Toggle);
    assert!(c.checked());
}

#[test]
fn test_checkbox_toggle_checked_to_unchecked() {
    let mut c = Checkbox::new("apple").with_checked(true);
    c.handle(CheckboxMessage::Toggle);
    assert!(!c.checked());
}

#[test]
fn test_checkbox_disabled_blocks_toggle() {
    let mut c = Checkbox::new("apple").with_disabled(true);
    c.handle(CheckboxMessage::Toggle);
    assert!(!c.checked());
}

#[test]
fn test_checkbox_set_checked_directly() {
    let mut c = Checkbox::new("apple");
    c.handle(CheckboxMessage::SetChecked(true));
    assert!(c.checked());
    c.handle(CheckboxMessage::SetChecked(false));
    assert!(!c.checked());
}

// ---------- indeterminate 半选 ----------

#[test]
fn test_checkbox_indeterminate_default_off() {
    let c = Checkbox::new("apple");
    assert!(!c.indeterminate());
}

#[test]
fn test_checkbox_with_indeterminate() {
    let c = Checkbox::new("apple").with_indeterminate(true);
    assert!(c.indeterminate());
}

#[test]
fn test_checkbox_set_indeterminate_runtime() {
    let mut c = Checkbox::new("apple");
    c.set_indeterminate(true);
    assert!(c.indeterminate());
    c.set_indeterminate(false);
    assert!(!c.indeterminate());
}

// ---------- true-value / false-value 自定义值 ----------

#[test]
fn test_checkbox_custom_true_false_value() {
    let c = Checkbox::new("apple")
        .with_true_value("yes")
        .with_false_value("no");
    assert_eq!(c.value(), "no");
    assert!(!c.checked());
}

#[test]
fn test_checkbox_custom_true_false_value_checked() {
    let c = Checkbox::new("apple")
        .with_checked(true)
        .with_true_value("yes")
        .with_false_value("no");
    assert_eq!(c.value(), "yes");
    assert!(c.checked());
}

// ---------- CheckboxGroup 基础 ----------

#[test]
fn test_checkbox_group_default_empty() {
    let g = CheckboxGroup::new();
    assert!(g.value().is_empty());
    assert!(!g.disabled());
    assert_eq!(g.min(), None);
    assert_eq!(g.max(), None);
}

#[test]
fn test_checkbox_group_with_initial_values() {
    let g = CheckboxGroup::new()
        .with_value(vec!["apple", "banana"]);
    assert_eq!(g.value(), &["apple", "banana"]);
}

#[test]
fn test_checkbox_group_with_disabled() {
    let g = CheckboxGroup::new().with_disabled(true);
    assert!(g.disabled());
}

#[test]
fn test_checkbox_group_with_min_max() {
    let g = CheckboxGroup::new().with_min(1).with_max(3);
    assert_eq!(g.min(), Some(1));
    assert_eq!(g.max(), Some(3));
}

// ---------- CheckboxGroup 切换选项 ----------

#[test]
fn test_checkbox_group_toggle_add() {
    let mut g = CheckboxGroup::new();
    g.handle(CheckboxMessage::ToggleValue("apple".to_string()));
    assert_eq!(g.value(), &["apple"]);
}

#[test]
fn test_checkbox_group_toggle_remove() {
    let mut g = CheckboxGroup::new().with_value(vec!["apple", "banana"]);
    g.handle(CheckboxMessage::ToggleValue("apple".to_string()));
    assert_eq!(g.value(), &["banana"]);
}

#[test]
fn test_checkbox_group_toggle_does_not_duplicate() {
    let mut g = CheckboxGroup::new().with_value(vec!["apple"]);
    g.handle(CheckboxMessage::ToggleValue("apple".to_string()));
    // 再切回来应该移除
    assert!(g.value().is_empty());
    g.handle(CheckboxMessage::ToggleValue("apple".to_string()));
    assert_eq!(g.value(), &["apple"]);
}

// ---------- CheckboxGroup min/max 限制 ----------

#[test]
fn test_checkbox_group_max_blocks_add() {
    let mut g = CheckboxGroup::new()
        .with_value(vec!["apple", "banana"])
        .with_max(2);
    // 已达上限，再加应被拒绝
    g.handle(CheckboxMessage::ToggleValue("cherry".to_string()));
    assert_eq!(g.value().len(), 2);
    assert!(!g.value().iter().any(|v| v == "cherry"));
}

#[test]
fn test_checkbox_group_min_blocks_remove() {
    let mut g = CheckboxGroup::new()
        .with_value(vec!["apple", "banana"])
        .with_min(2);
    // 已达下限，移除应被拒绝
    g.handle(CheckboxMessage::ToggleValue("apple".to_string()));
    assert_eq!(g.value().len(), 2);
    assert!(g.value().iter().any(|v| v == "apple"));
}

// ---------- CheckboxGroup disabled ----------

#[test]
fn test_checkbox_group_disabled_blocks_toggle() {
    let mut g = CheckboxGroup::new().with_disabled(true);
    g.handle(CheckboxMessage::ToggleValue("apple".to_string()));
    assert!(g.value().is_empty(), "disabled 状态下不能修改");
}

// ---------- CheckboxGroup 全选/取消全选 ----------

#[test]
fn test_checkbox_group_select_all() {
    let mut g = CheckboxGroup::new();
    g.handle(CheckboxMessage::SelectAll(vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
    ]));
    assert_eq!(g.value().len(), 3);
}

#[test]
fn test_checkbox_group_clear_all() {
    let mut g = CheckboxGroup::new()
        .with_value(vec!["apple", "banana"]);
    g.handle(CheckboxMessage::ClearAll);
    assert!(g.value().is_empty());
}

#[test]
fn test_checkbox_group_select_all_respects_max() {
    let mut g = CheckboxGroup::new().with_max(2);
    g.handle(CheckboxMessage::SelectAll(vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
    ]));
    // 受 max 限制只保留前 2 个
    assert_eq!(g.value().len(), 2);
}

// ---------- indeterminate 计算属性 ----------

#[test]
fn test_checkbox_group_indeterminate_partial_selected() {
    let g = CheckboxGroup::new()
        .with_value(vec!["apple"]);
    // 部分选中应返回 true（用于"全选"checkbox 半选状态）
    assert!(g.is_indeterminate(&["apple", "banana", "cherry"]));
}

#[test]
fn test_checkbox_group_indeterminate_none_selected() {
    let g = CheckboxGroup::new();
    assert!(!g.is_indeterminate(&["apple", "banana"]));
}

#[test]
fn test_checkbox_group_indeterminate_all_selected() {
    let g = CheckboxGroup::new()
        .with_value(vec!["apple", "banana"]);
    // 全部选中应为 false（非半选）
    assert!(!g.is_indeterminate(&["apple", "banana"]));
}

// ---------- 边界 ----------

#[test]
fn test_checkbox_group_empty_options_safe() {
    let mut g = CheckboxGroup::new();
    g.handle(CheckboxMessage::SelectAll(vec![]));
    assert!(g.value().is_empty());
    assert!(!g.is_indeterminate(&[]));
}

#[test]
fn test_checkbox_group_max_zero_blocks_all() {
    let mut g = CheckboxGroup::new().with_max(0);
    g.handle(CheckboxMessage::ToggleValue("apple".to_string()));
    // max=0 应不允许任何选项
    assert_eq!(g.value().len(), 0);
}
