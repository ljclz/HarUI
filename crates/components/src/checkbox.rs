//! Checkbox 多选框组件 — 参考 Element Plus `<el-checkbox>` 与 `<el-checkbox-group>`。
//! 支持：单个 Checkbox（checked/disabled/indeterminate/size/border/true-value/false-value）
//! 与 CheckboxGroup（多选/min/max/disabled/全选/取消全选/indeterminate 计算属性）。

use har_ui_core::theme::Theme;
use iced::widget::{button, container, text};
use iced::{Color, Element, Length, Padding};

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

    // ---------- 视觉辅助 ----------

    /// 按 size 计算 padding
    pub fn padding_for_size(size: CheckboxSize) -> Padding {
        match size {
            CheckboxSize::Large => Padding::from([10u16, 14u16]),
            CheckboxSize::Default => Padding::from([8u16, 12u16]),
            CheckboxSize::Small => Padding::from([6u16, 10u16]),
        }
    }

    /// 按 size 计算 font size
    pub fn font_size_for_size(size: CheckboxSize) -> f32 {
        match size {
            CheckboxSize::Large => 16.0,
            CheckboxSize::Default => 14.0,
            CheckboxSize::Small => 12.0,
        }
    }

    /// 渲染单个 Checkbox 为 iced::Element
    ///
    /// # 参数
    /// - `theme`: HarUI 主题引用
    /// - `on_toggle`: 点击切换时发出的消息
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        on_toggle: Message,
    ) -> Element<'a, Message> {
        let primary = Color::from(theme.primary.base);
        let text_color = if self.disabled {
            Color::from(theme.neutral.text_disabled)
        } else {
            Color::from(theme.neutral.text_regular)
        };
        let box_color = if self.disabled {
            Color::from(theme.neutral.text_disabled)
        } else if self.checked || self.indeterminate {
            primary
        } else {
            Color::from(theme.neutral.border_base)
        };

        // 框指示器：☐ 未选 / ☑ 已选 / ▣ 半选（用 - 减号）
        let indicator = if self.indeterminate {
            "▣"
        } else if self.checked {
            "☑"
        } else {
            "☐"
        };
        let size = Self::font_size_for_size(self.size);

        let content = iced::widget::Row::new()
            .push(text(indicator).color(box_color).size(size + 2.0))
            .push(iced::widget::Space::with_width(Length::Fixed(6.0)))
            .push(text(self.label.clone()).color(text_color).size(size))
            .align_y(iced::Alignment::Center);

        let mut btn = button(content)
            .padding(Self::padding_for_size(self.size))
            .style(move |_t, status| {
                let bg = if self.border {
                    match status {
                        iced::widget::button::Status::Hovered
                        | iced::widget::button::Status::Pressed => {
                            Some(iced::Background::Color(Color {
                                a: 0.05,
                                ..Color::from(theme.primary.base)
                            }))
                        }
                        _ => Some(iced::Background::Color(Color::from(
                            theme.neutral.bg_overlay,
                        ))),
                    }
                } else {
                    None
                };
                let border = if self.border {
                    iced::Border {
                        color: if self.checked || self.indeterminate {
                            primary
                        } else {
                            Color::from(theme.neutral.border_base)
                        },
                        width: 1.0,
                        radius: iced::border::radius(4.0),
                    }
                } else {
                    iced::Border::default()
                };
                iced::widget::button::Style {
                    background: bg,
                    text_color,
                    border,
                    shadow: iced::Shadow::default(),
                }
            });

        if !self.disabled {
            btn = btn.on_press(on_toggle);
        }

        btn.into()
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

    /// 渲染 CheckboxGroup 为 iced::Element（横排多个 Checkbox）
    ///
    /// # 参数
    /// - `theme`: HarUI 主题引用
    /// - `options`: 选项列表，每项为 (value, label)
    /// - `on_toggle`: 切换某项时发出消息，参数为该项 value
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        options: &'a [(&'a str, &'a str)],
        on_toggle: impl Fn(String) -> Message + 'a,
    ) -> Element<'a, Message> {
        if options.is_empty() {
            return container(text("")).into();
        }

        let primary = Color::from(theme.primary.base);
        let text_regular = Color::from(theme.neutral.text_regular);
        let text_disabled = Color::from(theme.neutral.text_disabled);
        let border_base = Color::from(theme.neutral.border_base);
        let size = Checkbox::font_size_for_size(CheckboxSize::Default);
        let padding = Checkbox::padding_for_size(CheckboxSize::Default);

        // 预计算所有 Message（on_toggle 是 Fn，可多次调用）
        let mut boxes: Vec<Element<'a, Message>> = Vec::with_capacity(options.len());
        for (value, label) in options {
            let is_checked = self.value.iter().any(|v| v == value);
            // max 上限且未选中：禁用此选项
            let option_disabled = self.disabled || (self.at_max() && !is_checked);
            let box_color = if option_disabled {
                text_disabled
            } else if is_checked {
                primary
            } else {
                border_base
            };
            let label_color = if option_disabled { text_disabled } else { text_regular };
            let indicator = if is_checked { "☑" } else { "☐" };

            let content = iced::widget::Row::new()
                .push(text(indicator).color(box_color).size(size + 2.0))
                .push(iced::widget::Space::with_width(Length::Fixed(6.0)))
                .push(text(label.to_string()).color(label_color).size(size))
                .align_y(iced::Alignment::Center);

            let mut btn = button(content)
                .padding(padding)
                .style(move |_t, _status| iced::widget::button::Style {
                    background: None,
                    text_color: label_color,
                    border: iced::Border::default(),
                    shadow: iced::Shadow::default(),
                });
            if !option_disabled {
                btn = btn.on_press(on_toggle(value.to_string()));
            }
            boxes.push(btn.into());
        }

        iced::widget::Row::with_children(boxes)
            .spacing(12)
            .align_y(iced::Alignment::Center)
            .into()
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
