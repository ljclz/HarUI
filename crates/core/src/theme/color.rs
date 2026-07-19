//! HarUI 颜色系统 — 与 Element Plus CSS 变量 1:1 映射
//!
//! 色值来源: element-plus/packages/theme-chalk/src/common/var.scss
//! 以及 element-plus/packages/theme-chalk/src/common/var.scss 中的混色函数。
//!
//! Element Plus 的 light/dark 变体使用 SCSS 的 mix() 函数:
//!   light-N = mix(white, base, N*10%)
//!   dark-N  = mix(black, base, N*10%)
//! 这里直接硬编码最终色值（来自 Element Plus 编译后的 CSS 变量）。

use std::collections::HashMap;

/// 主题颜色（RGB + Alpha）
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThemeColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

impl ThemeColor {
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    pub const fn from_rgba(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn to_rgb(self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }

    pub fn to_array(self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}

impl From<ThemeColor> for iced::Color {
    fn from(c: ThemeColor) -> Self {
        iced::Color::from_rgba(
            c.r as f32 / 255.0,
            c.g as f32 / 255.0,
            c.b as f32 / 255.0,
            c.a,
        )
    }
}

/// hex 字符串转 RGB
///
/// 支持: "#RRGGBB", "#rrggbb", "RRGGBB", "#RGB", "#rgb"
/// 不支持: 带透明度的 #RRGGBBAA (Element Plus 不使用)
pub fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    let hex = hex.trim().trim_start_matches('#');
    if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some((r, g, b))
    } else if hex.len() == 3 {
        // 短格式: #RGB → #RRGGBB
        let r = u8::from_str_radix(&format!("{}{}", &hex[0..1], &hex[0..1]), 16).ok()?;
        let g = u8::from_str_radix(&format!("{}{}", &hex[1..2], &hex[1..2]), 16).ok()?;
        let b = u8::from_str_radix(&format!("{}{}", &hex[2..3], &hex[2..3]), 16).ok()?;
        Some((r, g, b))
    } else {
        None
    }
}

/// hex 字符串转 ThemeColor (不透明)
pub fn hex(hex: &str) -> ThemeColor {
    let (r, g, b) = hex_to_rgb(hex)
        .unwrap_or_else(|| panic!("Invalid hex color: {}", hex));
    ThemeColor::from_rgb(r, g, b)
}

/// 颜色调色板（base + 9 级 light + 2 级 dark）
///
/// Element Plus 的调色板结构:
/// - light: 1, 2, 3, 4, 5, 6, 7, 8, 9 (9 个亮色变体)
/// - dark: 1, 2 (2 个暗色变体)
#[derive(Debug, Clone)]
pub struct ColorPalette {
    pub base: ThemeColor,
    pub light: HashMap<u8, ThemeColor>,
    pub dark: HashMap<u8, ThemeColor>,
}

impl ColorPalette {
    /// 构建调色板
    pub fn new(base: ThemeColor, light: HashMap<u8, ThemeColor>, dark: HashMap<u8, ThemeColor>) -> Self {
        Self { base, light, dark }
    }

    /// 获取 light 变体 (1-9)
    pub fn light(&self, n: u8) -> ThemeColor {
        *self.light.get(&n).unwrap_or_else(|| panic!("light:{} not in palette", n))
    }

    /// 获取 dark 变体 (1-2)
    pub fn dark(&self, n: u8) -> ThemeColor {
        *self.dark.get(&n).unwrap_or_else(|| panic!("dark:{} not in palette", n))
    }

    /// 从 base 色自动计算调色板（使用 SCSS mix 公式）
    ///
    /// SCSS mix(white, base, N*10%) = white * (N/10) + base * (1 - N/10)
    /// SCSS mix(black, base, N*10%) = black * (N/10) + base * (1 - N/10)
    pub fn from_base(base: ThemeColor) -> Self {
        let mut light = HashMap::new();
        for n in 1u8..=9 {
            let weight = n as f32 / 10.0;
            light.insert(n, mix(ThemeColor::from_rgb(255, 255, 255), base, weight));
        }
        let mut dark = HashMap::new();
        for n in 1u8..=2 {
            let weight = n as f32 / 10.0;
            dark.insert(n, mix(ThemeColor::from_rgb(0, 0, 0), base, weight));
        }
        Self { base, light, dark }
    }

