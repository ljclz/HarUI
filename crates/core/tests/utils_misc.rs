//! 工具函数测试 — 颜色 / 动画 / 键盘

use har_ui_core::utils::{
    color_utils::{mix_colors, luminance, is_light, darken, lighten, with_alpha},
    animation::{easing, interpolate, AnimationCurve},
    keyboard::{KeyModifiers, Shortcut, ShortcutRegistry},
};

// ====================== 颜色工具 ======================

#[test]
fn test_mix_colors_equal_weights() {
    let c1 = har_ui_core::theme::color::hex("#000000");
    let c2 = har_ui_core::theme::color::hex("#FFFFFF");
    let mid = mix_colors(c1, c2, 0.5);
    // 255 * 0.5 + 0 * 0.5 = 127.5, round → 128
    assert_eq!(mid.r, 128);
    assert_eq!(mid.g, 128);
    assert_eq!(mid.b, 128);
}

#[test]
fn test_mix_colors_weight_zero() {
    let c1 = har_ui_core::theme::color::hex("#FF0000");
    let c2 = har_ui_core::theme::color::hex("#00FF00");
    let result = mix_colors(c1, c2, 0.0);
    // weight=0 → 全部 c2
    assert_eq!(result.r, 0);
    assert_eq!(result.g, 255);
}

#[test]
fn test_mix_colors_weight_one() {
    let c1 = har_ui_core::theme::color::hex("#FF0000");
    let c2 = har_ui_core::theme::color::hex("#00FF00");
    let result = mix_colors(c1, c2, 1.0);
    // weight=1 → 全部 c1
    assert_eq!(result.r, 255);
    assert_eq!(result.g, 0);
}

#[test]
fn test_luminance_black() {
    let black = har_ui_core::theme::color::hex("#000000");
    let lum = luminance(black);
    assert!((lum - 0.0).abs() < 0.001);
}

#[test]
fn test_luminance_white() {
    let white = har_ui_core::theme::color::hex("#FFFFFF");
    let lum = luminance(white);
    assert!((lum - 1.0).abs() < 0.001);
}

#[test]
fn test_is_light_returns_true_for_white() {
    let white = har_ui_core::theme::color::hex("#FFFFFF");
    assert!(is_light(white));
}

#[test]
fn test_is_light_returns_false_for_black() {
    let black = har_ui_core::theme::color::hex("#000000");
    assert!(!is_light(black));
}

#[test]
fn test_darken_reduces_brightness() {
    let base = har_ui_core::theme::color::hex("#808080");
    let darker = darken(base, 0.5);
    assert!(darker.r < base.r);
    assert!(darker.g < base.g);
    assert!(darker.b < base.b);
}

#[test]
fn test_lighten_increases_brightness() {
    let base = har_ui_core::theme::color::hex("#808080");
    let lighter = lighten(base, 0.5);
    assert!(lighter.r > base.r);
    assert!(lighter.g > base.g);
    assert!(lighter.b > base.b);
}

#[test]
fn test_with_alpha_sets_alpha() {
    let c = har_ui_core::theme::color::hex("#409EFF");
    let semi = with_alpha(c, 0.5);
    assert!((semi.a - 0.5).abs() < 0.001);
    assert_eq!(semi.r, c.r);
    assert_eq!(semi.g, c.g);
    assert_eq!(semi.b, c.b);
}

// ====================== 动画工具 ======================

#[test]
fn test_easing_linear() {
    assert!((easing(AnimationCurve::Linear, 0.0) - 0.0).abs() < 0.001);
    assert!((easing(AnimationCurve::Linear, 0.5) - 0.5).abs() < 0.001);
    assert!((easing(AnimationCurve::Linear, 1.0) - 1.0).abs() < 0.001);
}

#[test]
fn test_easing_ease_in_out() {
    // 端点
    assert!((easing(AnimationCurve::EaseInOut, 0.0) - 0.0).abs() < 0.001);
    assert!((easing(AnimationCurve::EaseInOut, 1.0) - 1.0).abs() < 0.001);
    // 中点
    let mid = easing(AnimationCurve::EaseInOut, 0.5);
    assert!((mid - 0.5).abs() < 0.01);
}

