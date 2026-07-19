//! DatePicker 日期选择器 — 参考 Element Plus `<el-date-picker>`。
//!
//! 支持：4 种 type（date/datetime/month/year/daterange）、placeholder、disabled、clearable、范围选择。

/// 选择器类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DatePickerType {
    #[default]
    Date,
    DateTime,
    Month,
    Year,
    DateRange,
}

/// 简化日期（复用 calendar 模块的同名类型）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SimpleDate {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl SimpleDate {
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Self { year, month, day }
    }
}

/// DatePicker 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DatePickerMessage {
    Open,
    Close,
    SelectDate(SimpleDate),
    SelectDateTime(SimpleDate, u32, u32, u32),
    SelectRangeStart(SimpleDate),
    SelectRangeEnd(SimpleDate),
    Clear,
}

/// DatePicker 组件
#[derive(Debug, Clone)]
pub struct DatePicker {
    picker_type: DatePickerType,
    value: Option<SimpleDate>,
    time_part: Option<(u32, u32, u32)>,
    range_start: Option<SimpleDate>,
    range_end: Option<SimpleDate>,
    visible: bool,
    disabled: bool,
    clearable: bool,
    placeholder: String,
}

impl Default for DatePicker {
    fn default() -> Self {
        Self::new()
    }
}

impl DatePicker {
    pub fn new() -> Self {
        Self {
            picker_type: DatePickerType::Date,
            value: None,
            time_part: None,
            range_start: None,
            range_end: None,
            visible: false,
            disabled: false,
            clearable: false,
            placeholder: "选择日期".to_string(),
        }
    }

    pub fn with_type(mut self, t: DatePickerType) -> Self {
        self.picker_type = t;
        self
    }

    pub fn with_disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }

    pub fn with_clearable(mut self, v: bool) -> Self {
        self.clearable = v;
        self
    }

    pub fn with_placeholder(mut self, p: impl Into<String>) -> Self {
        self.placeholder = p.into();
        self
    }

    pub fn picker_type(&self) -> DatePickerType {
        self.picker_type
    }

    pub fn value(&self) -> Option<&SimpleDate> {
        self.value.as_ref()
    }

    pub fn range_start(&self) -> Option<&SimpleDate> {
        self.range_start.as_ref()
    }

    pub fn range_end(&self) -> Option<&SimpleDate> {
        self.range_end.as_ref()
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

    pub fn placeholder(&self) -> &str {
        &self.placeholder
    }

    /// 格式化值字符串
    pub fn formatted_value(&self) -> String {
        match self.picker_type {
            DatePickerType::Date | DatePickerType::Month | DatePickerType::Year => {
                match self.value {
                    Some(d) => format!("{:04}-{:02}-{:02}", d.year, d.month, d.day),
                    None => String::new(),
                }
            }
            DatePickerType::DateTime => {
                match (self.value, self.time_part) {
                    (Some(d), Some((h, m, s))) => {
                        format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}", d.year, d.month, d.day, h, m, s)
                    }
                    _ => String::new(),
                }
            }
            DatePickerType::DateRange => {
                match (self.range_start, self.range_end) {
                    (Some(s), Some(e)) => {
                        format!("{:04}-{:02}-{:02} ~ {:04}-{:02}-{:02}",
                            s.year, s.month, s.day, e.year, e.month, e.day)
                    }
                    _ => String::new(),
                }
            }
        }
    }

    pub fn handle(&mut self, msg: DatePickerMessage) {
        if self.disabled {
            // disabled 时只允许 Close
            if matches!(msg, DatePickerMessage::Close) {
                self.visible = false;
            }
            return;
        }
        match msg {
            DatePickerMessage::Open => self.visible = true,
            DatePickerMessage::Close => self.visible = false,
            DatePickerMessage::SelectDate(d) => {
                self.value = Some(d);
                self.visible = false;
            }
            DatePickerMessage::SelectDateTime(d, h, m, s) => {
                self.value = Some(d);
                self.time_part = Some((h, m, s));
                self.visible = false;
            }
            DatePickerMessage::SelectRangeStart(d) => {
                self.range_start = Some(d);
            }
            DatePickerMessage::SelectRangeEnd(d) => {
                self.range_end = Some(d);
                self.visible = false;
            }
            DatePickerMessage::Clear => {
                if self.clearable {
                    self.value = None;
                    self.time_part = None;
                    self.range_start = None;
                    self.range_end = None;
                }
            }
        }
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_date_picker_default_placeholder() {
        let p = DatePicker::new();
        assert_eq!(p.placeholder(), "选择日期");
        assert_eq!(p.picker_type(), DatePickerType::Date);
    }
}
