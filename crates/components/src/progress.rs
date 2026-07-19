//! Progress 进度条 — 参考 Element Plus `<el-progress>`。
//!
//! 支持：百分比钳制、3 种 type（line/circle/dashboard）、status（default/success/exception）、stroke_width、color、show_text。

/// 类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProgressType {
    #[default]
    Line,
    Circle,
    Dashboard,
}

/// 状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProgressStatus {
    #[default]
    Default,
    Success,
    Exception,
    Warning,
}

/// Progress 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProgressMessage {
    SetPercentage(i32),
    Increment(i32),
    Reset,
}

/// Progress 组件
#[derive(Debug, Clone)]
pub struct Progress {
    picker_type: ProgressType,
    percentage: i32,
    status: ProgressStatus,
    stroke_width: u32,
    show_text: bool,
    color: Option<String>,
    /// 用户显式设置的状态（不为 None 时覆盖自动推导）
    manual_status: Option<ProgressStatus>,
}

impl Default for Progress {
    fn default() -> Self {
        Self::new()
    }
}

impl Progress {
    pub fn new() -> Self {
        Self {
            picker_type: ProgressType::Line,
            percentage: 0,
            status: ProgressStatus::Default,
            stroke_width: 6,
            show_text: true,
            color: None,
            manual_status: None,
        }
    }

    pub fn with_type(mut self, t: ProgressType) -> Self {
        self.picker_type = t;
        self
    }

    pub fn with_status(mut self, s: ProgressStatus) -> Self {
        self.manual_status = Some(s);
        self.status = s;
        self
    }

    pub fn with_stroke_width(mut self, w: u32) -> Self {
        self.stroke_width = w;
        self
    }

    pub fn with_show_text(mut self, v: bool) -> Self {
        self.show_text = v;
        self
    }

    pub fn with_color(mut self, c: impl Into<String>) -> Self {
        self.color = Some(c.into());
        self
    }

    pub fn picker_type(&self) -> ProgressType {
        self.picker_type
    }

    pub fn percentage(&self) -> i32 {
        self.percentage
    }

    pub fn status(&self) -> ProgressStatus {
        self.status
    }

    pub fn stroke_width(&self) -> u32 {
        self.stroke_width
    }

    pub fn show_text(&self) -> bool {
        self.show_text
    }

    pub fn color(&self) -> Option<&str> {
        self.color.as_deref()
    }

    /// 格式化文本（line 类型）
    pub fn formatted_text(&self) -> String {
        format!("{}%", self.percentage)
    }

    fn recompute_status(&mut self) {
        if self.manual_status.is_some() {
            return;
        }
        self.status = if self.percentage >= 100 {
            ProgressStatus::Success
        } else {
            ProgressStatus::Default
        };
    }

    pub fn handle(&mut self, msg: ProgressMessage) {
        match msg {
            ProgressMessage::SetPercentage(pct) => {
                self.percentage = pct.clamp(0, 100);
                self.recompute_status();
            }
            ProgressMessage::Increment(delta) => {
                self.percentage = (self.percentage + delta).clamp(0, 100);
                self.recompute_status();
            }
            ProgressMessage::Reset => {
                self.percentage = 0;
                self.manual_status = None;
                self.status = ProgressStatus::Default;
            }
        }
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_progress_default_stroke_6() {
        let p = Progress::new();
        assert_eq!(p.stroke_width(), 6);
        assert!(p.show_text());
        assert_eq!(p.picker_type(), ProgressType::Line);
    }
}
