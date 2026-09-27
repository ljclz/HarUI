//! R.2.P0.1 Button view() 测试 — TDD RED 阶段
//!
//! 验证 Button 组件的 view() 方法正确产出 iced::Element，
//! 并按 ButtonType/Size/round/circle 注入 style。
//!
//! 这些测试在 view() 及其辅助方法未实现前必须编译失败/测试失败。

use har_ui_components::button::{Button, ButtonSize, ButtonType};
use har_ui_core::theme::Theme;
use har_ui_core::theme::style_sheets::ButtonKind;
use iced::widget::button;

// ============== R.2.P0.1.a ButtonType → ButtonKind 映射 ==============

#[test]
fn test_button_kind_mapping_all_types() {
    let cases = [
        (ButtonType::Default, ButtonKind::Default),
        (ButtonType::Primary, ButtonKind::Primary),
        (ButtonType::Success, ButtonKind::Success),
        (ButtonType::Warning, ButtonKind::Warning),
        (ButtonType::Danger, ButtonKind::Danger),
        (ButtonType::Info, ButtonKind::Info),
        (ButtonType::Text, ButtonKind::Text),
        (ButtonType::Link, ButtonKind::Link),
    ];
    for (bt, expected_kind) in cases {
        let btn = Button::new("test").with_type(bt);
        assert_eq!(
            btn.kind(),
            expected_kind,
            "ButtonType {:?} 应映射到 ButtonKind {:?}",
            bt,
            expected_kind
        );
    }
}

#[test]
fn test_button_kind_default_when_no_type_set() {
    let btn = Button::new("test");
    assert_eq!(btn.kind(), ButtonKind::Default);
}

// ============== R.2.P0.1.b padding_for_size 按尺寸返回不同 padding ==============

#[test]
fn test_button_padding_large_geq_default() {
    let large = Button::padding_for_size(ButtonSize::Large);
    let default = Button::padding_for_size(ButtonSize::Default);
    assert!(
        large.top >= default.top,
        "Large padding.top ({}) 应 >= Default ({})",
        large.top,
        default.top
    );
    assert!(
        large.left >= default.left,
        "Large padding.left ({}) 应 >= Default ({})",
        large.left,
        default.left
    );
}

#[test]
fn test_button_padding_small_leq_default() {
    let small = Button::padding_for_size(ButtonSize::Small);
    let default = Button::padding_for_size(ButtonSize::Default);
    assert!(
        small.top <= default.top,
        "Small padding.top ({}) 应 <= Default ({})",
        small.top,
        default.top
    );
    assert!(
        small.left <= default.left,
        "Small padding.left ({}) 应 <= Default ({})",
        small.left,
        default.left
    );
}

#[test]
fn test_button_padding_large_gt_small() {
    let large = Button::padding_for_size(ButtonSize::Large);
    let small = Button::padding_for_size(ButtonSize::Small);
    assert!(
        large.top > small.top,
        "Large padding.top ({}) 应 > Small ({})",
        large.top,
        small.top
    );
}

// ============== R.2.P0.1.c radius_for_shape：round/circle ==============

#[test]
fn test_button_radius_default_shape_is_zero() {
    let r = Button::radius_for_shape(false, false);
    assert_eq!(r.top_left, 0.0);
    assert_eq!(r.top_right, 0.0);
    assert_eq!(r.bottom_right, 0.0);
    assert_eq!(r.bottom_left, 0.0);
}

#[test]
fn test_button_radius_round_is_positive_and_uniform() {
    let r = Button::radius_for_shape(true, false);
    assert!(r.top_left > 0.0, "round 应有非零半径");
    assert_eq!(r.top_left, r.top_right, "四角半径应相等");
    assert_eq!(r.top_left, r.bottom_right, "四角半径应相等");
    assert_eq!(r.top_left, r.bottom_left, "四角半径应相等");
}

#[test]
fn test_button_radius_circle_is_positive_and_uniform() {
    let r = Button::radius_for_shape(false, true);
    assert!(r.top_left > 0.0, "circle 应有非零半径");
    assert_eq!(r.top_left, r.top_right);
    assert_eq!(r.top_left, r.bottom_right);
    assert_eq!(r.top_left, r.bottom_left);
}

