//! Divider 分割线 — 参考 Element Plus `<el-divider>`。
//!
//! 支持：方向（horizontal/vertical）、内容位置（left/center/right）、文本、虚线样式。

/// 方向
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DividerDirection {
    #[default]
    Horizontal,
    Vertical,
}

/// 内容位置
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DividerContentPosition {
    Left,
    #[default]
    Center,
    Right,
}

/// Divider 组件
#[derive(Debug, Clone)]
pub struct Divider {
    direction: DividerDirection,
    content_position: DividerContentPosition,
    text: Option<String>,
    border_dashed: bool,
}

impl Default for Divider {
    fn default() -> Self {
        Self::new()
    }
}

impl Divider {
    pub fn new() -> Self {
        Self {
            direction: DividerDirection::Horizontal,
            content_position: DividerContentPosition::Center,
            text: None,
            border_dashed: false,
        }
    }

    pub fn with_direction(mut self, d: DividerDirection) -> Self {
        self.direction = d;
        self
    }

    pub fn with_content_position(mut self, p: DividerContentPosition) -> Self {
        self.content_position = p;
        self
    }

    pub fn with_text(mut self, t: impl Into<String>) -> Self {
        self.text = Some(t.into());
        self
    }

    pub fn with_border_dashed(mut self, v: bool) -> Self {
        self.border_dashed = v;
        self
    }

    pub fn direction(&self) -> DividerDirection {
        self.direction
    }

    pub fn content_position(&self) -> DividerContentPosition {
        self.content_position
    }

    pub fn text(&self) -> Option<&str> {
        self.text.as_deref()
    }

    pub fn border_dashed(&self) -> bool {
        self.border_dashed
    }

    pub fn clear_text(&mut self) {
        self.text = None;
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_divider_default_no_text_no_dashed() {
        let d = Divider::new();
        assert!(d.text().is_none());
        assert!(!d.border_dashed());
        assert_eq!(d.direction(), DividerDirection::Horizontal);
    }
}
