//! Checkbox 多选框组件 — 参考 Element Plus `<el-checkbox>` 与 `<el-checkbox-group>`。
//! 支持：单个 Checkbox（checked/disabled/indeterminate/size/border/true-value/false-value）
//! 与 CheckboxGroup（多选/min/max/disabled/全选/取消全选/indeterminate 计算属性）。

/// Checkbox 尺寸
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CheckboxSize {
    Large,
    #[default]
    Default,
    Small,
}

/// Checkbox 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckboxMessage {
    /// 单个 Checkbox 点击切换
    Toggle,
    /// 单个 Checkbox 直接设置选中状态
    SetChecked(bool),
    /// CheckboxGroup 切换某个值的选中状态
    ToggleValue(String),
    /// CheckboxGroup 全选（受 max 限制）
    SelectAll(Vec<String>),
    /// CheckboxGroup 清空
    ClearAll,
}

// ---------- 单个 Checkbox ----------

/// 单个 Checkbox 组件
#[derive(Debug, Clone)]
pub struct Checkbox {
    /// 标签文本（同时也是值，对应 CheckboxGroup 中的 key）
    label: String,
    /// 是否选中
    checked: bool,
    /// 是否禁用
    disabled: bool,
    /// 是否半选（视觉上展示为减号）
    indeterminate: bool,
    /// 尺寸
    size: CheckboxSize,
    /// 是否带边框
    border: bool,
    /// 自定义 true 值
    true_value: Option<String>,
    /// 自定义 false 值
    false_value: Option<String>,
}

impl Checkbox {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            checked: false,
            disabled: false,
            indeterminate: false,
            size: CheckboxSize::Default,
            border: false,
            true_value: None,
            false_value: None,
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

    pub fn with_indeterminate(mut self, i: bool) -> Self {
        self.indeterminate = i;
        self
    }

    pub fn with_size(mut self, s: CheckboxSize) -> Self {
        self.size = s;
        self
    }

    pub fn with_border(mut self, b: bool) -> Self {
        self.border = b;
        self
    }

    pub fn with_true_value(mut self, v: &str) -> Self {
        self.true_value = Some(v.to_string());
        self
    }

    pub fn with_false_value(mut self, v: &str) -> Self {
        self.false_value = Some(v.to_string());
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

    pub fn indeterminate(&self) -> bool {
        self.indeterminate
    }

    pub fn size(&self) -> CheckboxSize {
        self.size
    }

    pub fn border(&self) -> bool {
        self.border
    }

    /// 当前值（如果设置了 true-value/false-value 则返回自定义值，否则按 bool 返回 "true"/"false"）
    pub fn value(&self) -> &str {
        if self.checked {
            self.true_value.as_deref().unwrap_or("true")
        } else {
            self.false_value.as_deref().unwrap_or("false")
        }
    }

    // ---------- 运行时修改 ----------

    pub fn set_checked(&mut self, c: bool) {
        if !self.disabled {
            self.checked = c;
        }
    }

    pub fn set_indeterminate(&mut self, i: bool) {
        self.indeterminate = i;
    }

    // ---------- 消息处理 ----------

    pub fn handle(&mut self, msg: CheckboxMessage) {
        match msg {
            CheckboxMessage::Toggle => {
                if !self.disabled {
                    self.checked = !self.checked;
                }
            }
            CheckboxMessage::SetChecked(c) => {
                if !self.disabled {
                    self.checked = c;
                }
            }
            // 单个 Checkbox 不处理 Group 相关消息
            _ => {}
        }
    }
}

// ---------- CheckboxGroup ----------

/// Checkbox 多选组
#[derive(Debug, Clone, Default)]
pub struct CheckboxGroup {
    /// 当前选中的值列表
    value: Vec<String>,
    /// 是否整体禁用
    disabled: bool,
    /// 最少选择数
    min: Option<usize>,
    /// 最多选择数
    max: Option<usize>,
}

impl CheckboxGroup {
    pub fn new() -> Self {
        Self::default()
    }

    // ---------- Builder ----------

    pub fn with_value(mut self, vals: Vec<&str>) -> Self {
        self.value = vals.into_iter().map(String::from).collect();
        self
    }

    pub fn with_disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }

    pub fn with_min(mut self, m: usize) -> Self {
        self.min = Some(m);
        self
    }

    pub fn with_max(mut self, m: usize) -> Self {
        self.max = Some(m);
        self
    }

    // ---------- Getter ----------

    pub fn value(&self) -> &[String] {
        &self.value
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn min(&self) -> Option<usize> {
        self.min
    }

    pub fn max(&self) -> Option<usize> {
        self.max
    }

    /// 是否已达到 max 上限
    pub fn at_max(&self) -> bool {
        self.max.map_or(false, |m| self.value.len() >= m)
    }

    /// 是否已达到 min 下限
    pub fn at_min(&self) -> bool {
        self.min.map_or(false, |m| self.value.len() <= m)
    }

    /// 判断给定选项集合下的"半选"状态
    /// - 全选 → false（非半选）
    /// - 全不选 → false
    /// - 部分选中 → true
    pub fn is_indeterminate(&self, all_options: &[&str]) -> bool {
        if all_options.is_empty() {
            return false;
        }
        let selected_count = all_options
            .iter()
            .filter(|o| self.value.iter().any(|v| v == *o))
            .count();
        selected_count > 0 && selected_count < all_options.len()
    }

    // ---------- 消息处理 ----------

    pub fn handle(&mut self, msg: CheckboxMessage) {
        if self.disabled {
            return;
        }
        match msg {
            CheckboxMessage::ToggleValue(v) => {
                if let Some(idx) = self.value.iter().position(|x| x == &v) {
                    // 已存在 → 移除（除非已达 min 下限）
                    if !self.at_min() {
                        self.value.remove(idx);
                    }
                } else {
                    // 不存在 → 添加（除非已达 max 上限）
                    if !self.at_max() {
                        self.value.push(v);
                    }
                }
            }
            CheckboxMessage::SelectAll(options) => {
                if let Some(m) = self.max {
                    // 受 max 限制只取前 m 个
                    self.value = options.into_iter().take(m).collect();
                } else {
                    self.value = options;
                }
            }
            CheckboxMessage::ClearAll => {
                // 受 min 限制：min=None 或 min=0 才能清空
                if self.min.map_or(true, |m| m == 0) {
                    self.value.clear();
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_checkbox_internal_default_values() {
        let c = Checkbox::new("x");
        assert_eq!(c.label, "x");
        assert!(!c.checked);
        assert_eq!(c.size, CheckboxSize::Default);
        assert!(c.true_value.is_none());
        assert!(c.false_value.is_none());
    }

    #[test]
    fn test_checkbox_internal_disabled_blocks_set() {
        let mut c = Checkbox::new("x").with_disabled(true);
        c.set_checked(true);
        assert!(!c.checked);
    }

    #[test]
    fn test_checkbox_group_internal_at_min_max() {
        let g = CheckboxGroup::new()
            .with_value(vec!["a", "b"])
            .with_min(2)
            .with_max(3);
        assert!(g.at_min());
        assert!(!g.at_max());
    }
}
