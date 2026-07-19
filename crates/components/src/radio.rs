//! Radio 单选框组件 — 参考 Element Plus `<el-radio>` 与 `<el-radio-group>`。
//! 支持：单个 Radio（checked/disabled/size/border）与 RadioGroup（单选/disabled/互斥/clear/toggle）。

/// Radio 尺寸
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RadioSize {
    Large,
    #[default]
    Default,
    Small,
}

/// Radio 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RadioMessage {
    /// 单个 Radio 设置选中状态
    SetChecked(bool),
    /// RadioGroup 设置选中值（互斥：替换原值）
    SetValue(String),
    /// RadioGroup 清空选中值
    Clear,
    /// RadioGroup 便捷切换：点击同一已选项时清空，否则设为新值
    ToggleOption(String),
}

// ---------- 单个 Radio ----------

/// 单个 Radio 组件
#[derive(Debug, Clone)]
pub struct Radio {
    /// 标签文本（同时作为值）
    label: String,
    /// 是否选中
    checked: bool,
    /// 是否禁用
    disabled: bool,
    /// 尺寸
    size: RadioSize,
    /// 是否带边框
    border: bool,
}

impl Radio {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            checked: false,
            disabled: false,
            size: RadioSize::Default,
            border: false,
        }
    }

    // ---------- Builder ----------

    pub fn with_checked(mut self, c: bool) -> Self {
        self.checked = c;
        self
    }

    pub fn with_disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }

    pub fn with_size(mut self, s: RadioSize) -> Self {
        self.size = s;
        self
    }

    pub fn with_border(mut self, b: bool) -> Self {
        self.border = b;
        self
    }

    // ---------- Getter ----------

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn checked(&self) -> bool {
        self.checked
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn size(&self) -> RadioSize {
        self.size
    }

    pub fn border(&self) -> bool {
        self.border
    }

    // ---------- 消息处理 ----------

    pub fn handle(&mut self, msg: RadioMessage) {
        match msg {
            RadioMessage::SetChecked(c) => {
                if !self.disabled {
                    self.checked = c;
                }
            }
            // 单个 Radio 不处理 Group 相关消息
            _ => {}
        }
    }
}

// ---------- RadioGroup ----------

/// Radio 单选组（互斥选择）
#[derive(Debug, Clone, Default)]
pub struct RadioGroup {
    /// 当前选中的值（None 表示未选）
    value: Option<String>,
    /// 是否整体禁用
    disabled: bool,
    /// 尺寸
    size: RadioSize,
}

impl RadioGroup {
    pub fn new() -> Self {
        Self::default()
    }

    // ---------- Builder ----------

    pub fn with_value(mut self, v: &str) -> Self {
        self.value = Some(v.to_string());
        self
    }

    pub fn with_disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }

    pub fn with_size(mut self, s: RadioSize) -> Self {
        self.size = s;
        self
    }

    // ---------- Getter ----------

    pub fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn size(&self) -> RadioSize {
        self.size
    }

    /// 判断给定选项是否被选中
    pub fn is_checked(&self, option: &str) -> bool {
        self.value.as_deref() == Some(option)
    }

    // ---------- 消息处理 ----------

    pub fn handle(&mut self, msg: RadioMessage) {
        if self.disabled {
            return;
        }
        match msg {
            RadioMessage::SetValue(v) => {
                self.value = Some(v);
            }
            RadioMessage::Clear => {
                self.value = None;
            }
            RadioMessage::ToggleOption(v) => {
                if self.value.as_deref() == Some(v.as_str()) {
                    // 已选中相同值 → 清空
                    self.value = None;
                } else {
                    self.value = Some(v);
                }
            }
            // Group 不处理单个 Radio 的消息
            _ => {}
        }
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_radio_internal_default() {
        let r = Radio::new("x");
        assert_eq!(r.label, "x");
        assert!(!r.checked);
        assert_eq!(r.size, RadioSize::Default);
    }

    #[test]
    fn test_radio_internal_disabled_blocks_set() {
        let mut r = Radio::new("x").with_disabled(true);
        r.handle(RadioMessage::SetChecked(true));
        assert!(!r.checked);
    }

    #[test]
    fn test_radio_group_internal_toggle_same_clears() {
        let mut g = RadioGroup::new().with_value("a");
        g.handle(RadioMessage::ToggleOption("a".to_string()));
        assert!(g.value.is_none());
        g.handle(RadioMessage::ToggleOption("b".to_string()));
        assert_eq!(g.value.as_deref(), Some("b"));
    }
}
