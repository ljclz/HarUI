//! R.2.P0.9 Checkbox view() 测试 — TDD RED 阶段
//!
//! 验证 Checkbox / CheckboxGroup view() 正确渲染方框指示器 + 标签。

use har_ui_components::checkbox::{Checkbox, CheckboxGroup, CheckboxSize};
use har_ui_core::theme::Theme;

// ============== 单个 Checkbox view() ==============

#[test]
fn test_checkbox_view_unchecked_renders() {
    let theme = Theme::element_light();
    let c = Checkbox::new("Apple");
    let _element = c.view(&theme, ());
}

#[test]
fn test_checkbox_view_checked_renders() {
    let theme = Theme::element_light();
    let c = Checkbox::new("Apple").with_checked(true);
    let _element = c.view(&theme, ());
}

#[test]
fn test_checkbox_view_disabled_renders() {
    let theme = Theme::element_light();
    let c = Checkbox::new("Apple").with_disabled(true);
    let _element = c.view(&theme, ());
}

#[test]
fn test_checkbox_view_indeterminate_renders() {
    let theme = Theme::element_light();
    let c = Checkbox::new("Apple").with_indeterminate(true);
    let _element = c.view(&theme, ());
}

#[test]
fn test_checkbox_view_with_border_renders() {
    let theme = Theme::element_light();
    let c = Checkbox::new("Apple").with_border(true);
    let _element = c.view(&theme, ());
}

#[test]
fn test_checkbox_view_large_size_renders() {
    let theme = Theme::element_light();
    let c = Checkbox::new("Apple").with_size(CheckboxSize::Large);
    let _element = c.view(&theme, ());
}

#[test]
fn test_checkbox_view_small_size_renders() {
    let theme = Theme::element_light();
    let c = Checkbox::new("Apple").with_size(CheckboxSize::Small);
    let _element = c.view(&theme, ());
}

#[test]
fn test_checkbox_view_checked_disabled_renders() {
    let theme = Theme::element_light();
    let c = Checkbox::new("Apple").with_checked(true).with_disabled(true);
    let _element = c.view(&theme, ());
}

#[test]
fn test_checkbox_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let c = Checkbox::new("Apple").with_checked(true);
    let _element = c.view(&theme, ());
}

#[test]
fn test_checkbox_view_with_custom_message_type() {
    let theme = Theme::element_light();
    let c = Checkbox::new("Apple");
    #[derive(Clone, Debug)]
    #[allow(dead_code)]
    enum AppMsg {
        Toggled,
    }
    let _element = c.view(&theme, AppMsg::Toggled);
}

// ============== CheckboxGroup view() ==============

#[test]
fn test_checkbox_group_view_empty_renders() {
    let theme = Theme::element_light();
    let g = CheckboxGroup::new();
    let options: Vec<(&str, &str)> = vec![];
    let _element = g.view(&theme, &options, |_| ());
}

#[test]
fn test_checkbox_group_view_with_options_renders() {
    let theme = Theme::element_light();
    let g = CheckboxGroup::new();
    let options = vec![("a", "Apple"), ("b", "Banana")];
    let _element = g.view(&theme, &options, |_| ());
}

#[test]
fn test_checkbox_group_view_with_value_renders() {
    let theme = Theme::element_light();
    let g = CheckboxGroup::new().with_value(vec!["a"]);
    let options = vec![("a", "Apple"), ("b", "Banana")];
    let _element = g.view(&theme, &options, |_| ());
}

#[test]
fn test_checkbox_group_view_disabled_renders() {
    let theme = Theme::element_light();
    let g = CheckboxGroup::new().with_disabled(true);
    let options = vec![("a", "Apple"), ("b", "Banana")];
    let _element = g.view(&theme, &options, |_| ());
}

#[test]
fn test_checkbox_group_view_with_max_renders() {
    let theme = Theme::element_light();
    let g = CheckboxGroup::new().with_value(vec!["a"]).with_max(2);
    let options = vec![("a", "Apple"), ("b", "Banana"), ("c", "Cherry")];
    let _element = g.view(&theme, &options, |_| ());
}

#[test]
fn test_checkbox_group_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let g = CheckboxGroup::new().with_value(vec!["a"]);
    let options = vec![("a", "Apple"), ("b", "Banana")];
    let _element = g.view(&theme, &options, |_| ());
}
