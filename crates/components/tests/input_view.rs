//! R.2.P0.2 Input view() 测试 — TDD RED 阶段
//!
//! 验证 Input 组件的 view() 方法正确产出 iced::Element，
//! 并按 mode/disabled 注入 style 与 secure 标志。

use har_ui_components::input::{Input, InputMode, InputState};
use har_ui_core::theme::Theme;
use iced::widget::text_input;

// ============== R.2.P0.2.a 辅助函数：mode → secure 映射 ==============

#[test]
fn test_input_is_secure_password_mode() {
    let inp = Input::new().with_mode(InputMode::Password);
    assert!(inp.is_secure(), "Password mode 应为 secure");
}

#[test]
fn test_input_is_secure_text_mode() {
    let inp = Input::new().with_mode(InputMode::Text);
    assert!(!inp.is_secure(), "Text mode 应非 secure");
}

#[test]
fn test_input_is_secure_digit_mode() {
    let inp = Input::new().with_mode(InputMode::Digit);
    assert!(!inp.is_secure(), "Digit mode 应非 secure");
}

#[test]
fn test_input_is_secure_price_mode() {
    let inp = Input::new().with_mode(InputMode::Price);
    assert!(!inp.is_secure(), "Price mode 应非 secure");
}

// ============== R.2.P0.2.b compute_style：按状态返回 style ==============

#[test]
fn test_input_compute_style_active() {
    let theme = Theme::element_light();
    let inp = Input::new();
    let style = inp.compute_style(&theme, text_input::Status::Active);
    // Active 态边框 = border_base
    assert_eq!(
        style.border.color,
        iced::Color::from(theme.neutral.border_base)
    );
}

#[test]
fn test_input_compute_style_focused() {
    let theme = Theme::element_light();
    let inp = Input::new();
    let style = inp.compute_style(&theme, text_input::Status::Focused { is_hovered: false });
    // Focused 态边框 = primary
    assert_eq!(style.border.color, iced::Color::from(theme.primary.base));
}

#[test]
fn test_input_compute_style_disabled() {
    let theme = Theme::element_light();
    let inp = Input::new();
    let style = inp.compute_style(&theme, text_input::Status::Disabled);
    // Disabled 态背景 = bg_base
    match style.background {
        iced::Background::Color(c) => assert_eq!(c, iced::Color::from(theme.neutral.bg_base)),
        _ => panic!("disabled 应有 Color 背景"),
    }
}

// ============== R.2.P0.2.c view() 渲染不 panic ==============

#[test]
fn test_input_view_default_renders() {
    let theme = Theme::element_light();
    let inp = Input::new().with_placeholder("Enter text");
    let _element = inp.view(&theme, |s: String| s);
}

#[test]
fn test_input_view_with_value_renders() {
    let theme = Theme::element_light();
    let mut inp = Input::new().with_placeholder("Name");
    inp.handle(har_ui_components::input::InputMessage::Char('A'));
    let _element = inp.view(&theme, |s: String| s);
}

#[test]
fn test_input_view_all_modes_renders() {
    let theme = Theme::element_light();
    let modes = [
        InputMode::Text,
        InputMode::Password,
        InputMode::Digit,
        InputMode::Price,
    ];
    for m in modes {
        let inp = Input::new().with_mode(m);
        let _element = inp.view(&theme, |s: String| s);
    }
}

#[test]
fn test_input_view_disabled_renders() {
    let theme = Theme::element_light();
    let inp = Input::new().disabled(true);
    let _element = inp.view(&theme, |s: String| s);
}

#[test]
fn test_input_view_with_maxlength_renders() {
    let theme = Theme::element_light();
    let inp = Input::new().with_maxlength(10);
    let _element = inp.view(&theme, |s: String| s);
}

#[test]
fn test_input_view_with_clearable_renders() {
    let theme = Theme::element_light();
    let inp = Input::new().with_clearable(true);
    let _element = inp.view(&theme, |s: String| s);
}

#[test]
fn test_input_view_with_prefix_renders() {
    let theme = Theme::element_light();
    let inp = Input::new().with_prefix("$");
    let _element = inp.view(&theme, |s: String| s);
}

#[test]
fn test_input_view_with_suffix_renders() {
    let theme = Theme::element_light();
    let inp = Input::new().with_suffix(".00");
    let _element = inp.view(&theme, |s: String| s);
}

#[test]
fn test_input_view_with_prefix_and_suffix_renders() {
    let theme = Theme::element_light();
    let inp = Input::new().with_prefix("$").with_suffix("USD");
    let _element = inp.view(&theme, |s: String| s);
}

#[test]
fn test_input_view_with_custom_message_type() {
    let theme = Theme::element_light();
    let inp = Input::new();
    #[derive(Clone, Debug)]
    #[allow(dead_code)]
    enum AppMsg {
        InputChanged(String),
    }
    let _element = inp.view(&theme, AppMsg::InputChanged);
}

#[test]
fn test_input_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let inp = Input::new().with_mode(InputMode::Password);
    let _element = inp.view(&theme, |s: String| s);
}

#[test]
fn test_input_view_composing_state_renders() {
    let theme = Theme::element_light();
    let mut inp = Input::new();
    // 触发 IME 组合开始
    inp.handle(har_ui_components::input::InputMessage::ImeEvent(
        har_ui_core::utils::ime::ImeEvent::CompositionStart,
    ));
    assert_eq!(inp.state(), InputState::Composing);
    let _element = inp.view(&theme, |s: String| s);
}
