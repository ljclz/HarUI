//! Radio 单选框组件 — 参考 Element Plus `<el-radio>` 与 `<el-radio-group>`。
//! 支持：单个 Radio（checked/disabled/size/border）与 RadioGroup（单选/disabled/互斥/clear/toggle）。

use har_ui_core::theme::Theme;
use iced::widget::{button, container, text};
use iced::{Color, Element, Length, Padding};

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
        // 单个 Radio 只处理 SetChecked，不处理 Group 相关消息
        if let (RadioMessage::SetChecked(c), false) = (&msg, self.disabled) {
            self.checked = *c;
        }
    }

    // ---------- 视觉辅助 ----------

    /// 按 size 计算 padding
    fn padding_for_size(size: RadioSize) -> Padding {
        Self::padding_for_size_pub(size)
    }

    /// 按 size 计算 padding（pub 版本，供 RadioGroup 调用）
    pub fn padding_for_size_pub(size: RadioSize) -> Padding {
        match size {
            RadioSize::Large => Padding::from([10u16, 14u16]),
            RadioSize::Default => Padding::from([8u16, 12u16]),
            RadioSize::Small => Padding::from([6u16, 10u16]),
        }
    }

    /// 按 size 计算 font size
    fn font_size_for_size(size: RadioSize) -> f32 {
        Self::font_size_for_size_pub(size)
    }

    /// 按 size 计算 font size（pub 版本，供 RadioGroup 调用）
    pub fn font_size_for_size_pub(size: RadioSize) -> f32 {
        match size {
            RadioSize::Large => 16.0,
            RadioSize::Default => 14.0,
            RadioSize::Small => 12.0,
        }
    }

    /// 渲染单个 Radio 为 iced::Element
    ///
    /// # 参数
    /// - `theme`: HarUI 主题引用
    /// - `on_press`: 点击时发出的消息
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        on_press: Message,
    ) -> Element<'a, Message> {
        let primary = Color::from(theme.primary.base);
        let text_color = if self.disabled {
            Color::from(theme.neutral.text_disabled)
        } else {
            Color::from(theme.neutral.text_regular)
        };
        let dot_color = if self.disabled {
            Color::from(theme.neutral.text_disabled)
        } else if self.checked {
            primary
        } else {
            Color::from(theme.neutral.border_base)
        };

        // 圆点指示器（用 Unicode 字符 ●/○）
        let dot = if self.checked { "●" } else { "○" };
        let size = Self::font_size_for_size(self.size);

        let content = iced::widget::Row::new()
            .push(text(dot).color(dot_color).size(size + 2.0))
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
                        color: if self.checked {
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
            btn = btn.on_press(on_press);
        }

        btn.into()
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

    /// 渲染 RadioGroup 为 iced::Element（横排多个 Radio）
    ///
    /// # 参数
    /// - `theme`: HarUI 主题引用
    /// - `options`: 选项列表，每项为 (value, label)
    /// - `on_change`: 选中某项时发出消息，参数为该项 value
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        options: &'a [(&'a str, &'a str)],
        on_change: impl Fn(String) -> Message + 'a,
    ) -> Element<'a, Message> {
        if options.is_empty() {
            return container(text("")).into();
        }

        let primary = Color::from(theme.primary.base);
        let text_regular = Color::from(theme.neutral.text_regular);
        let text_disabled = Color::from(theme.neutral.text_disabled);
        let border_base = Color::from(theme.neutral.border_base);
        let size = Radio::font_size_for_size_pub(self.size);
        let padding = Radio::padding_for_size_pub(self.size);

        // 预计算所有 Message（on_change 是 Fn，可多次调用）
        let mut radios: Vec<Element<'a, Message>> = Vec::with_capacity(options.len());
        for (value, label) in options {
            let is_checked = self.is_checked(value);
            let dot_color = if self.disabled {
                text_disabled
            } else if is_checked {
                primary
            } else {
                border_base
            };
            let label_color = if self.disabled {
                text_disabled
            } else {
                text_regular
            };
            let dot = if is_checked { "●" } else { "○" };

            let content = iced::widget::Row::new()
                .push(text(dot).color(dot_color).size(size + 2.0))
                .push(iced::widget::Space::with_width(Length::Fixed(6.0)))
                .push(text(label.to_string()).color(label_color).size(size))
                .align_y(iced::Alignment::Center);

            let mut btn = button(content).padding(padding).style(move |_t, _status| {
                iced::widget::button::Style {
                    background: None,
                    text_color: label_color,
                    border: iced::Border::default(),
                    shadow: iced::Shadow::default(),
                }
            });
            if !self.disabled {
                btn = btn.on_press(on_change(value.to_string()));
            }
            radios.push(btn.into());
        }

        iced::widget::Row::with_children(radios)
            .spacing(12)
            .align_y(iced::Alignment::Center)
            .into()
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
