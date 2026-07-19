//! Steps 步骤条 — 参考 Element Plus `<el-steps>`。
//!
//! 支持：方向（horizontal/vertical）、simple 模式、当前步骤切换、finish/reset、状态计算。

/// 方向
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StepsDirection {
    #[default]
    Horizontal,
    Vertical,
}

/// 步骤状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StepStatus {
    #[default]
    Wait,
    Process,
    Finish,
    Error,
    Success,
}

/// 单个步骤
#[derive(Debug, Clone)]
pub struct Step {
    title: String,
    description: Option<String>,
    icon: Option<String>,
}

impl Step {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            icon: None,
        }
    }

    pub fn with_description(mut self, d: impl Into<String>) -> Self {
        self.description = Some(d.into());
        self
    }

    pub fn with_icon(mut self, i: impl Into<String>) -> Self {
        self.icon = Some(i.into());
        self
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn icon(&self) -> Option<&str> {
        self.icon.as_deref()
    }
}

/// Steps 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepsMessage {
    Next,
    Prev,
    JumpTo(usize),
    Finish,
    Reset,
}

/// Steps 组件
#[derive(Debug, Clone)]
pub struct Steps {
    steps: Vec<Step>,
    current: usize,
    direction: StepsDirection,
    simple: bool,
    finished: bool,
}

impl Default for Steps {
    fn default() -> Self {
        Self::new()
    }
}

impl Steps {
    pub fn new() -> Self {
        Self {
            steps: Vec::new(),
            current: 0,
            direction: StepsDirection::Horizontal,
            simple: false,
            finished: false,
        }
    }

    pub fn with_step(mut self, s: Step) -> Self {
        self.steps.push(s);
        self
    }

    pub fn with_direction(mut self, d: StepsDirection) -> Self {
        self.direction = d;
        self
    }

    pub fn with_simple(mut self, v: bool) -> Self {
        self.simple = v;
        self
    }

    pub fn steps(&self) -> &[Step] {
        &self.steps
    }

    pub fn current(&self) -> usize {
        self.current
    }

    pub fn direction(&self) -> StepsDirection {
        self.direction
    }

    pub fn simple(&self) -> bool {
        self.simple
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }

    /// 计算指定索引步骤的状态
    pub fn step_status(&self, idx: usize) -> StepStatus {
        if self.finished {
            return StepStatus::Success;
        }
        if idx < self.current {
            StepStatus::Finish
        } else if idx == self.current {
            StepStatus::Process
        } else {
            StepStatus::Wait
        }
    }

    pub fn handle(&mut self, msg: StepsMessage) {
        let max = if self.steps.is_empty() { 0 } else { self.steps.len() - 1 };
        match msg {
            StepsMessage::Next => {
                if self.current < max {
                    self.current += 1;
                }
            }
            StepsMessage::Prev => {
                if self.current > 0 {
                    self.current -= 1;
                }
            }
            StepsMessage::JumpTo(idx) => {
                self.current = idx.min(max);
            }
            StepsMessage::Finish => {
                self.finished = true;
            }
            StepsMessage::Reset => {
                self.current = 0;
                self.finished = false;
            }
        }
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_steps_default_no_steps_no_current() {
        let s = Steps::new();
        assert_eq!(s.current(), 0);
        assert_eq!(s.direction(), StepsDirection::Horizontal);
        assert!(!s.simple());
        assert!(!s.is_finished());
    }
}