#[test]
fn test_button_radius_circle_greater_than_round() {
    let circle = Button::radius_for_shape(false, true);
    let round = Button::radius_for_shape(true, false);
    assert!(
        circle.top_left > round.top_left,
        "circle 半径 ({}) 应 > round 半径 ({})",
        circle.top_left,
        round.top_left
    );
}

#[test]
fn test_button_radius_circle_dominates_round() {
    // 同时 round=true circle=true 时，circle 优先
    let both = Button::radius_for_shape(true, true);
    let circle_only = Button::radius_for_shape(false, true);
    assert_eq!(both.top_left, circle_only.top_left, "circle 应优先于 round");
}

// ============== R.2.P0.1.d compute_style：包含 round/circle 半径 ==============

#[test]
fn test_button_compute_style_round_includes_radius() {
    let theme = Theme::element_light();
    let btn = Button::new("test").round(true);
    let style = btn.compute_style(&theme, button::Status::Active);
    assert!(
        style.border.radius.top_left > 0.0,
        "round button 的 style 应包含非零半径"
    );
}

#[test]
fn test_button_compute_style_circle_includes_radius() {
    let theme = Theme::element_light();
    let btn = Button::new("test").circle(true);
    let style = btn.compute_style(&theme, button::Status::Active);
    assert!(
        style.border.radius.top_left > 0.0,
        "circle button 的 style 应包含非零半径"
    );
}

#[test]
fn test_button_compute_style_default_no_radius() {
    let theme = Theme::element_light();
    let btn = Button::new("test");
    let style = btn.compute_style(&theme, button::Status::Active);
    assert_eq!(
        style.border.radius.top_left, 0.0,
        "非 round/circle button 的半径应为 0"
    );
}

#[test]
fn test_button_compute_style_primary_uses_primary_palette() {
    let theme = Theme::element_light();
    let btn = Button::new("test").with_type(ButtonType::Primary);
    let style = btn.compute_style(&theme, button::Status::Active);
    use iced::{Background, Color};
    match style.background {
        Some(Background::Color(c)) => {
            let expected = Color::from(theme.primary.base);
            assert_eq!(
                c, expected,
                "Primary button Active 应使用 primary.base 背景"
            );
        }
        other => panic!("Primary button 应有 Color 背景, got {:?}", other),
    }
}

// ============== R.2.P0.1.e view() 渲染不 panic ==============

#[test]
fn test_button_view_default_renders() {
    let theme = Theme::element_light();
    let btn = Button::new("Click me");
    let _element = btn.view(&theme, ());
}

#[test]
fn test_button_view_all_types_renders() {
    let theme = Theme::element_light();
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
    for t in types {
        let btn = Button::new("test").with_type(t);
        let _element = btn.view(&theme, ());
    }
}

#[test]
fn test_button_view_all_sizes_renders() {
    let theme = Theme::element_light();
    let btn_l = Button::new("L").with_size(ButtonSize::Large);
    let _e1 = btn_l.view(&theme, ());
    let btn_d = Button::new("D");
    let _e2 = btn_d.view(&theme, ());
    let btn_s = Button::new("S").with_size(ButtonSize::Small);
    let _e3 = btn_s.view(&theme, ());
}

#[test]
fn test_button_view_disabled_renders() {
    let theme = Theme::element_light();
    let btn = Button::new("test").disabled(true);
    let _element = btn.view(&theme, ());
}

#[test]
fn test_button_view_loading_renders() {
    let theme = Theme::element_light();
    let btn = Button::new("test").loading(true);
    let _element = btn.view(&theme, ());
}

#[test]
fn test_button_view_plain_renders() {
    let theme = Theme::element_light();
    let btn = Button::new("test")
        .plain(true)
        .with_type(ButtonType::Primary);
    let _element = btn.view(&theme, ());
}

#[test]
fn test_button_view_round_circle_renders() {
    let theme = Theme::element_light();
    let btn_round = Button::new("R").round(true);
    let _e1 = btn_round.view(&theme, ());
    let btn_circle = Button::new("C").circle(true);
    let _e2 = btn_circle.view(&theme, ());
}

#[test]
fn test_button_view_with_custom_message_type() {
    let theme = Theme::element_light();
    let btn = Button::new("test");
    #[derive(Clone, Debug)]
    struct AppMsg;
    let _element = btn.view(&theme, AppMsg);
}

#[test]
fn test_button_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let btn = Button::new("test").with_type(ButtonType::Primary);
    let _element = btn.view(&theme, ());
}
