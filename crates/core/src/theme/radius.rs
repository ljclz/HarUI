//! 圆角系统
//!
//! 来源: Element Plus --el-border-radius-* 变量

/// 圆角系统
#[derive(Debug, Clone, Copy)]
pub struct Radius {
    pub none: f32,     // 0
    pub sm: f32,       // 2px
    pub base: f32,     // 4px (默认)
    pub md: f32,       // 6px
    pub lg: f32,       // 8px
    pub xl: f32,       // 12px
    pub xxl: f32,      // 16px
    pub round: f32,    // 20px
    pub circle: f32,   // 9999px (圆形)
}

impl Default for Radius {
    fn default() -> Self {
        Self {
            none: 0.0,
            sm: 2.0,
            base: 4.0,
            md: 6.0,
            lg: 8.0,
            xl: 12.0,
            xxl: 16.0,
            round: 20.0,
            circle: 9999.0,
        }
    }
}

impl Radius {
    /// 按数值获取圆角（向最近的档位对齐）
    pub fn at(&self, px: f32) -> f32 {
        match px as i32 {
            0 => 0.0,
            2 => 2.0,
            4 => 4.0,
            6 => 6.0,
            8 => 8.0,
            12 => 12.0,
            16 => 16.0,
            20 => 20.0,
            _ => px,
        }
    }
}