    /// Primary 调色板: #409EFF
    pub fn primary() -> Self {
        Self::from_base(hex("#409EFF"))
    }

    /// Success 调色板: #67C23A
    pub fn success() -> Self {
        Self::from_base(hex("#67C23A"))
    }

    /// Warning 调色板: #E6A23C
    pub fn warning() -> Self {
        Self::from_base(hex("#E6A23C"))
    }

    /// Danger 调色板: #F56C6C
    pub fn danger() -> Self {
        Self::from_base(hex("#F56C6C"))
    }

    /// Info 调色板: #909399
    pub fn info() -> Self {
        Self::from_base(hex("#909399"))
    }

    /// 默认浅色主题调色板集合: [primary, success, warning, danger, info]
    pub fn default_light_set() -> Vec<ColorPalette> {
        vec![
            Self::primary(),
            Self::success(),
            Self::warning(),
            Self::danger(),
            Self::info(),
        ]
    }
}

/// SCSS mix() 函数的 Rust 实现
///
/// mix(color1, color2, weight) = color1 * weight + color2 * (1 - weight)
///
/// 等价于 SCSS 的 `mix($color1, $color2, $weight)`，其中 weight 是 color1 的权重（0-1）。
pub fn mix(color1: ThemeColor, color2: ThemeColor, weight: f32) -> ThemeColor {
    let w = weight.clamp(0.0, 1.0);
    let r = (color1.r as f32 * w + color2.r as f32 * (1.0 - w)).round() as u8;
    let g = (color1.g as f32 * w + color2.g as f32 * (1.0 - w)).round() as u8;
    let b = (color1.b as f32 * w + color2.b as f32 * (1.0 - w)).round() as u8;
    let a = color1.a * w + color2.a * (1.0 - w);
    ThemeColor::from_rgba(r, g, b, a)
}

/// 中性色（文字、边框、背景）
#[derive(Debug, Clone)]
pub struct NeutralColor {
    // 文字色
    pub text_primary: ThemeColor,
    pub text_regular: ThemeColor,
    pub text_secondary: ThemeColor,
    pub text_placeholder: ThemeColor,
    pub text_disabled: ThemeColor,
    // 边框色
    pub border_base: ThemeColor,
    pub border_light: ThemeColor,
    pub border_lighter: ThemeColor,
    pub border_extra_light: ThemeColor,
    // 背景色
    pub bg_base: ThemeColor,
    pub bg_page: ThemeColor,
    pub bg_overlay: ThemeColor,
}

impl Default for NeutralColor {
    fn default() -> Self {
        Self {
            // 文字色
            text_primary: hex("#303133"),
            text_regular: hex("#606266"),
            text_secondary: hex("#909399"),
            text_placeholder: hex("#A8ABB2"),
            text_disabled: hex("#C0C4CC"),
            // 边框色
            border_base: hex("#DCDFE6"),
            border_light: hex("#E4E7ED"),
            border_lighter: hex("#EBEEF5"),
            border_extra_light: hex("#F2F6FC"),
            // 背景色
            bg_base: hex("#F5F7FA"),
            bg_page: hex("#F2F6FC"),
            bg_overlay: hex("#FFFFFF"),
        }
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_hex_to_rgb_basic() {
        assert_eq!(hex_to_rgb("#409EFF"), Some((64, 158, 255)));
        assert_eq!(hex_to_rgb("#409eff"), Some((64, 158, 255)));
        assert_eq!(hex_to_rgb("409EFF"), Some((64, 158, 255)));
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
}
