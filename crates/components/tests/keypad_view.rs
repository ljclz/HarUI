//! Keypad view() 测试 — TDD RED 阶段

use har_ui_components::keypad::{Keypad, KeypadMessage, KeypadMode};
use har_ui_core::theme::Theme;

#[test]
fn test_keypad_view_default_renders() {
    let theme = Theme::element_light();
    let k = Keypad::new();
    let _element = k.view(&theme, |_| ());
}

#[test]
fn test_keypad_view_with_value_renders() {
    let theme = Theme::element_light();
    let mut k = Keypad::new();
    k.handle(KeypadMessage::Digit(1));
    k.handle(KeypadMessage::Digit(2));
    k.handle(KeypadMessage::Digit(3));
    let _element = k.view(&theme, |_| ());
}

#[test]
fn test_keypad_view_price_mode_renders() {
    let theme = Theme::element_light();
    let mut k = Keypad::new().with_mode(KeypadMode::Price);
    k.handle(KeypadMessage::Digit(9));
    k.handle(KeypadMessage::Dot);
    k.handle(KeypadMessage::Digit(9));
    let _element = k.view(&theme, |_| ());
}

#[test]
fn test_keypad_view_number_mode_renders() {
    let theme = Theme::element_light();
    let k = Keypad::new().with_mode(KeypadMode::Number);
    let _element = k.view(&theme, |_| ());
}

#[test]
fn test_keypad_view_quantity_mode_renders() {
    let theme = Theme::element_light();
    let k = Keypad::new().with_mode(KeypadMode::Quantity);
    let _element = k.view(&theme, |_| ());
}

#[test]
fn test_keypad_view_with_max_min_renders() {
    let theme = Theme::element_light();
    let k = Keypad::new().with_max(9999.0).with_min(0.0);
    let _element = k.view(&theme, |_| ());
}

#[test]
fn test_keypad_view_confirmed_state_renders() {
    let theme = Theme::element_light();
    let mut k = Keypad::new();
    k.handle(KeypadMessage::Digit(5));
    k.handle(KeypadMessage::Ok);
    let _element = k.view(&theme, |_| ());
}

#[test]
fn test_keypad_view_invalid_state_renders() {
    let theme = Theme::element_light();
    let mut k = Keypad::new().with_max(10.0);
    k.handle(KeypadMessage::Digit(9));
    k.handle(KeypadMessage::Digit(9));
    k.handle(KeypadMessage::Ok);
    let _element = k.view(&theme, |_| ());
}

#[test]
fn test_keypad_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let k = Keypad::new();
    let _element = k.view(&theme, |_| ());
}

#[test]
fn test_keypad_view_custom_message_type() {
    let theme = Theme::element_light();
    let k = Keypad::new();
    #[derive(Clone, Debug)]
    #[allow(dead_code)]
    enum AppMsg {
        Key(String),
    }
    let _element = k.view(&theme, AppMsg::Key);
}
