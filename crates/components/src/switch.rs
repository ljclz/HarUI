//! Switch 开关组件 — 参考 Element Plus `<el-switch>`。
//! 支持：开/关切换、disabled、loading、active-color/inactive-color、文本描述、自定义 active/inactive 值。

use har_ui_core::theme::Theme;
use iced::widget::{button, text};
use iced::{Color, Element, Length, Padding};

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

    // ---------- 视觉辅助 ----------

    /// 解析 hex 颜色字符串为 Color，失败回退到 fallback
    fn parse_color(hex: &str, fallback: Color) -> Color {
        let hex = hex.trim_start_matches('#');
        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).ok();
            let g = u8::from_str_radix(&hex[2..4], 16).ok();
            let b = u8::from_str_radix(&hex[4..6], 16).ok();
            if let (Some(r), Some(g), Some(b)) = (r, g, b) {
                return Color::from_rgb8(r, g, b);
            }
        }
        fallback
    }

    /// 渲染 Switch 为 iced::Element
    ///
    /// # 参数
    /// - `theme`: HarUI 主题引用
    /// - `on_toggle`: 点击切换时发出的消息
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        on_toggle: Message,
    ) -> Element<'a, Message> {
        let is_on = self.is_on();
        let primary = Color::from(theme.primary.base);
        let border_base = Color::from(theme.neutral.border_base);
        let text_regular = Color::from(theme.neutral.text_regular);
        let text_disabled = Color::from(theme.neutral.text_disabled);

        let active_color = Self::parse_color(&self.active_color, primary);
        let inactive_color = Self::parse_color(&self.inactive_color, border_base);
        let track_color = if is_on { active_color } else { inactive_color };
        let text_color = if self.disabled {
            text_disabled
        } else {
            text_regular
        };

        // 滑块：ON 状态在右侧（●----），OFF 在左侧（----●）
        // 用 Unicode 字符可视化：开=[●  ] 关=[  ●]
        let slider_visual = if is_on { "[ ● ]" } else { "[   ]" };
        let slider_text = text(slider_visual).color(track_color).size(16);

        // 文本描述（左侧 inactive_text / 右侧 active_text）
        let mut row = iced::widget::Row::new().align_y(iced::Alignment::Center);

        // 左侧文本（仅 inactive 状态显示 inactive_text，或者左侧始终显示 inactive_text）
        if let Some(t) = &self.inactive_text {
            row = row.push(text(t.clone()).color(text_color).size(14));
            row = row.push(iced::widget::Space::with_width(Length::Fixed(6.0)));
        }

        row = row.push(slider_text);

        // 右侧文本
        if let Some(t) = &self.active_text {
            row = row.push(iced::widget::Space::with_width(Length::Fixed(6.0)));
            row = row.push(text(t.clone()).color(text_color).size(14));
        }

        // loading 时显示加载指示
        if self.loading {
            row = row.push(iced::widget::Space::with_width(Length::Fixed(6.0)));
            row = row.push(text("⟳").color(text_color).size(14));
        }

        let mut btn =
            button(row)
                .padding(Padding::from([6u16, 10u16]))
                .style(move |_t, _status| iced::widget::button::Style {
                    background: None,
                    text_color,
                    border: iced::Border::default(),
                    shadow: iced::Shadow::default(),
                });

        if self.can_toggle() {
            btn = btn.on_press(on_toggle);
        }

        btn.into()
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
