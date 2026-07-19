//! Backtop 回到顶部 — 参考 Element Plus `<el-backtop>`。
//!
//! 支持：visibility_height 阈值、scroll_y 跟踪、Click 回顶、right/bottom、smooth。

/// Backtop 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BacktopMessage {
    /// 滚动事件，更新当前 scroll_y
    Scroll(u32),
    /// 点击回到顶部
    Click,
    /// 设置可见阈值
    SetVisibilityHeight(u32),
    /// 设置是否平滑滚动
    SetSmooth(bool),
}

/// Backtop 组件
#[derive(Debug, Clone)]
pub struct Backtop {
    visibility_height: u32,
    right: u32,
    bottom: u32,
    smooth: bool,
    scroll_y: u32,
}

impl Default for Backtop {
    fn default() -> Self {
        Self::new()
    }
}

impl Backtop {
    pub fn new() -> Self {
        Self {
            visibility_height: 200,
            right: 40,
            bottom: 40,
            smooth: false,
            scroll_y: 0,
        }
    }

    pub fn with_visibility_height(mut self, v: u32) -> Self {
        self.visibility_height = v;
        self
    }

    pub fn with_right(mut self, v: u32) -> Self {
        self.right = v;
        self
    }

    pub fn with_bottom(mut self, v: u32) -> Self {
        self.bottom = v;
        self
    }

    pub fn with_smooth(mut self, v: bool) -> Self {
        self.smooth = v;
        self
    }

    pub fn visibility_height(&self) -> u32 {
        self.visibility_height
    }

    pub fn right(&self) -> u32 {
        self.right
    }

    pub fn bottom(&self) -> u32 {
        self.bottom
    }

    pub fn smooth(&self) -> bool {
        self.smooth
    }

    pub fn scroll_y(&self) -> u32 {
        self.scroll_y
    }

    /// 是否可见（滚动距离严格大于阈值时显示）
    pub fn visible(&self) -> bool {
        self.scroll_y > self.visibility_height
    }

    pub fn handle(&mut self, msg: BacktopMessage) {
        match msg {
            BacktopMessage::Scroll(y) => {
                self.scroll_y = y;
            }
            BacktopMessage::Click => {
                self.scroll_y = 0;
            }
            BacktopMessage::SetVisibilityHeight(v) => {
                self.visibility_height = v;
            }
            BacktopMessage::SetSmooth(v) => {
                self.smooth = v;
            }
        }
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_backtop_visibility_threshold_strict() {
        let b = Backtop::new().with_visibility_height(100);
        // 100 不显示，101 显示
        let mut b1 = b.clone();
        b1.handle(BacktopMessage::Scroll(100));
        assert!(!b1.visible());
        let mut b2 = b.clone();
        b2.handle(BacktopMessage::Scroll(101));
        assert!(b2.visible());
    }
}
