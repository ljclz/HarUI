//! R.2.P0.7 Select view() 测试 — TDD RED 阶段
//!
//! 验证 Select view() 正确渲染触发器 + 下拉列表。

use har_ui_components::select::{Select, SelectMessage, SelectOption};
use har_ui_core::theme::Theme;

// ============== R.2.P0.7.a view() 基础渲染 ==============

#[test]
fn test_select_view_empty_renders() {
    let theme = Theme::element_light();
    let s = Select::new();
    let _element = s.view(&theme, |_| ());
}

#[test]
fn test_select_view_with_options_renders() {
    let theme = Theme::element_light();
    let s = Select::new()
        .with_option(SelectOption::new("a", "Apple"))
        .with_option(SelectOption::new("b", "Banana"));
    let _element = s.view(&theme, |_| ());
}

#[test]
fn test_select_view_with_value_renders() {
    let theme = Theme::element_light();
    let mut s = Select::new()
        .with_option(SelectOption::new("a", "Apple"))
        .with_option(SelectOption::new("b", "Banana"));
    s.handle(SelectMessage::Choose("a".to_string()));
    assert_eq!(s.value(), Some(&"a".to_string()));
    let _element = s.view(&theme, |_| ());
}

#[test]
fn test_select_view_open_state_renders() {
    let theme = Theme::element_light();
    let mut s = Select::new()
        .with_option(SelectOption::new("a", "Apple"))
        .with_option(SelectOption::new("b", "Banana"));
    s.handle(SelectMessage::Open);
    let _element = s.view(&theme, |_| ());
}

#[test]
fn test_select_view_disabled_renders() {
    let theme = Theme::element_light();
    let s = Select::new()
        .with_option(SelectOption::new("a", "Apple"))
        .with_disabled(true);
    let _element = s.view(&theme, |_| ());
}

#[test]
fn test_select_view_clearable_renders() {
    let theme = Theme::element_light();
    let mut s = Select::new()
        .with_option(SelectOption::new("a", "Apple"))
        .with_clearable(true);
    s.handle(SelectMessage::Choose("a".to_string()));
    let _element = s.view(&theme, |_| ());
}

#[test]
fn test_select_view_filterable_renders() {
    let theme = Theme::element_light();
    let s = Select::new()
        .with_option(SelectOption::new("a", "Apple"))
        .with_filterable(true);
    let _element = s.view(&theme, |_| ());
}

#[test]
fn test_select_view_multiple_renders() {
    let theme = Theme::element_light();
    let mut s = Select::new()
        .with_option(SelectOption::new("a", "Apple"))
        .with_option(SelectOption::new("b", "Banana"))
        .with_multiple(true);
    s.handle(SelectMessage::Choose("a".to_string()));
    s.handle(SelectMessage::Choose("b".to_string()));
    let _element = s.view(&theme, |_| ());
}

#[test]
fn test_select_view_with_query_renders() {
    let theme = Theme::element_light();
    let mut s = Select::new()
        .with_option(SelectOption::new("a", "Apple"))
        .with_option(SelectOption::new("b", "Banana"))
        .with_filterable(true);
    s.handle(SelectMessage::Open);
    s.handle(SelectMessage::Query("ban".to_string()));
    let _element = s.view(&theme, |_| ());
}

#[test]
fn test_select_view_disabled_option_renders() {
    let theme = Theme::element_light();
    let s = Select::new()
        .with_option(SelectOption::new("a", "Apple").set_disabled(true))
        .with_option(SelectOption::new("b", "Banana"));
    let _element = s.view(&theme, |_| ());
}

#[test]
fn test_select_view_many_options_renders() {
    let theme = Theme::element_light();
    let mut s = Select::new();
    for i in 0..50 {
        s = s.with_option(SelectOption::new(format!("v{}", i), format!("Option {}", i)));
    }
    s.handle(SelectMessage::Open);
    let _element = s.view(&theme, |_| ());
}

#[test]
fn test_select_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let s = Select::new()
        .with_option(SelectOption::new("a", "Apple"));
    let _element = s.view(&theme, |_| ());
}

#[test]
fn test_select_view_with_custom_message_type() {
    let theme = Theme::element_light();
    let s = Select::new()
        .with_option(SelectOption::new("a", "Apple"));
    #[derive(Clone, Debug)]
    #[allow(dead_code)]
    enum AppMsg {
        Choose(String),
        Open,
        Close,
    }
    let _element = s.view(&theme, |v| AppMsg::Choose(v));
}
