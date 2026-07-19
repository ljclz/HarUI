//! HarUI 主题令牌测试 — 字体/间距/圆角/阴影/Z-index
//!
//! TDD RED 阶段：先写测试，预期失败。
//! 这些值来自 Element Plus 官方 SCSS 变量。

use har_ui_core::theme::{
    typography::{Typography, FontSize, FontWeight, LineHeight},
    spacing::Spacing,
    radius::Radius,
    shadow::Shadow,
    zindex::ZIndex,
};

// ============ 字体系统 ============

#[test]
fn test_font_sizes() {
    let typo = Typography::default();

    // Element Plus 字号: 12, 14, 16, 18, 20, 24, 28, 32, 36, 40
    assert_eq!(typo.size(FontSize::Xs), 12.0);
    assert_eq!(typo.size(FontSize::Sm), 14.0);     // 基础字号
    assert_eq!(typo.size(FontSize::Md), 16.0);
    assert_eq!(typo.size(FontSize::Lg), 18.0);
    assert_eq!(typo.size(FontSize::Xl), 20.0);
    assert_eq!(typo.size(FontSize::Xxl), 24.0);
    assert_eq!(typo.size(FontSize::Xxxl), 28.0);
    assert_eq!(typo.size(FontSize::Xxxxl), 32.0);
    assert_eq!(typo.size(FontSize::Xxxxxl), 36.0);
    assert_eq!(typo.size(FontSize::Xxxxxxl), 40.0);
}

#[test]
fn test_default_font_size_is_sm() {
    let typo = Typography::default();
    assert_eq!(typo.base_size(), 14.0);  // Element Plus 默认字号 14px
}

#[test]
fn test_font_weights() {
    let typo = Typography::default();
    assert_eq!(typo.weight(FontWeight::Normal), 400);
    assert_eq!(typo.weight(FontWeight::Medium), 500);
    assert_eq!(typo.weight(FontWeight::Bold), 700);
}

#[test]
fn test_line_heights() {
    let typo = Typography::default();
    assert_eq!(typo.line_height(LineHeight::Tight), 1.2);
    assert_eq!(typo.line_height(LineHeight::Normal), 1.5);
    assert_eq!(typo.line_height(LineHeight::Loose), 1.8);
}

#[test]
fn test_default_font_family() {
    let typo = Typography::default();
    // Element Plus 默认字体栈
    assert!(typo.font_family().contains("Helvetica Neue"));
    assert!(typo.font_family().contains("PingFang SC"));
    assert!(typo.font_family().contains("Microsoft YaHei"));
}

// ============ 间距系统（4px 网格）============

#[test]
fn test_spacing_4px_grid() {
    let s = Spacing::default();
    // Element Plus 间距: 4px 网格 (n*4)
    assert_eq!(s.xxs, 2.0);    // 2px 微间距
    assert_eq!(s.xs, 4.0);     // 4px
    assert_eq!(s.sm, 8.0);     // 8px
    assert_eq!(s.md, 12.0);    // 12px
    assert_eq!(s.base, 16.0);  // 16px 基础间距
    assert_eq!(s.lg, 20.0);    // 20px
    assert_eq!(s.xl, 24.0);    // 24px
    assert_eq!(s.xxl, 32.0);   // 32px
    assert_eq!(s.xxxl, 40.0);  // 40px
}

#[test]
fn test_spacing_at() {
    let s = Spacing::default();
    assert_eq!(s.at(0), 0.0);
    assert_eq!(s.at(1), 4.0);
    assert_eq!(s.at(2), 8.0);
    assert_eq!(s.at(3), 12.0);
    assert_eq!(s.at(4), 16.0);
    assert_eq!(s.at(6), 24.0);
    assert_eq!(s.at(8), 32.0);
    assert_eq!(s.at(10), 40.0);
    assert_eq!(s.at(12), 48.0);
}

// ============ 圆角系统 ============

#[test]
fn test_radius_values() {
    let r = Radius::default();
    // Element Plus 圆角: 0, 2, 4, 8, 12, 16, 20, 24, 50%
    assert_eq!(r.none, 0.0);
    assert_eq!(r.sm, 2.0);
    assert_eq!(r.base, 4.0);    // 默认圆角
    assert_eq!(r.md, 6.0);
    assert_eq!(r.lg, 8.0);
    assert_eq!(r.xl, 12.0);
    assert_eq!(r.xxl, 16.0);
    assert_eq!(r.round, 20.0);
    assert_eq!(r.circle, 9999.0);  // 圆形
}

#[test]
fn test_radius_at() {
    let r = Radius::default();
    assert_eq!(r.at(0.0), 0.0);
    assert_eq!(r.at(2.0), 2.0);
    assert_eq!(r.at(4.0), 4.0);
    assert_eq!(r.at(8.0), 8.0);
}

// ============ 阴影系统 ============

#[test]
fn test_shadow_values() {
    let sh = Shadow::default();

    // Element Plus 阴影（基础、中等、深色）
    // --el-box-shadow: 0px 0px 12px rgba(0, 0, 0, 0.12)
    assert_eq!(sh.base.offset_y, 12.0);
    assert_eq!(sh.base.blur, 12.0);
    assert_eq!(sh.base.color.a, 0.12);

    // --el-box-shadow-light: 0px 0px 8px rgba(0, 0, 0, 0.12)
    assert_eq!(sh.light.offset_y, 8.0);
    assert_eq!(sh.light.blur, 8.0);
    assert_eq!(sh.light.color.a, 0.12);

    // --el-box-shadow-lighter: 0px 0px 6px rgba(0, 0, 0, 0.12)
    assert_eq!(sh.lighter.offset_y, 6.0);
    assert_eq!(sh.lighter.blur, 6.0);
    assert_eq!(sh.lighter.color.a, 0.12);

    // --el-box-shadow-dark: 0px 0px 16px rgba(0, 0, 0, 0.16)
    assert_eq!(sh.dark.offset_y, 16.0);
    assert_eq!(sh.dark.blur, 16.0);
    assert_eq!(sh.dark.color.a, 0.16);
}

#[test]
fn test_shadow_to_string() {
    let sh = Shadow::default();
    let s = sh.base.to_css();
    // 应该包含 "0px 0px 12px"
    assert!(s.contains("12px"), "shadow css should contain 12px, got: {}", s);
    assert!(s.contains("rgba"), "shadow css should contain rgba, got: {}", s);
}

// ============ Z-index 层级系统 ============

#[test]
fn test_zindex_values() {
    let z = ZIndex::default();

    // Element Plus 层级（从低到高）
    assert!(z.normal < z.dropdown);
    assert!(z.dropdown < z.sticky);
    assert!(z.sticky < z.fixed);
    assert!(z.fixed < z.modal_mask);
    assert!(z.modal_mask < z.modal);
    assert!(z.modal < z.popover);
    assert!(z.popover < z.tooltip);
    assert!(z.tooltip < z.notification);
    assert!(z.notification < z.message_box);

    // 验证具体数值（来自 Element Plus）
    assert_eq!(z.normal, 1);
    assert_eq!(z.dropdown, 1000);
    assert_eq!(z.sticky, 1100);
    assert_eq!(z.fixed, 1200);
    assert_eq!(z.modal_mask, 2000);
    assert_eq!(z.modal, 2001);
    assert_eq!(z.popover, 3000);
    assert_eq!(z.tooltip, 3100);
    assert_eq!(z.notification, 4000);
    assert_eq!(z.message_box, 4001);
}
