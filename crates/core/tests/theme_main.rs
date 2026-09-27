//! HarUI Theme 主结构测试 — 深浅主题 + 序列化

use har_ui_core::theme::Theme;

#[test]
fn test_default_theme_is_light() {
    let theme = Theme::default();
    assert!(!theme.is_dark);
    assert_eq!(theme.name, "element-light");
}

#[test]
fn test_element_light_theme() {
    let theme = Theme::element_light();
    assert!(!theme.is_dark);
    assert_eq!(theme.name, "element-light");
    // primary 基础色应为 #409EFF
    assert_eq!(theme.primary.base.to_rgb(), (64, 158, 255));
    // 浅色主题背景应为白色
    assert_eq!(theme.neutral.bg_overlay.to_rgb(), (255, 255, 255));
}

#[test]
fn test_element_dark_theme() {
    let theme = Theme::element_dark();
    assert!(theme.is_dark);
    assert_eq!(theme.name, "element-dark");
    // primary 基础色在深色主题下仍为 #409EFF（Element Plus 深色主题 primary 不变）
    assert_eq!(theme.primary.base.to_rgb(), (64, 158, 255));
    // 深色主题背景应为深色
    let bg = theme.neutral.bg_overlay.to_rgb();
    assert!(
        bg.0 < 50 && bg.1 < 50 && bg.2 < 50,
        "dark bg should be dark, got {:?}",
        bg
    );
}

#[test]
fn test_theme_contains_all_palettes() {
    let theme = Theme::element_light();
    assert_eq!(theme.primary.base.to_rgb(), (64, 158, 255));
    assert_eq!(theme.success.base.to_rgb(), (103, 194, 58));
    assert_eq!(theme.warning.base.to_rgb(), (230, 162, 60));
    assert_eq!(theme.danger.base.to_rgb(), (245, 108, 108));
    assert_eq!(theme.info.base.to_rgb(), (144, 147, 153));
}

#[test]
fn test_theme_typography() {
    let theme = Theme::element_light();
    assert_eq!(theme.typography.base_size(), 14.0);
    assert!(theme.typography.font_family().contains("Microsoft YaHei"));
}

#[test]
fn test_theme_spacing() {
    let theme = Theme::element_light();
    assert_eq!(theme.spacing.base, 16.0);
    assert_eq!(theme.spacing.at(4), 16.0);
}

#[test]
fn test_theme_radius() {
    let theme = Theme::element_light();
    assert_eq!(theme.radius.base, 4.0);
    assert_eq!(theme.radius.circle, 9999.0);
}

#[test]
fn test_theme_shadow() {
    let theme = Theme::element_light();
    assert_eq!(theme.shadow.base.offset_y, 12.0);
}

#[test]
fn test_theme_zindex() {
    let theme = Theme::element_light();
    assert!(theme.zindex.modal > theme.zindex.dropdown);
    assert!(theme.zindex.tooltip > theme.zindex.popover);
}

#[test]
fn test_theme_clone_and_modify() {
    let theme = Theme::element_light();
    let mut cloned = theme.clone();
    cloned.name = "custom".to_string();
    assert_eq!(theme.name, "element-light");
    assert_eq!(cloned.name, "custom");
}

#[test]
fn test_dark_theme_neutral_colors_are_dark() {
    let dark = Theme::element_dark();
    // 深色主题文字主色应为浅色
    let text = dark.neutral.text_primary.to_rgb();
    assert!(
        text.0 > 200 && text.1 > 200 && text.2 > 200,
        "dark theme text should be light, got {:?}",
        text
    );
    // 深色主题背景应为深色
    let bg = dark.neutral.bg_overlay.to_rgb();
    assert!(
        bg.0 < 30 && bg.1 < 30 && bg.2 < 30,
        "dark theme bg should be dark, got {:?}",
        bg
    );
}

#[test]
fn test_theme_palette_light_method() {
    let theme = Theme::element_light();
    // 测试通过 Theme 访问调色板变体
    let primary_light_3 = theme.primary.light(3);
    assert_eq!(primary_light_3.to_rgb(), (121, 187, 255));
}
