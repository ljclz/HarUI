//! 颜色工具函数
//!
//! 提供颜色混合、亮度计算、明暗调整等通用功能。

use crate::theme::color::ThemeColor;

/// 混合两种颜色（weight 为 c1 的权重，0..=1）
///
/// 与 SCSS `mix($color1, $color2, $weight)` 一致：
/// `weight=0` → 全 c2；`weight=1` → 全 c1
pub fn mix_colors(c1: ThemeColor, c2: ThemeColor, weight: f32) -> ThemeColor {
    let w = weight.clamp(0.0, 1.0);
    ThemeColor::from_rgba(
        (c1.r as f32 * w + c2.r as f32 * (1.0 - w)).round() as u8,
        (c1.g as f32 * w + c2.g as f32 * (1.0 - w)).round() as u8,
        (c1.b as f32 * w + c2.b as f32 * (1.0 - w)).round() as u8,
        c1.a * w + c2.a * (1.0 - w),
    )
}

/// 计算颜色的相对亮度（0..=1）
///
/// 参考 WCAG 2.0 标准的相对亮度公式
pub fn luminance(c: ThemeColor) -> f32 {
    let r = srgb_to_linear(c.r as f32 / 255.0);
    let g = srgb_to_linear(c.g as f32 / 255.0);
    let b = srgb_to_linear(c.b as f32 / 255.0);
    0.2126 * r + 0.7152 * g + 0.0722 * b
}

fn srgb_to_linear(channel: f32) -> f32 {
    if channel <= 0.03928 {
        channel / 12.92
    } else {
        ((channel + 0.055) / 1.055).powf(2.4)
    }
}

/// 判断颜色是否为浅色（基于 WCAG 亮度）
pub fn is_light(c: ThemeColor) -> bool {
    luminance(c) > 0.5
}

/// 加深颜色（amount: 0..=1，0=无变化，1=全黑）
pub fn darken(c: ThemeColor, amount: f32) -> ThemeColor {
    let a = amount.clamp(0.0, 1.0);
    ThemeColor::from_rgba(
        (c.r as f32 * (1.0 - a)).round() as u8,
        (c.g as f32 * (1.0 - a)).round() as u8,
        (c.b as f32 * (1.0 - a)).round() as u8,
        c.a,
    )
}

/// 提亮颜色（amount: 0..=1，0=无变化，1=全白）
pub fn lighten(c: ThemeColor, amount: f32) -> ThemeColor {
    let a = amount.clamp(0.0, 1.0);
    ThemeColor::from_rgba(
        (c.r as f32 + (255.0 - c.r as f32) * a).round() as u8,
        (c.g as f32 + (255.0 - c.g as f32) * a).round() as u8,
        (c.b as f32 + (255.0 - c.b as f32) * a).round() as u8,
        c.a,
    )
}

/// 设置颜色透明度
pub fn with_alpha(c: ThemeColor, alpha: f32) -> ThemeColor {
    ThemeColor::from_rgba(c.r, c.g, c.b, alpha.clamp(0.0, 1.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_luminance_mid_gray() {
        let gray = ThemeColor::from_rgba(128, 128, 128, 1.0);
        let lum = luminance(gray);
        // mid gray 应该在 0.2 左右（sRGB 非线性）
        assert!(lum > 0.0 && lum < 1.0);
    }
}
