//! R.2.P0.5 ColorPicker view() 测试 — TDD RED 阶段
//!
//! 验证 ColorPicker view() 正确渲染颜色块 + 色板网格（纯展示）。

use har_ui_components::color_picker::{ColorFormat, ColorPicker, ColorPickerMessage};
use har_ui_core::theme::Theme;

// ============== R.2.P0.5.a view() 基础渲染 ==============

#[test]
fn test_color_picker_view_default_renders() {
    let theme = Theme::element_light();
    let cp = ColorPicker::new();
    let _element = cp.view(&theme);
}

#[test]
fn test_color_picker_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let cp = ColorPicker::new();
    let _element = cp.view(&theme);
}

#[test]
fn test_color_picker_view_with_color_renders() {
    let theme = Theme::element_light();
    let mut cp = ColorPicker::new();
    cp.handle(ColorPickerMessage::SetColor("#ff0000".to_string()));
    assert_eq!(cp.color(), "#ff0000");
    let _element = cp.view(&theme);
}

#[test]
fn test_color_picker_view_with_rgb_color_renders() {
    let theme = Theme::element_light();
    let mut cp = ColorPicker::new();
    cp.handle(ColorPickerMessage::SetColor("#aabbcc".to_string()));
    let _element = cp.view(&theme);
}

#[test]
fn test_color_picker_view_with_short_hex_renders() {
    let theme = Theme::element_light();
    let mut cp = ColorPicker::new();
    cp.handle(ColorPickerMessage::SetColor("#abc".to_string()));
    let _element = cp.view(&theme);
}

#[test]
fn test_color_picker_view_with_alpha_renders() {
    let theme = Theme::element_light();
    let mut cp = ColorPicker::new().with_show_alpha(true);
    cp.handle(ColorPickerMessage::SetColor("#ff0000".to_string()));
    cp.handle(ColorPickerMessage::SetAlpha(128));
    assert_eq!(cp.alpha(), 128);
    let _element = cp.view(&theme);
}

#[test]
fn test_color_picker_view_disabled_renders() {
    let theme = Theme::element_light();
    let cp = ColorPicker::new().with_disabled(true);
    assert!(cp.disabled());
    let _element = cp.view(&theme);
}

#[test]
fn test_color_picker_view_disabled_with_color_renders() {
    let theme = Theme::element_light();
    let mut cp = ColorPicker::new().with_disabled(true);
    // disabled 状态下 SetColor 应被忽略
    cp.handle(ColorPickerMessage::SetColor("#ff0000".to_string()));
    assert_eq!(cp.color(), "");
    let _element = cp.view(&theme);
}

// ============== R.2.P0.5.b 色板网格渲染 ==============

#[test]
fn test_color_picker_view_with_predefine_renders() {
    let theme = Theme::element_light();
    let cp = ColorPicker::new().with_predefine(vec![
        "#ff0000".to_string(),
        "#00ff00".to_string(),
        "#0000ff".to_string(),
    ]);
    let _element = cp.view(&theme);
}

#[test]
fn test_color_picker_view_panel_visible_with_predefine_renders() {
    let theme = Theme::element_light();
    let mut cp = ColorPicker::new().with_predefine(vec![
        "#ff0000".to_string(),
        "#00ff00".to_string(),
        "#0000ff".to_string(),
    ]);
    cp.handle(ColorPickerMessage::TogglePanel);
    assert!(cp.panel_visible());
    let _element = cp.view(&theme);
}

#[test]
fn test_color_picker_view_panel_visible_no_predefine_renders() {
    let theme = Theme::element_light();
    let mut cp = ColorPicker::new();
    cp.handle(ColorPickerMessage::TogglePanel);
    assert!(cp.panel_visible());
    // predefine 为空时面板不应渲染网格
    let _element = cp.view(&theme);
}

#[test]
fn test_color_picker_view_select_predefine_renders() {
    let theme = Theme::element_light();
    let mut cp =
        ColorPicker::new().with_predefine(vec!["#ff0000".to_string(), "#00ff00".to_string()]);
    cp.handle(ColorPickerMessage::TogglePanel);
    cp.handle(ColorPickerMessage::SelectPredefine(0));
    assert_eq!(cp.color(), "#ff0000");
    let _element = cp.view(&theme);
}

#[test]
fn test_color_picker_view_clear_renders() {
    let theme = Theme::element_light();
    let mut cp = ColorPicker::new();
    cp.handle(ColorPickerMessage::SetColor("#ff0000".to_string()));
    cp.handle(ColorPickerMessage::Clear);
    assert_eq!(cp.color(), "");
    let _element = cp.view(&theme);
}

// ============== R.2.P0.5.c format 变体 ==============

#[test]
fn test_color_picker_view_format_hex_renders() {
    let theme = Theme::element_light();
    let cp = ColorPicker::new().with_format(ColorFormat::Hex);
    assert_eq!(cp.format(), ColorFormat::Hex);
    let _element = cp.view(&theme);
}

#[test]
fn test_color_picker_view_format_rgb_renders() {
    let theme = Theme::element_light();
    let cp = ColorPicker::new().with_format(ColorFormat::Rgb);
    let _element = cp.view(&theme);
}

#[test]
fn test_color_picker_view_format_hsl_renders() {
    let theme = Theme::element_light();
    let cp = ColorPicker::new().with_format(ColorFormat::Hsl);
    let _element = cp.view(&theme);
}

#[test]
fn test_color_picker_view_format_hsv_renders() {
    let theme = Theme::element_light();
    let cp = ColorPicker::new().with_format(ColorFormat::Hsv);
    let _element = cp.view(&theme);
}

// ============== R.2.P0.5.d 边界情况 ==============

#[test]
fn test_color_picker_view_invalid_color_ignored_renders() {
    let theme = Theme::element_light();
    let mut cp = ColorPicker::new();
    cp.handle(ColorPickerMessage::SetColor("not-a-color".to_string()));
    // 无效颜色应被忽略
    assert_eq!(cp.color(), "");
    let _element = cp.view(&theme);
}

#[test]
fn test_color_picker_view_alpha_clamp_renders() {
    let theme = Theme::element_light();
    let mut cp = ColorPicker::new().with_show_alpha(true);
    cp.handle(ColorPickerMessage::SetAlpha(300));
    assert_eq!(cp.alpha(), 255);
    cp.handle(ColorPickerMessage::SetAlpha(-50));
    assert_eq!(cp.alpha(), 0);
    let _element = cp.view(&theme);
}

#[test]
fn test_color_picker_view_predefine_out_of_bounds_renders() {
    let theme = Theme::element_light();
    let mut cp = ColorPicker::new().with_predefine(vec!["#ff0000".to_string()]);
    cp.handle(ColorPickerMessage::SelectPredefine(10));
    // 越界应无操作
    assert_eq!(cp.color(), "");
    let _element = cp.view(&theme);
}

#[test]
fn test_color_picker_view_full_state_renders() {
    let theme = Theme::element_light();
    let mut cp = ColorPicker::new()
        .with_show_alpha(true)
        .with_predefine(vec![
            "#ff0000".to_string(),
            "#00ff00".to_string(),
            "#0000ff".to_string(),
            "#ffff00".to_string(),
        ]);
    cp.handle(ColorPickerMessage::SetColor("#aabbcc".to_string()));
    cp.handle(ColorPickerMessage::SetAlpha(200));
    cp.handle(ColorPickerMessage::TogglePanel);
    let _element = cp.view(&theme);
}
