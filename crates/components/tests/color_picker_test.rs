//! ColorPicker 颜色选择 — 参考 Element Plus `<el-color-picker>`。
//!
//! 覆盖：color、alpha、show_alpha、disabled、format、predefine、panel、clear。

use har_ui_components::color_picker::{ColorFormat, ColorPicker, ColorPickerMessage};

#[test]
fn test_color_picker_default() {
    let c = ColorPicker::new();
    assert_eq!(c.color(), "");
    assert_eq!(c.alpha(), 255);
    assert!(!c.show_alpha());
    assert!(!c.disabled());
    assert_eq!(c.format(), ColorFormat::Hex);
    assert!(c.predefine().is_empty());
    assert!(!c.panel_visible());
}

#[test]
fn test_color_picker_set_color() {
    let mut c = ColorPicker::new();
    c.handle(ColorPickerMessage::SetColor("#ff0000".into()));
    assert_eq!(c.color(), "#ff0000");
}

#[test]
fn test_color_picker_set_alpha() {
    let mut c = ColorPicker::new();
    c.handle(ColorPickerMessage::SetAlpha(128));
    assert_eq!(c.alpha(), 128);
}

#[test]
fn test_color_picker_alpha_clamp_high() {
    let mut c = ColorPicker::new();
    c.handle(ColorPickerMessage::SetAlpha(300));
    assert_eq!(c.alpha(), 255);
}

#[test]
fn test_color_picker_alpha_clamp_low() {
    let mut c = ColorPicker::new();
    c.handle(ColorPickerMessage::SetAlpha(-50));
    assert_eq!(c.alpha(), 0);
}

#[test]
fn test_color_picker_with_show_alpha() {
    let c = ColorPicker::new().with_show_alpha(true);
    assert!(c.show_alpha());
}

#[test]
fn test_color_picker_disabled_noop() {
    let mut c = ColorPicker::new().with_disabled(true);
    c.handle(ColorPickerMessage::SetColor("#ff0000".into()));
    assert_eq!(c.color(), "");
}

#[test]
fn test_color_picker_with_format_rgb() {
    let c = ColorPicker::new().with_format(ColorFormat::Rgb);
    assert_eq!(c.format(), ColorFormat::Rgb);
}

#[test]
fn test_color_picker_with_predefine() {
    let c = ColorPicker::new()
        .with_predefine(vec!["#ff0000".into(), "#00ff00".into(), "#0000ff".into()]);
    assert_eq!(c.predefine().len(), 3);
}

#[test]
fn test_color_picker_select_predefine() {
    let mut c = ColorPicker::new()
        .with_predefine(vec!["#ff0000".into(), "#00ff00".into()]);
    c.handle(ColorPickerMessage::SelectPredefine(1));
    assert_eq!(c.color(), "#00ff00");
}

#[test]
fn test_color_picker_select_predefine_out_of_bounds_noop() {
    let mut c = ColorPicker::new()
        .with_predefine(vec!["#ff0000".into()]);
    c.handle(ColorPickerMessage::SelectPredefine(5));
    assert_eq!(c.color(), "");
}

#[test]
fn test_color_picker_toggle_panel() {
    let mut c = ColorPicker::new();
    assert!(!c.panel_visible());
    c.handle(ColorPickerMessage::TogglePanel);
    assert!(c.panel_visible());
    c.handle(ColorPickerMessage::TogglePanel);
    assert!(!c.panel_visible());
}

#[test]
fn test_color_picker_clear() {
    let mut c = ColorPicker::new();
    c.handle(ColorPickerMessage::SetColor("#ff0000".into()));
    c.handle(ColorPickerMessage::SetAlpha(128));
    c.handle(ColorPickerMessage::Clear);
    assert_eq!(c.color(), "");
    assert_eq!(c.alpha(), 255);
}

#[test]
fn test_color_picker_set_invalid_color_noop() {
    let mut c = ColorPicker::new();
    c.handle(ColorPickerMessage::SetColor("not-a-color".into()));
    assert_eq!(c.color(), "");
}

#[test]
fn test_color_picker_valid_color_forms() {
    let mut c = ColorPicker::new();
    // 3 位 hex
    c.handle(ColorPickerMessage::SetColor("#abc".into()));
    assert_eq!(c.color(), "#abc");
    // 6 位 hex
    c.handle(ColorPickerMessage::SetColor("#aabbcc".into()));
    assert_eq!(c.color(), "#aabbcc");
    // 8 位 hex (带 alpha)
    c.handle(ColorPickerMessage::SetColor("#aabbcc80".into()));
    assert_eq!(c.color(), "#aabbcc80");
}
