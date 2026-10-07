//! R.1 iced 适配层测试 — 验证 har-ui Theme → iced::Theme/Style 的映射
//!
//! TDD RED 阶段：这些测试在 iced_adapter / style_sheets 模块未实现前必须编译失败/测试失败。

use har_ui_core::theme::Theme;
use har_ui_core::theme::iced_adapter;
use har_ui_core::theme::style_sheets;
use iced::widget::{button, text_input};
use iced::{Background, Border, Color, Shadow};

// ============== R.1.1 iced_adapter：Theme → iced::Theme ==============

#[test]
fn test_to_iced_theme_light_basic() {
    let theme = Theme::element_light();
    let iced_theme = iced_adapter::to_iced_theme(&theme);
    // 自定义主题名称
    let display = format!("{}", iced_theme);
    assert!(
        display.contains("HarUI"),
        "iced theme display 应包含 HarUI, got: {}",
        display
    );
    assert!(
        display.contains("Light"),
        "iced theme display 应包含 Light, got: {}",
        display
    );
}

#[test]
fn test_to_iced_theme_dark_basic() {
    let theme = Theme::element_dark();
    let iced_theme = iced_adapter::to_iced_theme(&theme);
    let display = format!("{}", iced_theme);
    assert!(display.contains("HarUI"));
    assert!(display.contains("Dark"));
}

#[test]
fn test_to_iced_theme_primary_color_mapped() {
    // 验证 primary base 色被映射到 iced Palette.primary
    let theme = Theme::element_light();
    let iced_theme = iced_adapter::to_iced_theme(&theme);
    let palette = iced_theme.palette();
    let expected = Color::from(theme.primary.base);
    assert_eq!(palette.primary, expected);
}

#[test]
fn test_to_iced_theme_success_color_mapped() {
    let theme = Theme::element_light();
    let palette = iced_adapter::to_iced_theme(&theme).palette();
    assert_eq!(palette.success, Color::from(theme.success.base));
}

#[test]
fn test_to_iced_theme_danger_color_mapped() {
    let theme = Theme::element_light();
    let palette = iced_adapter::to_iced_theme(&theme).palette();
    assert_eq!(palette.danger, Color::from(theme.danger.base));
}

#[test]
fn test_to_iced_theme_text_color_mapped() {
    // 浅色主题：text = text_primary
    let theme = Theme::element_light();
    let palette = iced_adapter::to_iced_theme(&theme).palette();
    assert_eq!(palette.text, Color::from(theme.neutral.text_primary));
    // 深色主题：text 仍 = text_primary（已是浅色 #E5EAF3）
    let theme_dark = Theme::element_dark();
    let palette_dark = iced_adapter::to_iced_theme(&theme_dark).palette();
    assert_eq!(
        palette_dark.text,
        Color::from(theme_dark.neutral.text_primary)
    );
}

#[test]
fn test_to_iced_theme_background_color_mapped() {
    // 浅色主题：background = bg_overlay（白）
    let theme = Theme::element_light();
    let palette = iced_adapter::to_iced_theme(&theme).palette();
    assert_eq!(palette.background, Color::from(theme.neutral.bg_overlay));
    // 深色主题：background = bg_page（黑）
    let theme_dark = Theme::element_dark();
    let palette_dark = iced_adapter::to_iced_theme(&theme_dark).palette();
    assert_eq!(
        palette_dark.background,
        Color::from(theme_dark.neutral.bg_page)
    );
}

// ============== R.1.2 style_sheets：Button style ==============

#[test]
fn test_button_style_primary_active() {
    let theme = Theme::element_light();
    let style = style_sheets::button_style(
        &theme,
        har_ui_core::theme::style_sheets::ButtonKind::Primary,
        false,
        button::Status::Active,
    );
    // Primary 按钮 Active 态：背景 = primary.base
    match style.background {
        Some(Background::Color(c)) => assert_eq!(c, Color::from(theme.primary.base)),
        other => panic!("Primary button 应有 Color 背景, got {:?}", other),
    }
    // 文字色：白色
    assert_eq!(style.text_color, Color::WHITE);
    // 无边框
    assert_eq!(style.border.width, 0.0);
}

#[test]
fn test_button_style_default_active() {
    let theme = Theme::element_light();
    let style = style_sheets::button_style(
        &theme,
        style_sheets::ButtonKind::Default,
        false,
        button::Status::Active,
    );
    // Default 按钮：背景透明（None）或浅色，文字 = text_primary
    assert_eq!(style.text_color, Color::from(theme.neutral.text_regular));
    // 边框 = border_base
    assert_eq!(style.border.color, Color::from(theme.neutral.border_base));
    assert!(style.border.width > 0.0);
}

#[test]
fn test_button_style_success_active() {
    let theme = Theme::element_light();
    let style = style_sheets::button_style(
        &theme,
        style_sheets::ButtonKind::Success,
        false,
        button::Status::Active,
    );
    match style.background {
        Some(Background::Color(c)) => assert_eq!(c, Color::from(theme.success.base)),
        other => panic!("Success button 应有 Color 背景, got {:?}", other),
    }
    assert_eq!(style.text_color, Color::WHITE);
}

#[test]
fn test_button_style_warning_active() {
    let theme = Theme::element_light();
    let style = style_sheets::button_style(
        &theme,
        style_sheets::ButtonKind::Warning,
        false,
        button::Status::Active,
    );
    match style.background {
        Some(Background::Color(c)) => assert_eq!(c, Color::from(theme.warning.base)),
        other => panic!("Warning button 应有 Color 背景, got {:?}", other),
    }
    assert_eq!(style.text_color, Color::WHITE);
}