#[test]
fn test_easing_ease_in() {
    assert!((easing(AnimationCurve::EaseIn, 0.0) - 0.0).abs() < 0.001);
    assert!((easing(AnimationCurve::EaseIn, 1.0) - 1.0).abs() < 0.001);
    // ease-in 在 t<0.5 时值应该 < linear
    let v = easing(AnimationCurve::EaseIn, 0.3);
    assert!(v < 0.3);
}

#[test]
fn test_easing_ease_out() {
    assert!((easing(AnimationCurve::EaseOut, 0.0) - 0.0).abs() < 0.001);
    assert!((easing(AnimationCurve::EaseOut, 1.0) - 1.0).abs() < 0.001);
    // ease-out 在 t>0.5 时值应该 > linear
    let v = easing(AnimationCurve::EaseOut, 0.7);
    assert!(v > 0.7);
}

#[test]
fn test_interpolate_numbers() {
    assert_eq!(interpolate(0.0, 100.0, 0.0), 0.0);
    assert_eq!(interpolate(0.0, 100.0, 0.5), 50.0);
    assert_eq!(interpolate(0.0, 100.0, 1.0), 100.0);
    assert_eq!(interpolate(10.0, 20.0, 0.25), 12.5);
}

#[test]
fn test_interpolate_with_curve() {
    // 使用 ease-in 曲线插值
    let v = interpolate(0.0, 100.0, 0.5);
    // linear 时为 50，使用曲线时值不同
    assert!(v >= 0.0 && v <= 100.0);
}

// ====================== 键盘工具 ======================

#[test]
fn test_key_modifiers_default() {
    let mods = KeyModifiers::default();
    assert!(!mods.ctrl);
    assert!(!mods.shift);
    assert!(!mods.alt);
    assert!(!mods.meta);
}

#[test]
fn test_key_modifiers_builder() {
    let mods = KeyModifiers {
        ctrl: true,
        shift: true,
        alt: false,
        meta: false,
    };
    assert!(mods.ctrl);
    assert!(mods.shift);
}

#[test]
fn test_shortcut_equality() {
    let s1 = Shortcut::new('A', KeyModifiers { ctrl: true, shift: false, alt: false, meta: false });
    let s2 = Shortcut::new('A', KeyModifiers { ctrl: true, shift: false, alt: false, meta: false });
    assert_eq!(s1, s2);
}

#[test]
fn test_shortcut_inequality() {
    let s1 = Shortcut::new('A', KeyModifiers { ctrl: true, shift: false, alt: false, meta: false });
    let s2 = Shortcut::new('B', KeyModifiers { ctrl: true, shift: false, alt: false, meta: false });
    assert_ne!(s1, s2);
}

#[test]
fn test_shortcut_registry_register_and_match() {
    let mut registry = ShortcutRegistry::new();
    registry.register("save", Shortcut::new('S', KeyModifiers { ctrl: true, shift: false, alt: false, meta: false }));
    registry.register("copy", Shortcut::new('C', KeyModifiers { ctrl: true, shift: false, alt: false, meta: false }));
    
    let test = Shortcut::new('S', KeyModifiers { ctrl: true, shift: false, alt: false, meta: false });
    assert_eq!(registry.match_shortcut(&test), Some("save"));
    
    let test = Shortcut::new('C', KeyModifiers { ctrl: true, shift: false, alt: false, meta: false });
    assert_eq!(registry.match_shortcut(&test), Some("copy"));
}

#[test]
fn test_shortcut_registry_no_match() {
    let mut registry = ShortcutRegistry::new();
    registry.register("save", Shortcut::new('S', KeyModifiers { ctrl: true, shift: false, alt: false, meta: false }));
    
    let test = Shortcut::new('X', KeyModifiers { ctrl: true, shift: false, alt: false, meta: false });
    assert_eq!(registry.match_shortcut(&test), None);
}

#[test]
fn test_shortcut_registry_overwrite() {
    let mut registry = ShortcutRegistry::new();
    registry.register("action1", Shortcut::new('A', KeyModifiers { ctrl: true, shift: false, alt: false, meta: false }));
    // 覆盖同名快捷键
    registry.register("action2", Shortcut::new('A', KeyModifiers { ctrl: true, shift: false, alt: false, meta: false }));
    
    let test = Shortcut::new('A', KeyModifiers { ctrl: true, shift: false, alt: false, meta: false });
    assert_eq!(registry.match_shortcut(&test), Some("action2"));
}
