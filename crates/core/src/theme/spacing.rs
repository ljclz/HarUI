//! 间距系统 — 4px 网格
//!
//! 与 Element Plus 设计规范一致。
//! 4px 网格是 Element Plus 间距系统的核心。

/// 间距系统（4px 网格）
#[derive(Debug, Clone, Copy)]
pub struct Spacing {
    pub xxs: f32,    // 2px
    pub xs: f32,     // 4px
    pub sm: f32,     // 8px
    pub md: f32,     // 12px
    pub base: f32,   // 16px
    pub lg: f32,     // 20px
    pub xl: f32,     // 24px
    pub xxl: f32,    // 32px
    pub xxxl: f32,   // 40px
}

impl Default for Spacing {
    fn default() -> Self {
        Self {
            xxs: 2.0,
            xs: 4.0,
            sm: 8.0,
            md: 12.0,
            base: 16.0,
            lg: 20.0,
            xl: 24.0,
            xxl: 32.0,
            xxxl: 40.0,
        }
    }
}

impl Spacing {
    /// 按网格档位获取间距值
    ///
    /// at(0) = 0px
    /// at(1) = 4px
    /// at(2) = 8px
    /// at(n) = n * 4px
    pub fn at(&self, n: u32) -> f32 {
        n as f32 * 4.0
    }
}