#[test]
fn test_button_style_danger_active() {
    let theme = Theme::element_light();
    let style = style_sheets::button_style(
        &theme,
        style_sheets::ButtonKind::Danger,
        false,
        button::Status::Active,
    );
    match style.background {
        Some(Background::Color(c)) => assert_eq!(c, Color::from(theme.danger.base)),
        other => panic!("Danger button 应有 Color 背景, got {:?}", other),
    }
    assert_eq!(style.text_color, Color::WHITE);
}

#[test]
fn test_button_style_plain_primary() {
    // Plain 按钮：背景透明，文字 = primary 色，边框 = primary
    let theme = Theme::element_light();
    let style = style_sheets::button_style(
        &theme,
        style_sheets::ButtonKind::Primary,
        true, // plain
        button::Status::Active,
    );
    assert!(
        style.background.is_none()
            || matches!(
                style.background,
                Some(Background::Color(Color { a: 0.0, .. }))
            )
    );
    assert_eq!(style.text_color, Color::from(theme.primary.base));
    assert_eq!(style.border.color, Color::from(theme.primary.base));
}

#[test]
fn test_button_style_disabled() {
    let theme = Theme::element_light();
    let style = style_sheets::button_style(
        &theme,
        style_sheets::ButtonKind::Primary,
        false,
        button::Status::Disabled,
    );
    // Disabled 态：背景应该淡化（alpha 降低），文字色淡化
    match style.background {
        Some(Background::Color(c)) => assert!(c.a < 1.0, "disabled bg alpha 应 < 1, got {}", c.a),
        _ => panic!("disabled 应该有 Color 背景"),
    }
}

#[test]
fn test_button_style_hovered_state_changed() {
    let theme = Theme::element_light();
    let active = style_sheets::button_style(
        &theme,
        style_sheets::ButtonKind::Primary,
        false,
        button::Status::Active,
    );
    let hovered = style_sheets::button_style(
        &theme,
        style_sheets::ButtonKind::Primary,
        false,
        button::Status::Hovered,
    );
    // Hovered 态的背景应该比 Active 态亮（用 light-3 或 light-5）
    assert_ne!(
        active.background, hovered.background,
        "hovered 应有不同背景"
    );
}

// ============== R.1.2 style_sheets：Input (text_input) style ==============

#[test]
fn test_input_style_active() {
    let theme = Theme::element_light();
    let style = style_sheets::input_style(&theme, text_input::Status::Active);
    // 背景：白
    match style.background {
        Background::Color(c) => assert_eq!(c, Color::from(theme.neutral.bg_overlay)),
        _ => panic!("input active 应有 Color 背景"),
    }
    // 边框：border_base
    assert_eq!(style.border.color, Color::from(theme.neutral.border_base));
    // 文字值色：text_regular
    assert_eq!(style.value, Color::from(theme.neutral.text_regular));
    // placeholder 色：text_placeholder
    assert_eq!(
        style.placeholder,
        Color::from(theme.neutral.text_placeholder)
    );
}

#[test]
fn test_input_style_focused() {
    let theme = Theme::element_light();
    let style =
        style_sheets::input_style(&theme, text_input::Status::Focused { is_hovered: false });
    // Focused 态：边框色 = primary
    assert_eq!(style.border.color, Color::from(theme.primary.base));
    assert!(style.border.width >= 1.0);
}

#[test]
fn test_input_style_disabled() {
    let theme = Theme::element_light();
    let style = style_sheets::input_style(&theme, text_input::Status::Disabled);
    // Disabled 态：背景 = bg_base（浅灰）
    match style.background {
        Background::Color(c) => assert_eq!(c, Color::from(theme.neutral.bg_base)),
        _ => panic!("input disabled 应有 Color 背景"),
    }
}

// ============== R.1.2 style_sheets：Container style ==============

#[test]
fn test_container_style_card() {
    let theme = Theme::element_light();
    let style = style_sheets::container_card_style(&theme);
    // 背景：白
    match style.background {
        Some(Background::Color(c)) => assert_eq!(c, Color::from(theme.neutral.bg_overlay)),
        _ => panic!("card 应有 Color 背景"),
    }
    // 边框：border_lighter
    assert_eq!(
        style.border.color,
        Color::from(theme.neutral.border_lighter)
    );
    // 阴影：shadow.base
    assert_ne!(style.shadow.color, Color::TRANSPARENT);
    assert!(style.shadow.blur_radius > 0.0);
}

#[test]
fn test_container_style_dialog_mask() {
    let theme = Theme::element_light();
    let style = style_sheets::container_dialog_mask_style(&theme);
    // 背景：半透明黑
    match style.background {
        Some(Background::Color(c)) => {
            assert!(c.a > 0.0 && c.a < 1.0, "mask 应有半透明背景, got a={}", c.a);
        }
        _ => panic!("mask 应有 Color 背景"),
    }
}

#[test]
fn test_container_style_default() {
    let theme = Theme::element_light();
    let style = style_sheets::container_default_style(&theme);
    // 默认 container：无背景、无边框
    assert!(style.background.is_none());
    assert_eq!(style.border, Border::default());
    let _ = Shadow::default(); // 仅引用 Shadow 避免未使用 import 警告
}
