//! R.2.P0.8 Radio view() 测试 — TDD RED 阶段
//!
//! 验证 Radio / RadioGroup view() 正确渲染圆点指示器 + 标签。

use har_ui_components::radio::{Radio, RadioGroup, RadioSize};
use har_ui_core::theme::Theme;

// ============== 单个 Radio view() ==============

#[test]
fn test_radio_view_unchecked_renders() {
    let theme = Theme::element_light();
    let r = Radio::new("Apple");
    let _element = r.view(&theme, ());
}

#[test]
fn test_radio_view_checked_renders() {
    let theme = Theme::element_light();
    let r = Radio::new("Apple").with_checked(true);
    let _element = r.view(&theme, ());
}

#[test]
fn test_radio_view_disabled_renders() {
    let theme = Theme::element_light();
    let r = Radio::new("Apple").with_disabled(true);
    let _element = r.view(&theme, ());
}

#[test]
fn test_radio_view_with_border_renders() {
    let theme = Theme::element_light();
    let r = Radio::new("Apple").with_border(true);
    let _element = r.view(&theme, ());
}

#[test]
fn test_radio_view_large_size_renders() {
    let theme = Theme::element_light();
    let r = Radio::new("Apple").with_size(RadioSize::Large);
    let _element = r.view(&theme, ());
}

#[test]
fn test_radio_view_small_size_renders() {
    let theme = Theme::element_light();
    let r = Radio::new("Apple").with_size(RadioSize::Small);
    let _element = r.view(&theme, ());
}

#[test]
fn test_radio_view_checked_disabled_renders() {
    let theme = Theme::element_light();
    let r = Radio::new("Apple").with_checked(true).with_disabled(true);
    let _element = r.view(&theme, ());
}

#[test]
fn test_radio_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let r = Radio::new("Apple").with_checked(true);
    let _element = r.view(&theme, ());
}

#[test]
fn test_radio_view_with_custom_message_type() {
    let theme = Theme::element_light();
    let r = Radio::new("Apple");
    #[derive(Clone, Debug)]
    #[allow(dead_code)]
    enum AppMsg {
        RadioClicked,
    }
    let _element = r.view(&theme, AppMsg::RadioClicked);
}

// ============== RadioGroup view() ==============

#[test]
fn test_radio_group_view_empty_renders() {
    let theme = Theme::element_light();
    let g = RadioGroup::new();
    let options: Vec<(&str, &str)> = vec![];
    let _element = g.view(&theme, &options, |_| ());
}

#[test]
fn test_radio_group_view_with_options_renders() {
    let theme = Theme::element_light();
    let g = RadioGroup::new();
    let options = vec![("a", "Apple"), ("b", "Banana")];
    let _element = g.view(&theme, &options, |_| ());
}

#[test]
fn test_radio_group_view_with_value_renders() {
    let theme = Theme::element_light();
    let g = RadioGroup::new().with_value("a");
    let options = vec![("a", "Apple"), ("b", "Banana")];
    let _element = g.view(&theme, &options, |_| ());
}

#[test]
fn test_radio_group_view_disabled_renders() {
    let theme = Theme::element_light();
    let g = RadioGroup::new().with_disabled(true);
    let options = vec![("a", "Apple"), ("b", "Banana")];
    let _element = g.view(&theme, &options, |_| ());
}

#[test]
fn test_radio_group_view_many_options_renders() {
    let theme = Theme::element_light();
    let g = RadioGroup::new().with_value("v3");
    let options: Vec<(String, String)> = (0..20)
        .map(|i| (format!("v{}", i), format!("Option {}", i)))
        .collect();
    let option_refs: Vec<(&str, &str)> = options
        .iter()
        .map(|(v, l)| (v.as_str(), l.as_str()))
        .collect();
    let _element = g.view(&theme, &option_refs, |_| ());
}

#[test]
fn test_radio_group_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let g = RadioGroup::new().with_value("a");
    let options = vec![("a", "Apple"), ("b", "Banana")];
    let _element = g.view(&theme, &options, |_| ());
}
