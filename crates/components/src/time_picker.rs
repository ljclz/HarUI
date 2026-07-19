//! TimePicker 时间选择器 — 参考 Element Plus `<el-time-picker>`。
//!
//! 支持：基础选择、is_range 范围、format、disabled、clearable、placeholder、方向键增减。

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
