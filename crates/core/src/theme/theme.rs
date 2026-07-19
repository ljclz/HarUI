//! HarUI 主题主结构 — 包含全部令牌系统，支持 light/dark 双主题
//!
//! 设计参考: Element Plus 主题 + GPUI Component theme/schema.rs

use crate::theme::{
    color::{ColorPalette, NeutralColor},
    radius::Radius,
    shadow::Shadow,
    spacing::Spacing,
    typography::Typography,
    zindex::ZIndex,
};

/// HarUI 主题
///
/// 包含 Element Plus 全部设计令牌。通过 `Theme::element_light()`
/// 或 `Theme::element_dark()` 创建预置主题。
#[derive(Debug, Clone)]
pub struct Theme {
    /// 主题名称
    pub name: String,
    /// 是否深色主题
    pub is_dark: bool,
    /// Primary 调色板
    pub primary: ColorPalette,
    /// Success 调色板
    pub success: ColorPalette,
    /// Warning 调色板
    pub warning: ColorPalette,
    /// Danger 调色板
    pub danger: ColorPalette,
    /// Info 调色板
    pub info: ColorPalette,
    /// 中性色（文字/边框/背景）
    pub neutral: NeutralColor,
    /// 字体系统
    pub typography: Typography,
    /// 间距系统
    pub spacing: Spacing,
    /// 圆角系统
    pub radius: Radius,
    /// 阴影系统
    pub shadow: Shadow,
    /// Z-index 层级
    pub zindex: ZIndex,
}

impl Default for Theme {
    fn default() -> Self {
        Self::element_light()
    }
}

impl Theme {
    /// Element Plus 浅色主题
    pub fn element_light() -> Self {
        Self {
            name: "element-light".to_string(),
            is_dark: false,
            primary: ColorPalette::primary(),
            success: ColorPalette::success(),
            warning: ColorPalette::warning(),
            danger: ColorPalette::danger(),
            info: ColorPalette::info(),
            neutral: NeutralColor::default(),
            typography: Typography::default(),
            spacing: Spacing::default(),
            radius: Radius::default(),
            shadow: Shadow::default(),
            zindex: ZIndex::default(),
        }
    }

    /// Element Plus 深色主题
    ///
    /// 深色主题的色值参考 element-plus/packages/theme-chalk/src/dark/css-vars.scss
    pub fn element_dark() -> Self {
        use crate::theme::color::hex;
        Self {
            name: "element-dark".to_string(),
            is_dark: true,
            // 5 个主题色在深色模式下保持不变（Element Plus 深色主题 primary 等不变）
            primary: ColorPalette::primary(),
            success: ColorPalette::success(),
            warning: ColorPalette::warning(),
            danger: ColorPalette::danger(),
            info: ColorPalette::info(),
            // 中性色反转
            neutral: NeutralColor {
                // 文字色变浅
                text_primary: hex("#E5EAF3"),
                text_regular: hex("#CFD3DC"),
                text_secondary: hex("#A3A6AD"),
                text_placeholder: hex("#8D9095"),
                text_disabled: hex("#4C4D4F"),
                // 边框色变深
                border_base: hex("#4C4D4F"),
                border_light: hex("#414243"),
                border_lighter: hex("#363637"),
                border_extra_light: hex("#2B2B2C"),
                // 背景色变深
                bg_base: hex("#141414"),
                bg_page: hex("#000000"),
                bg_overlay: hex("#141414"),
            },
            typography: Typography::default(),
            spacing: Spacing::default(),
            radius: Radius::default(),
            shadow: Shadow::default(),
            zindex: ZIndex::default(),
        }
    }
}
