//! HarUI 颜色系统 — TDD 测试
//!
//! 测试策略：
//! 1. hex_to_rgb 基础转换测试
//! 2. 已知的 Element Plus 官方色值（从源码 var.scss 直接摘录的 base 色）
//! 3. mix() 公式验证（与 SCSS mix 函数等价）
//! 4. 调色板 light/dark 变体由 mix 公式计算，测试验证公式正确性

use har_ui_core::theme::color::{ColorPalette, NeutralColor, ThemeColor, hex_to_rgb, mix};

/// Element Plus 官方 base 色值（来自 element-plus/packages/theme-chalk/src/common/var.scss）
/// 这些是硬编码的"真相"，任何实现必须匹配。
mod element_plus_truth {
    /// primary: #409EFF
    pub const PRIMARY: (u8, u8, u8) = (64, 158, 255);
    /// success: #67C23A
    pub const SUCCESS: (u8, u8, u8) = (103, 194, 58);
    /// warning: #E6A23C
    pub const WARNING: (u8, u8, u8) = (230, 162, 60);
    /// danger: #F56C6C
    pub const DANGER: (u8, u8, u8) = (245, 108, 108);
    /// info: #909399
    pub const INFO: (u8, u8, u8) = (144, 147, 153);

    /// 已知的 Element Plus 编译后色值（用于验证 mix 公式正确性）
    /// 来源：Element Plus 官方文档示例页面 inspect 的 CSS 变量值
    pub const PRIMARY_LIGHT_3: (u8, u8, u8) = (121, 187, 255); // #79bbff
    pub const PRIMARY_LIGHT_5: (u8, u8, u8) = (160, 207, 255); // #a0cfff
    pub const PRIMARY_LIGHT_7: (u8, u8, u8) = (198, 226, 255); // #c6e2ff
    pub const PRIMARY_LIGHT_9: (u8, u8, u8) = (236, 245, 255); // #ecf5ff
    pub const PRIMARY_DARK_2: (u8, u8, u8) = (51, 126, 204); // #337ecc

    pub const SUCCESS_LIGHT_3: (u8, u8, u8) = (149, 212, 117); // #95d475
    pub const SUCCESS_LIGHT_5: (u8, u8, u8) = (179, 225, 157); // #b3e19d
    pub const SUCCESS_LIGHT_9: (u8, u8, u8) = (240, 249, 235); // #f0f9eb
    pub const SUCCESS_DARK_2: (u8, u8, u8) = (82, 155, 46); // #529b2e
}

#[test]
fn test_hex_to_rgb_primary() {
    assert_eq!(hex_to_rgb("#409EFF"), Some(element_plus_truth::PRIMARY));
    assert_eq!(hex_to_rgb("#409eff"), Some(element_plus_truth::PRIMARY));
    assert_eq!(hex_to_rgb("409EFF"), Some(element_plus_truth::PRIMARY));
}

#[test]
fn test_hex_to_rgb_all_theme_colors() {
    assert_eq!(hex_to_rgb("#67C23A"), Some(element_plus_truth::SUCCESS));
    assert_eq!(hex_to_rgb("#E6A23C"), Some(element_plus_truth::WARNING));
    assert_eq!(hex_to_rgb("#F56C6C"), Some(element_plus_truth::DANGER));
    assert_eq!(hex_to_rgb("#909399"), Some(element_plus_truth::INFO));
}

#[test]
fn test_hex_to_rgb_invalid() {
    assert_eq!(hex_to_rgb("#GGGGGG"), None);
    assert_eq!(hex_to_rgb("#12345"), None);
    assert_eq!(hex_to_rgb(""), None);
}

#[test]
fn test_hex_to_rgb_short() {
    assert_eq!(hex_to_rgb("#fff"), Some((255, 255, 255)));
    assert_eq!(hex_to_rgb("#000"), Some((0, 0, 0)));
}

/// 验证 SCSS mix 公式：mix(white, base, 30%) 应等于 Element Plus 已知的 light-3 值
#[test]
fn test_mix_formula_primary_light_3() {
    let white = ThemeColor::from_rgb(255, 255, 255);
    let primary = ThemeColor::from_rgb(64, 158, 255);
    let light_3 = mix(white, primary, 0.3);
    assert_eq!(light_3.to_rgb(), element_plus_truth::PRIMARY_LIGHT_3);
}

#[test]
fn test_mix_formula_primary_light_9() {
    let white = ThemeColor::from_rgb(255, 255, 255);
    let primary = ThemeColor::from_rgb(64, 158, 255);
    let light_9 = mix(white, primary, 0.9);
    assert_eq!(light_9.to_rgb(), element_plus_truth::PRIMARY_LIGHT_9);
}

#[test]
fn test_mix_formula_primary_dark_2() {
    let black = ThemeColor::from_rgb(0, 0, 0);
    let primary = ThemeColor::from_rgb(64, 158, 255);
    let dark_2 = mix(black, primary, 0.2);
    assert_eq!(dark_2.to_rgb(), element_plus_truth::PRIMARY_DARK_2);
}

#[test]
fn test_mix_formula_success_light_3() {
    let white = ThemeColor::from_rgb(255, 255, 255);
    let success = ThemeColor::from_rgb(103, 194, 58);
    let light_3 = mix(white, success, 0.3);
    assert_eq!(light_3.to_rgb(), element_plus_truth::SUCCESS_LIGHT_3);
}

#[test]
fn test_mix_formula_success_dark_2() {
    let black = ThemeColor::from_rgb(0, 0, 0);
    let success = ThemeColor::from_rgb(103, 194, 58);
    let dark_2 = mix(black, success, 0.2);
    assert_eq!(dark_2.to_rgb(), element_plus_truth::SUCCESS_DARK_2);
}

