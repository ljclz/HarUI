//! Rate 评分 — 参考 Element Plus `<el-rate>`。
//!
//! 支持：max、value、disabled、allow_half、increase/decrease、clear、show_text。

/// Rate 消息
#[derive(Debug, Clone, PartialEq)]
pub enum RateMessage {
    /// 设置分值（自动按 allow_half 规范化）
    SetValue(f64),
    /// 增加 1 星
    Increase,
    /// 减少 1 星
    Decrease,
    /// 清零
    Clear,
}

/// Rate 组件
#[derive(Debug, Clone)]
pub struct Rate {
    max: u32,
    value: f64,
    disabled: bool,
    allow_half: bool,
    show_text: bool,
    show_score: bool,
    clearable: bool,
}

impl Default for Rate {
    fn default() -> Self {
        Self::new()
    }
}

impl Rate {
    pub fn new() -> Self {
        Self {
            max: 5,
            value: 0.0,
            disabled: false,
            allow_half: false,
            show_text: false,
            show_score: false,
            clearable: false,
        }
    }

    pub fn with_max(mut self, m: u32) -> Self {
        self.max = m;
        self
    }

    pub fn with_disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }

    pub fn with_allow_half(mut self, v: bool) -> Self {
        self.allow_half = v;
        self
    }

    pub fn with_show_text(mut self, v: bool) -> Self {
        self.show_text = v;
        self
    }

    pub fn with_show_score(mut self, v: bool) -> Self {
        self.show_score = v;
        self
    }

    pub fn with_clearable(mut self, v: bool) -> Self {
        self.clearable = v;
        self
    }

    pub fn max(&self) -> u32 {
        self.max
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn allow_half(&self) -> bool {
        self.allow_half
    }

    pub fn show_text(&self) -> bool {
        self.show_text
    }

    pub fn show_score(&self) -> bool {
        self.show_score
    }

    pub fn clearable(&self) -> bool {
        self.clearable
    }

    /// 规范化分值：钳制到 [0, max]，按 allow_half 规则吸附到 0.5 网格
    fn normalize(&self, raw: f64) -> f64 {
        let clamped = raw.clamp(0.0, self.max as f64);
        if self.allow_half {
            // 吸附到最近的 0.5
            (clamped * 2.0).round() / 2.0
        } else {
            // 不允许半星：截断到整数（floor）
            clamped.floor()
        }
    }

    pub fn handle(&mut self, msg: RateMessage) {
        if self.disabled {
            return;
        }
        match msg {
            RateMessage::SetValue(v) => {
                self.value = self.normalize(v);
            }
            RateMessage::Increase => {
                self.value = self.normalize(self.value + 1.0);
            }
            RateMessage::Decrease => {
                self.value = self.normalize(self.value - 1.0);
            }
            RateMessage::Clear => {
                self.value = 0.0;
            }
        }
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_normalize_no_half() {
        let r = Rate::new();
        assert_eq!(r.normalize(3.4), 3.0);
        assert_eq!(r.normalize(3.6), 3.0);
    }

    #[test]
    fn test_normalize_with_half() {
        let r = Rate::new().with_allow_half(true);
        assert_eq!(r.normalize(3.2), 3.0);
        assert_eq!(r.normalize(3.3), 3.5);
        assert_eq!(r.normalize(3.7), 3.5);
        assert_eq!(r.normalize(3.8), 4.0);
    }

    #[test]
    fn test_normalize_clamp() {
        let r = Rate::new().with_max(5);
        assert_eq!(r.normalize(10.0), 5.0);
        assert_eq!(r.normalize(-3.0), 0.0);
    }
}
