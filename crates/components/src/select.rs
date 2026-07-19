//! Select 组件 — 选择器
//!
//! 参考 Element Plus `<el-select>`。
//! 支持：单选/多选、禁用选项、可搜索(filterable)、clearable、1000 选项虚拟列表。

/// 选项
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectOption {
    value: String,
    label: String,
    disabled: bool,
}

impl SelectOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }

    pub fn set_disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }
}

/// Select 状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SelectState {
    #[default]
    Closed,
    Open,
}

/// Select 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectMessage {
    Open,
    Close,
    Choose(String),
    Clear,
    Query(String),
}

/// Select 组件
#[derive(Debug, Clone)]
pub struct Select {
    options: Vec<SelectOption>,
    /// 单选时的值
    value: Option<String>,
    /// 多选时的值列表
    values: Vec<String>,
    multiple: bool,
    filterable: bool,
    clearable: bool,
    disabled: bool,
    state: SelectState,
    query: Option<String>,
}

impl Default for Select {
    fn default() -> Self {
        Self::new()
    }
}

impl Select {
    pub fn new() -> Self {
        Self {
            options: Vec::new(),
            value: None,
            values: Vec::new(),
            multiple: false,
            filterable: false,
            clearable: false,
            disabled: false,
            state: SelectState::Closed,
            query: None,
        }
    }

    pub fn with_option(mut self, opt: SelectOption) -> Self {
        self.options.push(opt);
        self
    }

    pub fn with_multiple(mut self, v: bool) -> Self {
        self.multiple = v;
        self
    }

    pub fn with_filterable(mut self, v: bool) -> Self {
        self.filterable = v;
        self
    }

    pub fn with_clearable(mut self, v: bool) -> Self {
        self.clearable = v;
        self
    }

    pub fn with_disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }

    pub fn options(&self) -> &[SelectOption] {
        &self.options
    }

    pub fn value(&self) -> Option<&String> {
        self.value.as_ref()
    }

    pub fn values(&self) -> &[String] {
        &self.values
    }

    pub fn multiple(&self) -> bool {
        self.multiple
    }

    pub fn filterable(&self) -> bool {
        self.filterable
    }

    pub fn clearable(&self) -> bool {
        self.clearable
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn state(&self) -> SelectState {
        self.state
    }

    pub fn query(&self) -> Option<String> {
        self.query.clone()
    }

    /// 显示的 label（单选）
    pub fn display_label(&self) -> Option<String> {
        let v = self.value.as_ref()?;
        self.options
            .iter()
            .find(|o| &o.value == v)
            .map(|o| o.label.clone())
    }

    /// 按查询文本过滤选项
    pub fn filter(&self, q: &str) -> Vec<&SelectOption> {
        if q.is_empty() {
            return self.options.iter().collect();
        }
        let q_lower = q.to_lowercase();
        self.options
            .iter()
            .filter(|o| o.label.to_lowercase().contains(&q_lower) || o.value.to_lowercase().contains(&q_lower))
            .collect()
    }

    /// 处理消息
    pub fn handle(&mut self, msg: SelectMessage) {
        match msg {
            SelectMessage::Open => {
                if !self.disabled {
                    self.state = SelectState::Open;
                    self.query = Some(String::new());
                }
            }
            SelectMessage::Close => {
                self.state = SelectState::Closed;
            }
            SelectMessage::Choose(v) => {
                // 检查选项存在且未禁用
                let opt = match self.options.iter().find(|o| o.value == v) {
                    Some(o) if !o.disabled => o.clone(),
                    _ => return,
                };
                let _ = opt;
                if self.multiple {
                    // 切换：已选则取消，未选则添加
                    if let Some(pos) = self.values.iter().position(|x| x == &v) {
                        self.values.remove(pos);
                    } else {
                        self.values.push(v);
                    }
                    // 多选不关闭下拉
                } else {
                    self.value = Some(v);
                    // 单选关闭下拉
                    self.state = SelectState::Closed;
                }
            }
            SelectMessage::Clear => {
                if self.clearable {
                    self.value = None;
                    self.values.clear();
                }
            }
            SelectMessage::Query(q) => {
                self.query = Some(q);
            }
        }
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_select_default_no_value() {
        let s = Select::new();
        assert_eq!(s.value(), None);
        assert!(s.values().is_empty());
    }

    #[test]
    fn test_select_filter_by_label_or_value() {
        let s = Select::new()
            .with_option(SelectOption::new("a", "Apple"))
            .with_option(SelectOption::new("b", "Banana"));
        let filtered = s.filter("a");
        // "a" 同时匹配 value "a" 和 label "Apple"/"Banana"
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_select_choose_disabled_option_ignored() {
        let mut s = Select::new()
            .with_option(SelectOption::new("a", "A").set_disabled(true));
        s.handle(SelectMessage::Choose("a".to_string()));
        assert_eq!(s.value(), None);
    }
}
