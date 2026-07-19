//! Select 组件 — 选择器
//!
//! 参考 Element Plus `<el-select>`。
//! 支持：单选/多选、禁用选项、可搜索(filterable)、clearable、虚拟列表(1000 选项)。

use har_ui_components::select::{Select, SelectMessage, SelectOption, SelectState};

// ---------- SelectOption ----------

#[test]
fn test_select_option_default() {
    let opt = SelectOption::new("opt1", "Option 1");
    assert_eq!(opt.value(), "opt1");
    assert_eq!(opt.label(), "Option 1");
    assert!(!opt.disabled());
}

#[test]
fn test_select_option_disabled() {
    let opt = SelectOption::new("opt1", "Option 1").set_disabled(true);
    assert!(opt.disabled());
}

// ---------- 基础构造 ----------

#[test]
fn test_select_default() {
    let s = Select::new();
    assert!(s.options().is_empty());
    assert_eq!(s.value(), None);
    assert!(!s.multiple());
    assert!(!s.filterable());
    assert!(!s.clearable());
    assert!(!s.disabled());
    assert_eq!(s.state(), SelectState::Closed);
}

#[test]
fn test_select_with_options() {
    let s = Select::new()
        .with_option(SelectOption::new("a", "A"))
        .with_option(SelectOption::new("b", "B"))
        .with_option(SelectOption::new("c", "C"));
    assert_eq!(s.options().len(), 3);
}

#[test]
fn test_select_with_multiple() {
    let s = Select::new().with_multiple(true);
    assert!(s.multiple());
}

#[test]
fn test_select_with_filterable() {
    let s = Select::new().with_filterable(true);
    assert!(s.filterable());
}

#[test]
fn test_select_with_clearable() {
    let s = Select::new().with_clearable(true);
    assert!(s.clearable());
}

#[test]
fn test_select_with_disabled() {
    let s = Select::new().with_disabled(true);
    assert!(s.disabled());
}

// ---------- 打开/关闭 ----------

#[test]
fn test_select_open_close() {
    let mut s = Select::new();
    s.handle(SelectMessage::Open);
    assert_eq!(s.state(), SelectState::Open);
    s.handle(SelectMessage::Close);
    assert_eq!(s.state(), SelectState::Closed);
}

#[test]
fn test_select_disabled_cannot_open() {
    let mut s = Select::new().with_disabled(true);
    s.handle(SelectMessage::Open);
    assert_eq!(s.state(), SelectState::Closed);
}

// ---------- 单选 ----------

#[test]
fn test_select_single_choose() {
    let mut s = Select::new()
        .with_option(SelectOption::new("a", "A"))
        .with_option(SelectOption::new("b", "B"));
    s.handle(SelectMessage::Choose("a".to_string()));
    assert_eq!(s.value(), Some(&"a".to_string()));
}

#[test]
fn test_select_choose_nonexistent_ignored() {
    let mut s = Select::new().with_option(SelectOption::new("a", "A"));
    s.handle(SelectMessage::Choose("nonexistent".to_string()));
    assert_eq!(s.value(), None);
}

#[test]
fn test_select_choose_disabled_ignored() {
    let mut s = Select::new()
        .with_option(SelectOption::new("a", "A").set_disabled(true));
    s.handle(SelectMessage::Choose("a".to_string()));
    assert_eq!(s.value(), None);
}

#[test]
fn test_select_choose_closes_dropdown() {
    let mut s = Select::new()
        .with_option(SelectOption::new("a", "A"));
    s.handle(SelectMessage::Open);
    s.handle(SelectMessage::Choose("a".to_string()));
    assert_eq!(s.state(), SelectState::Closed);
}

// ---------- 多选 ----------

#[test]
fn test_select_multiple_choose() {
    let mut s = Select::new()
        .with_multiple(true)
        .with_option(SelectOption::new("a", "A"))
        .with_option(SelectOption::new("b", "B"))
        .with_option(SelectOption::new("c", "C"));
    s.handle(SelectMessage::Choose("a".to_string()));
    s.handle(SelectMessage::Choose("b".to_string()));
    assert_eq!(s.values().len(), 2);
    assert!(s.values().contains(&"a".to_string()));
    assert!(s.values().contains(&"b".to_string()));
}

