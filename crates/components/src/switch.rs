//! Switch 开关组件 — 参考 Element Plus `<el-switch>`。
//! 支持：开/关切换、disabled、loading、active-color/inactive-color、文本描述、自定义 active/inactive 值。

/// Switch 值类型，支持 bool/i32/文本三种自定义值。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SwitchValue {
    Bool(bool),
    Int(i32),
    Text(String),
}

impl From<bool> for SwitchValue {
    fn from(b: bool) -> Self {
        SwitchValue::Bool(b)
    }
}

impl From<i32> for SwitchValue {
    fn from(i: i32) -> Self {
        SwitchValue::Int(i)
    }
}

impl From<&str> for SwitchValue {
    fn from(s: &str) -> Self {
        SwitchValue::Text(s.to_string())
    }
}

/// Switch 消息
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwitchMessage {
    /// 点击切换
    Toggle,
    /// 直接设置 bool 值（仅默认 bool 模式生效）
    SetValue(bool),
}

/// Switch 组件
#[derive(Debug, Clone)]
pub struct Switch {
    /// 当前值
    value: SwitchValue,
    /// 激活值（默认 Bool(true)）
    active_value: SwitchValue,
    /// 未激活值（默认 Bool(false)）
    inactive_value: SwitchValue,
    /// 是否禁用
    disabled: bool,
    /// 是否加载中（加载中不可切换）
    loading: bool,
    /// 宽度（px），默认 40
    width: u32,
    /// 激活时颜色，默认 #409EFF
    active_color: String,
    /// 未激活时颜色，默认 #C0CCDA
    inactive_color: String,
    /// 激活时显示的文本
    active_text: Option<String>,
    /// 未激活时显示的文本
    inactive_text: Option<String>,
}

impl Default for Switch {
    fn default() -> Self {
        Self::new()
    }
}

impl Switch {
    /// 创建默认 Switch（关闭状态，bool 值）
    pub fn new() -> Self {
        Self {
            value: SwitchValue::Bool(false),
            active_value: SwitchValue::Bool(true),
            inactive_value: SwitchValue::Bool(false),
            disabled: false,
            loading: false,
            width: 40,
            active_color: "#409EFF".to_string(),
            inactive_color: "#C0CCDA".to_string(),
            active_text: None,
            inactive_text: None,
        }
    }

    /// 创建指定初始 bool 值的 Switch
    pub fn with_value(initial: bool) -> Self {
        let mut s = Self::new();
        s.value = SwitchValue::Bool(initial);
        s
    }

    // ---------- Builder 方法 ----------

    pub fn with_width(mut self, w: u32) -> Self {
        self.width = w;
        self
    }

    pub fn with_disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }

    pub fn with_loading(mut self, l: bool) -> Self {
        self.loading = l;
        self
    }

    pub fn with_active_color(mut self, c: &str) -> Self {
        self.active_color = c.to_string();
        self
    }

    pub fn with_inactive_color(mut self, c: &str) -> Self {
        self.inactive_color = c.to_string();
        self
    }

    pub fn with_active_text(mut self, t: &str) -> Self {
        self.active_text = Some(t.to_string());
        self
    }

    pub fn with_inactive_text(mut self, t: &str) -> Self {
        self.inactive_text = Some(t.to_string());
        self
    }

    /// 设置自定义激活值（会自动把当前值置为 inactive_value）
    pub fn with_active_value(mut self, v: SwitchValue) -> Self {
        self.active_value = v.clone();
        // 设置 active/inactive 后当前值默认为 inactive
        self.value = self.inactive_value.clone();
        self
    }

    /// 设置自定义未激活值（会自动把当前值置为 inactive_value）
    pub fn with_inactive_value(mut self, v: SwitchValue) -> Self {
        self.inactive_value = v.clone();
        self.value = v;
        self
    }

    // ---------- Getter 方法 ----------

    pub fn is_on(&self) -> bool {
        self.value == self.active_value
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn loading(&self) -> bool {
        self.loading
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn active_color(&self) -> &str {
        &self.active_color
    }

    pub fn inactive_color(&self) -> &str {
        &self.inactive_color
    }

    pub fn active_text(&self) -> Option<&str> {
        self.active_text.as_deref()
    }

    pub fn inactive_text(&self) -> Option<&str> {
        self.inactive_text.as_deref()
    }

    /// 当前应显示的文本（根据开关状态返回 active_text 或 inactive_text）
    pub fn current_text(&self) -> Option<&str> {
        if self.is_on() {
            self.active_text.as_deref()
        } else {
            self.inactive_text.as_deref()
        }
    }

    /// 获取当前值
    pub fn value(&self) -> SwitchValue {
        self.value.clone()
    }

    // ---------- 运行时修改 ----------

    pub fn set_disabled(&mut self, d: bool) {
        self.disabled = d;
    }

    pub fn set_loading(&mut self, l: bool) {
        self.loading = l;
    }

    // ---------- 消息处理 ----------

    /// 是否允许切换（disabled 或 loading 时不可切换）
    fn can_toggle(&self) -> bool {
        !self.disabled && !self.loading
    }

    /// 处理消息
    pub fn handle(&mut self, msg: SwitchMessage) {
        match msg {
            SwitchMessage::Toggle => {
                if self.can_toggle() {
                    // 切换到相反值
                    self.value = if self.value == self.active_value {
                        self.inactive_value.clone()
                    } else {
                        self.active_value.clone()
                    };
                }
            }
            SwitchMessage::SetValue(v) => {
                if self.can_toggle() {
                    self.value = SwitchValue::Bool(v);
                }
            }
        }
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_switch_internal_default_active_inactive() {
        let s = Switch::new();
        assert_eq!(s.active_value, SwitchValue::Bool(true));
        assert_eq!(s.inactive_value, SwitchValue::Bool(false));
    }

    #[test]
    fn test_switch_internal_can_toggle_blocks() {
        let s = Switch::new().with_disabled(true);
        assert!(!s.can_toggle());
        let s = Switch::new().with_loading(true);
        assert!(!s.can_toggle());
        let s = Switch::new();
        assert!(s.can_toggle());
    }
}
