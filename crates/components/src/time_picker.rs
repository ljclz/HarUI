//! TimePicker 时间选择器 — 参考 Element Plus `<el-time-picker>`。
//!
//! 支持：基础选择、is_range 范围、format、disabled、clearable、placeholder、方向键增减。

use har_ui_core::theme::Theme;
use iced::widget::{button, container, text};
use iced::{Color, Element, Length, Padding};

/// 时间值
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TimeValue {
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
}

impl TimeValue {
    pub fn new(hour: u32, minute: u32, second: u32) -> Self {
        Self {
            hour: hour.min(23),
            minute: minute.min(59),
            second: second.min(59),
        }
    }
}

/// TimePicker 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimePickerMessage {
    Open,
    Close,
    Select(TimeValue),
    SelectStart(TimeValue),
    SelectEnd(TimeValue),
    Clear,
    IncHour,
    DecHour,
    IncMinute,
    DecMinute,
    IncSecond,
    DecSecond,
}

/// TimePicker 组件
#[derive(Debug, Clone)]
pub struct TimePicker {
    value: Option<TimeValue>,
    start_value: Option<TimeValue>,
    end_value: Option<TimeValue>,
    visible: bool,
    disabled: bool,
    clearable: bool,
    is_range: bool,
    format: String,
    placeholder: String,
}

impl Default for TimePicker {
    fn default() -> Self {
        Self::new()
    }
}

impl TimePicker {
    pub fn new() -> Self {
        Self {
            value: None,
            start_value: None,
            end_value: None,
            visible: false,
            disabled: false,
            clearable: false,
            is_range: false,
            format: "HH:mm:ss".to_string(),
            placeholder: "选择时间".to_string(),
        }
    }

