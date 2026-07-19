//! 字体系统 — 与 Element Plus 字号/字重/行高 1:1 映射
//!
//! 来源: element-plus/packages/theme-chalk/src/common/var.scss

/// 字号档位（10 档）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontSize {
    Xs,       // 12px
    Sm,       // 14px (基础字号)
    Md,       // 16px
    Lg,       // 18px
    Xl,       // 20px
    Xxl,      // 24px
    Xxxl,     // 28px
    Xxxxl,    // 32px
    Xxxxxl,   // 36px
    Xxxxxxl,  // 40px
}

/// 字重
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontWeight {
    Normal,   // 400
    Medium,   // 500
    Bold,     // 700
}

/// 行高
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LineHeight {
    Tight,    // 1.2
    Normal,   // 1.5
    Loose,    // 1.8
}

/// 字体系统
#[derive(Debug, Clone)]
pub struct Typography {
    pub font_family: String,
}

impl Default for Typography {
    fn default() -> Self {
        Self {
            // Element Plus 默认字体栈
            // "Helvetica Neue", Helvetica, "PingFang SC", "Hiragino Sans GB",
            // "Microsoft YaHei", "微软雅黑", Arial, sans-serif
            font_family: format!(
                "Helvetica Neue, Helvetica, PingFang SC, \
                 Hiragino Sans GB, Microsoft YaHei, 微软雅黑, \
                 Arial, sans-serif"
            ),
        }
    }
}

impl Typography {
    pub fn font_family(&self) -> &str {
        &self.font_family
    }

    /// 基础字号 14px
    pub fn base_size(&self) -> f32 {
        14.0
    }

    pub fn size(&self, fs: FontSize) -> f32 {
        match fs {
            FontSize::Xs => 12.0,
            FontSize::Sm => 14.0,
            FontSize::Md => 16.0,
            FontSize::Lg => 18.0,
            FontSize::Xl => 20.0,
            FontSize::Xxl => 24.0,
            FontSize::Xxxl => 28.0,
            FontSize::Xxxxl => 32.0,
            FontSize::Xxxxxl => 36.0,
            FontSize::Xxxxxxl => 40.0,
        }
    }

    pub fn weight(&self, w: FontWeight) -> u16 {
        match w {
            FontWeight::Normal => 400,
            FontWeight::Medium => 500,
            FontWeight::Bold => 700,
        }
    }

    pub fn line_height(&self, lh: LineHeight) -> f32 {
        match lh {
            LineHeight::Tight => 1.2,
            LineHeight::Normal => 1.5,
            LineHeight::Loose => 1.8,
        }
    }
}
