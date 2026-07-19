//! R.2.P1.8 Statistic view() 测试 — TDD RED 阶段

use har_ui_components::statistic::{Statistic, StatisticValue};
use har_ui_core::theme::Theme;

#[test]
fn test_statistic_view_default_renders() {
    let theme = Theme::element_light();
    let s = Statistic::default();
    let _element = s.view(&theme);
}

#[test]
fn test_statistic_view_int_renders() {
    let theme = Theme::element_light();
    let s = Statistic::new(StatisticValue::Int(42));
    let _element = s.view(&theme);
}

#[test]
fn test_statistic_view_float_renders() {
    let theme = Theme::element_light();
    let s = Statistic::new(StatisticValue::Float(3.14)).with_precision(2);
    let _element = s.view(&theme);
}

#[test]
fn test_statistic_view_with_title_renders() {
    let theme = Theme::element_light();
    let s = Statistic::new(StatisticValue::Int(100)).with_title("活跃用户");
    let _element = s.view(&theme);
}

#[test]
fn test_statistic_view_with_prefix_suffix_renders() {
    let theme = Theme::element_light();
    let s = Statistic::new(StatisticValue::Int(99))
        .with_prefix("¥")
        .with_suffix("元");
    let _element = s.view(&theme);
}

#[test]
fn test_statistic_view_with_grouping_renders() {
    let theme = Theme::element_light();
    let s = Statistic::new(StatisticValue::Int(1234567))
        .with_grouping(true);
    let _element = s.view(&theme);
}

#[test]
fn test_statistic_view_text_value_renders() {
    let theme = Theme::element_light();
    let s = Statistic::new(StatisticValue::Text("N/A".to_string()));
    let _element = s.view(&theme);
}

#[test]
fn test_statistic_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let s = Statistic::new(StatisticValue::Int(0));
    let _element = s.view(&theme);
}