    pub fn with_disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }

    pub fn with_clearable(mut self, v: bool) -> Self {
        self.clearable = v;
        self
    }

    pub fn with_range(mut self, v: bool) -> Self {
        self.is_range = v;
        self
    }

    pub fn with_format(mut self, f: impl Into<String>) -> Self {
        self.format = f.into();
        self
    }

    pub fn with_placeholder(mut self, p: impl Into<String>) -> Self {
        self.placeholder = p.into();
        self
    }

    pub fn value(&self) -> Option<&TimeValue> {
        self.value.as_ref()
    }

    pub fn start_value(&self) -> Option<&TimeValue> {
        self.start_value.as_ref()
    }

    pub fn end_value(&self) -> Option<&TimeValue> {
        self.end_value.as_ref()
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn clearable(&self) -> bool {
        self.clearable
    }

    pub fn is_range(&self) -> bool {
        self.is_range
    }

    pub fn placeholder(&self) -> &str {
        &self.placeholder
    }

    /// 格式化输出
    pub fn formatted_value(&self) -> String {
        let fmt = &self.format;
        match self.value {
            Some(v) => {
                let s = fmt.replace("HH", &format!("{:02}", v.hour))
                    .replace("mm", &format!("{:02}", v.minute))
                    .replace("ss", &format!("{:02}", v.second));
                s
            }
            None => String::new(),
        }
    }

    pub fn handle(&mut self, msg: TimePickerMessage) {
        if self.disabled {
            if matches!(msg, TimePickerMessage::Close) {
                self.visible = false;
            }
            return;
        }
        match msg {
            TimePickerMessage::Open => self.visible = true,
            TimePickerMessage::Close => self.visible = false,
            TimePickerMessage::Select(t) => {
                self.value = Some(t);
                self.visible = false;
            }
            TimePickerMessage::SelectStart(t) => {
                self.start_value = Some(t);
            }
            TimePickerMessage::SelectEnd(t) => {
                self.end_value = Some(t);
                self.visible = false;
            }
            TimePickerMessage::Clear => {
                if self.clearable {
                    self.value = None;
                    self.start_value = None;
                    self.end_value = None;
                }
            }
            TimePickerMessage::IncHour => self.adjust(|t| TimeValue::new((t.hour + 1) % 24, t.minute, t.second)),
            TimePickerMessage::DecHour => self.adjust(|t| TimeValue::new((t.hour + 23) % 24, t.minute, t.second)),
            TimePickerMessage::IncMinute => self.adjust(|t| TimeValue::new(t.hour, (t.minute + 1) % 60, t.second)),
            TimePickerMessage::DecMinute => self.adjust(|t| TimeValue::new(t.hour, (t.minute + 59) % 60, t.second)),
            TimePickerMessage::IncSecond => self.adjust(|t| TimeValue::new(t.hour, t.minute, (t.second + 1) % 60)),
            TimePickerMessage::DecSecond => self.adjust(|t| TimeValue::new(t.hour, t.minute, (t.second + 59) % 60)),
        }
    }

    fn adjust<F: Fn(TimeValue) -> TimeValue>(&mut self, f: F) {
        if let Some(t) = self.value {
            self.value = Some(f(t));
        }
    }

    /// 渲染 TimePicker 为 iced::Element
    ///
    /// # 参数
    /// - `theme`: HarUI 主题引用
    /// - `on_pick`: 用户选择某个时间时发出消息，参数为 "HH:MM:SS" 格式字符串
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        on_pick: impl Fn(String) -> Message + 'a,
    ) -> Element<'a, Message> {
        let primary = Color::from(theme.primary.base);
        let text_primary = Color::from(theme.neutral.text_primary);
        let text_secondary = Color::from(theme.neutral.text_secondary);
        let text_placeholder = Color::from(theme.neutral.text_placeholder);
        let border_lighter = Color::from(theme.neutral.border_lighter);

        let display_str = if self.is_range {
            let start = self
                .start_value
                .map(|t| format!("{:02}:{:02}:{:02}", t.hour, t.minute, t.second))
                .unwrap_or_else(|| "--:--:--".to_string());
            let end = self
                .end_value
                .map(|t| format!("{:02}:{:02}:{:02}", t.hour, t.minute, t.second))
                .unwrap_or_else(|| "--:--:--".to_string());
            format!("{} ~ {}", start, end)
        } else {
            let formatted = self.formatted_value();
            if formatted.is_empty() {
                self.placeholder.clone()
            } else {
                formatted
            }
        };

        let display_color = if self.value.is_none() && !self.is_range {
            text_placeholder
        } else if self.is_range && self.start_value.is_none() {
            text_placeholder
        } else {
            text_primary
        };

        let display_text = text(display_str).color(display_color).size(14);
        let display_area = container(display_text)
            .width(Length::Fill)
            .padding(Padding::from([8u16, 12u16]))
            .style(move |_t| iced::widget::container::Style {
                text_color: None,
                background: None,
                border: iced::Border {
                    color: if self.disabled {
                        border_lighter
                    } else {
                        primary
                    },
                    width: 1.0,
                    radius: iced::border::radius(4.0),
                },
                shadow: iced::Shadow::default(),
            });

        let mut col_children: Vec<Element<'a, Message>> = vec![display_area.into()];

        if self.visible && !self.disabled {
            let presets: Vec<&str> = vec!["08:00:00", "12:00:00", "18:00:00", "23:59:59"];
            let mut row_children: Vec<Element<'a, Message>> = Vec::new();
            for preset in presets {
                let label = preset.to_string();
                let msg = on_pick(label.clone());
                let btn = button(text(label).color(text_secondary).size(12.0))
                    .padding(Padding::from([4u16, 8u16]))
                    .on_press(msg)
                    .style(move |_t, _status| iced::widget::button::Style {
                        background: None,
                        text_color: text_secondary,
                        border: iced::Border {
                            color: border_lighter,
                            width: 1.0,
                            radius: iced::border::radius(4.0),
                        },
                        shadow: iced::Shadow::default(),
                    });
                row_children.push(btn.into());
            }
            let presets_row = iced::widget::Row::with_children(row_children).spacing(4);
            let presets_wrap = container(presets_row)
                .width(Length::Fill)
                .padding(Padding::from([4u16, 0u16]));
            col_children.push(presets_wrap.into());
        }

        iced::widget::Column::with_children(col_children)
            .spacing(4)
            .into()
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_time_picker_default_format() {
        let p = TimePicker::new();
        assert_eq!(p.format, "HH:mm:ss");
    }
}