/// 验证调色板（基于 mix 公式自动计算的变体应与已知 Element Plus 值匹配）
#[test]
fn test_primary_color_palette() {
    let palette = ColorPalette::primary();

    assert_eq!(palette.base.to_rgb(), element_plus_truth::PRIMARY);
    assert_eq!(
        palette.light(3).to_rgb(),
        element_plus_truth::PRIMARY_LIGHT_3
    );
    assert_eq!(
        palette.light(5).to_rgb(),
        element_plus_truth::PRIMARY_LIGHT_5
    );
    assert_eq!(
        palette.light(7).to_rgb(),
        element_plus_truth::PRIMARY_LIGHT_7
    );
    assert_eq!(
        palette.light(9).to_rgb(),
        element_plus_truth::PRIMARY_LIGHT_9
    );
    assert_eq!(palette.dark(2).to_rgb(), element_plus_truth::PRIMARY_DARK_2);
}

#[test]
fn test_success_color_palette() {
    let palette = ColorPalette::success();

    assert_eq!(palette.base.to_rgb(), element_plus_truth::SUCCESS);
    assert_eq!(
        palette.light(3).to_rgb(),
        element_plus_truth::SUCCESS_LIGHT_3
    );
    assert_eq!(
        palette.light(5).to_rgb(),
        element_plus_truth::SUCCESS_LIGHT_5
    );
    assert_eq!(
        palette.light(9).to_rgb(),
        element_plus_truth::SUCCESS_LIGHT_9
    );
    assert_eq!(palette.dark(2).to_rgb(), element_plus_truth::SUCCESS_DARK_2);
}

#[test]
fn test_warning_color_palette() {
    let palette = ColorPalette::warning();
    assert_eq!(palette.base.to_rgb(), element_plus_truth::WARNING);
    // 验证 mix 公式内部一致性
    let expected_light_5 = mix(
        ThemeColor::from_rgb(255, 255, 255),
        ThemeColor::from_rgb(230, 162, 60),
        0.5,
    );
    assert_eq!(palette.light(5).to_rgb(), expected_light_5.to_rgb());
}

#[test]
fn test_danger_color_palette() {
    let palette = ColorPalette::danger();
    assert_eq!(palette.base.to_rgb(), element_plus_truth::DANGER);
    let expected_light_5 = mix(
        ThemeColor::from_rgb(255, 255, 255),
        ThemeColor::from_rgb(245, 108, 108),
        0.5,
    );
    assert_eq!(palette.light(5).to_rgb(), expected_light_5.to_rgb());
}

#[test]
fn test_info_color_palette() {
    let palette = ColorPalette::info();
    assert_eq!(palette.base.to_rgb(), element_plus_truth::INFO);
    let expected_light_5 = mix(
        ThemeColor::from_rgb(255, 255, 255),
        ThemeColor::from_rgb(144, 147, 153),
        0.5,
    );
    assert_eq!(palette.light(5).to_rgb(), expected_light_5.to_rgb());
}

#[test]
fn test_theme_color_to_iced_color() {
    let theme_color = ThemeColor::from_rgb(64, 158, 255);
    let iced_color: iced::Color = theme_color.into();
    assert_eq!(
        iced_color,
        iced::Color::from_rgb(64.0 / 255.0, 158.0 / 255.0, 255.0 / 255.0,)
    );
}

#[test]
fn test_theme_color_with_alpha() {
    let theme_color = ThemeColor::from_rgba(64, 158, 255, 0.5);
    let iced_color: iced::Color = theme_color.into();
    assert_eq!(iced_color.a, 0.5);
}

#[test]
fn test_default_light_palette_set() {
    let palettes = ColorPalette::default_light_set();
    assert_eq!(palettes.len(), 5);
    assert_eq!(palettes[0].base.to_rgb(), element_plus_truth::PRIMARY);
    assert_eq!(palettes[1].base.to_rgb(), element_plus_truth::SUCCESS);
    assert_eq!(palettes[2].base.to_rgb(), element_plus_truth::WARNING);
    assert_eq!(palettes[3].base.to_rgb(), element_plus_truth::DANGER);
    assert_eq!(palettes[4].base.to_rgb(), element_plus_truth::INFO);
}

#[test]
fn test_neutral_colors() {
    let neutral = NeutralColor::default();

    // 文字主色: #303133
    assert_eq!(neutral.text_primary.to_rgb(), (48, 49, 51));
    // 文字常规: #606266
    assert_eq!(neutral.text_regular.to_rgb(), (96, 98, 102));
    // 文字次要: #909399
    assert_eq!(neutral.text_secondary.to_rgb(), (144, 147, 153));
    // 文字占位: #A8ABB2
    assert_eq!(neutral.text_placeholder.to_rgb(), (168, 171, 178));

    // 边框色: #DCDFE6
    assert_eq!(neutral.border_base.to_rgb(), (220, 223, 230));
    // 浅边框: #E4E7ED
    assert_eq!(neutral.border_light.to_rgb(), (228, 231, 237));
    // 更浅边框: #EBEEF5
    assert_eq!(neutral.border_lighter.to_rgb(), (235, 238, 245));
    // 最浅边框: #F2F6FC
    assert_eq!(neutral.border_extra_light.to_rgb(), (242, 246, 252));

    // 背景: #F5F7FA
    assert_eq!(neutral.bg_base.to_rgb(), (245, 247, 250));
    // 页面背景: #F2F6FC
    assert_eq!(neutral.bg_page.to_rgb(), (242, 246, 252));
    // 覆盖背景: #FFFFFF
    assert_eq!(neutral.bg_overlay.to_rgb(), (255, 255, 255));
}