#[test]
fn test_select_multiple_toggle_off() {
    let mut s = Select::new()
        .with_multiple(true)
        .with_option(SelectOption::new("a", "A"));
    s.handle(SelectMessage::Choose("a".to_string()));
    assert_eq!(s.values().len(), 1);
    // 再次选择同一项，取消
    s.handle(SelectMessage::Choose("a".to_string()));
    assert_eq!(s.values().len(), 0);
}

#[test]
fn test_select_multiple_keeps_dropdown_open() {
    let mut s = Select::new()
        .with_multiple(true)
        .with_option(SelectOption::new("a", "A"));
    s.handle(SelectMessage::Open);
    s.handle(SelectMessage::Choose("a".to_string()));
    assert_eq!(s.state(), SelectState::Open);
}

// ---------- Clearable ----------

#[test]
fn test_select_clear_single() {
    let mut s = Select::new()
        .with_clearable(true)
        .with_option(SelectOption::new("a", "A"));
    s.handle(SelectMessage::Choose("a".to_string()));
    s.handle(SelectMessage::Clear);
    assert_eq!(s.value(), None);
}

#[test]
fn test_select_clear_multiple() {
    let mut s = Select::new()
        .with_clearable(true)
        .with_multiple(true)
        .with_option(SelectOption::new("a", "A"))
        .with_option(SelectOption::new("b", "B"));
    s.handle(SelectMessage::Choose("a".to_string()));
    s.handle(SelectMessage::Choose("b".to_string()));
    s.handle(SelectMessage::Clear);
    assert_eq!(s.values().len(), 0);
}

// ---------- Filterable 搜索 ----------

#[test]
fn test_select_filter_options() {
    let s = Select::new()
        .with_option(SelectOption::new("apple", "Apple"))
        .with_option(SelectOption::new("banana", "Banana"))
        .with_option(SelectOption::new("cherry", "Cherry"));
    let filtered = s.filter("an");
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].value(), "banana");
}

#[test]
fn test_select_filter_case_insensitive() {
    let s = Select::new()
        .with_option(SelectOption::new("apple", "Apple"));
    let filtered = s.filter("APP");
    assert_eq!(filtered.len(), 1);
}

#[test]
fn test_select_filter_empty_query_returns_all() {
    let s = Select::new()
        .with_option(SelectOption::new("a", "A"))
        .with_option(SelectOption::new("b", "B"));
    let filtered = s.filter("");
    assert_eq!(filtered.len(), 2);
}

// ---------- 1000 选项虚拟列表（性能基线） ----------

#[test]
fn test_select_1000_options_filter_fast() {
    let mut s = Select::new();
    for i in 0..1000 {
        s = s.with_option(SelectOption::new(format!("opt{}", i), format!("Option {}", i)));
    }
    assert_eq!(s.options().len(), 1000);
    // 搜索应能精确定位
    let filtered = s.filter("opt500");
    assert!(filtered.iter().any(|o| o.value() == "opt500"));
}

// ---------- 查询文本 ----------

#[test]
fn test_select_set_query() {
    let mut s = Select::new().with_filterable(true);
    s.handle(SelectMessage::Query("test".to_string()));
    assert_eq!(s.query(), Some("test".to_string()));
}

#[test]
fn test_select_clear_query() {
    let mut s = Select::new().with_filterable(true);
    s.handle(SelectMessage::Query("test".to_string()));
    s.handle(SelectMessage::Query(String::new()));
    assert_eq!(s.query(), Some(String::new()));
}

// ---------- 显示文本 ----------

#[test]
fn test_select_display_label_single() {
    let mut s = Select::new()
        .with_option(SelectOption::new("a", "Apple"))
        .with_option(SelectOption::new("b", "Banana"));
    s.handle(SelectMessage::Choose("a".to_string()));
    assert_eq!(s.display_label(), Some("Apple".to_string()));
}

#[test]
fn test_select_display_label_empty_when_no_selection() {
    let s = Select::new().with_option(SelectOption::new("a", "A"));
    assert_eq!(s.display_label(), None);
}
