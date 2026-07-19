//! 阴影系统
//!
//! 来源: Element Plus --el-box-shadow-* 变量
//!   --el-box-shadow: 0px 0px 12px rgba(0, 0, 0, 0.12)
//!   --el-box-shadow-light: 0px 0px 8px rgba(0, 0, 0, 0.12)
//!   --el-box-shadow-lighter: 0px 0px 6px rgba(0, 0, 0, 0.12)
//!   --el-box-shadow-dark: 0px 0px 16px rgba(0, 0, 0, 0.16)

use crate::theme::color::ThemeColor;

/// 单个阴影
#[derive(Debug, Clone, Copy)]
pub struct ShadowSpec {
    pub offset_x: f32,
    pub offset_y: f32,
    pub blur: f32,
    pub spread: f32,
    pub color: ThemeColor,
}

impl ShadowSpec {
    /// 转换为 CSS box-shadow 字符串（主要用于调试和测试）
    pub fn to_css(&self) -> String {
        format!(
            "{}px {}px {}px rgba({}, {}, {}, {})",
            self.offset_x,
            self.offset_y,
            self.blur,
            self.color.r,
            self.color.g,
            self.color.b,
            self.color.a
        )
    }
}

/// 阴影系统（4 档）
#[derive(Debug, Clone, Copy)]
pub struct Shadow {
    pub base: ShadowSpec,
    pub light: ShadowSpec,
    pub lighter: ShadowSpec,
    pub dark: ShadowSpec,
}

impl Default for Shadow {
    fn default() -> Self {
        let black = ThemeColor::from_rgba(0, 0, 0, 0.12);
        let black_dark = ThemeColor::from_rgba(0, 0, 0, 0.16);

        Self {
            // --el-box-shadow: 0px 0px 12px rgba(0, 0, 0, 0.12)
            base: ShadowSpec {
                offset_x: 0.0,
                offset_y: 12.0,
                blur: 12.0,
                spread: 0.0,
                color: black,
            },
            // --el-box-shadow-light: 0px 0px 8px rgba(0, 0, 0, 0.12)
            light: ShadowSpec {
                offset_x: 0.0,
                offset_y: 8.0,
                blur: 8.0,
                spread: 0.0,
                color: black,
            },
            // --el-box-shadow-lighter: 0px 0px 6px rgba(0, 0, 0, 0.12)
            lighter: ShadowSpec {
                offset_x: 0.0,
                offset_y: 6.0,
                blur: 6.0,
                spread: 0.0,
                color: black,
            },
            // --el-box-shadow-dark: 0px 0px 16px rgba(0, 0, 0, 0.16)
            dark: ShadowSpec {
                offset_x: 0.0,
                offset_y: 16.0,
                blur: 16.0,
                spread: 0.0,
                color: black_dark,
            },
        }
    }
}
