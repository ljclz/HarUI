//! Statistic 统计数值 — 参考 Element Plus `<el-statistic>`。
//!
//! 支持：title、value（整数/浮点/字符串）、prefix、suffix、precision、grouping、value_style。

use har_ui_core::theme::Theme;
use iced::widget::{container, text};
use iced::{Color, Element, Length, Padding};

/// Statistic 值
#[derive(Debug, Clone)]
pub enum StatisticValue {
    Int(i64),
    Float(f64),
    Text(String),
}

impl StatisticValue {
    /// 按 precision 格式化为字符串（precision 仅对 Float 生效；grouping 加千分位）
    pub fn format(&self, precision: Option<u32>, grouping: bool) -> String {
        let raw = match self {
            StatisticValue::Int(v) => v.to_string(),
            StatisticValue::Float(v) => match precision {
                Some(p) => format!("{:.*}", p as usize, v),
                None => v.to_string(),
            },
            StatisticValue::Text(s) => s.clone(),
        };
        if grouping {
            apply_grouping(&raw)
        } else {
            raw
        }
    }
}

/// 给数字字符串加千分位（仅对整数部分生效，保留小数和符号）
fn apply_grouping(s: &str) -> String {
    let neg = s.starts_with('-');
    let body = if neg { &s[1..] } else { s };
    let (int_part, frac_part) = match body.find('.') {
        Some(i) => (&body[..i], Some(&body[i..])),
        None => (body, None),
    };
    let grouped = group_thousands(int_part);
    let mut out = String::new();
    if neg {
        out.push('-');
    }
    out.push_str(&grouped);
    if let Some(f) = frac_part {
        out.push_str(f);
    }
    out
}

fn group_thousands(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    if len <= 3 {
        return s.to_string();
    }
    let mut out = String::new();
    let first_group_len = len % 3;
    let mut idx = 0;
    if first_group_len > 0 {
        for &c in &chars[0..first_group_len] {
            out.push(c);
        }
        out.push(',');
        idx = first_group_len;
    }
    while idx < len {
        for &c in &chars[idx..idx + 3] {
            out.push(c);
        }
        idx += 3;
        if idx < len {
            out.push(',');
        }
    }
    out
}

/// Statistic 组件
#[derive(Debug, Clone)]
pub struct Statistic {
    title: Option<String>,
    value: StatisticValue,
    prefix: Option<String>,
    suffix: Option<String>,
    precision: Option<u32>,
    grouping: bool,
    value_color: Option<Color>,
}

impl Default for Statistic {
    fn default() -> Self {
        Self::new(StatisticValue::Int(0))
    }
}

impl Statistic {
    pub fn new(value: StatisticValue) -> Self {
        Self {
            title: None,
            value,
            prefix: None,
            suffix: None,
            precision: None,
            grouping: false,
            value_color: None,
        }
    }

    pub fn with_title(mut self, t: impl Into<String>) -> Self {
        self.title = Some(t.into());
        self
    }

    pub fn with_prefix(mut self, p: impl Into<String>) -> Self {
        self.prefix = Some(p.into());
        self
    }

    pub fn with_suffix(mut self, s: impl Into<String>) -> Self {
        self.suffix = Some(s.into());
        self
    }

    pub fn with_precision(mut self, p: u32) -> Self {
        self.precision = Some(p);
        self
    }

    pub fn with_grouping(mut self, g: bool) -> Self {
        self.grouping = g;
        self
    }

    pub fn with_value_color(mut self, c: Color) -> Self {
        self.value_color = Some(c);
        self
    }

    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    pub fn value(&self) -> &StatisticValue {
        &self.value
    }

    pub fn prefix(&self) -> Option<&str> {
        self.prefix.as_deref()
    }

    pub fn suffix(&self) -> Option<&str> {
        self.suffix.as_deref()
    }

    pub fn precision(&self) -> Option<u32> {
        self.precision
    }

    pub fn grouping(&self) -> bool {
        self.grouping
    }

    pub fn value_color(&self) -> Option<Color> {
        self.value_color
    }

    /// 格式化后的 value 字符串
    pub fn formatted_value(&self) -> String {
        self.value.format(self.precision, self.grouping)
    }

    /// 渲染 Statistic 为 iced::Element
    pub fn view<'a>(&'a self, theme: &'a Theme) -> Element<'a, ()> {
        let title_color = Color::from(theme.neutral.text_secondary);
        let value_color = self.value_color.unwrap_or_else(|| Color::from(theme.neutral.text_primary));
        let affix_color = Color::from(theme.neutral.text_regular);

        let mut col_children: Vec<Element<'a, ()>> = Vec::new();

        // title
        if let Some(t) = &self.title {
            col_children.push(
                text(t.clone())
                    .color(title_color)
                    .size(13.0)
                    .into(),
            );
            col_children.push(iced::widget::Space::with_height(Length::Fixed(4.0)).into());
        }

        // value row: prefix + value + suffix
        let mut value_row_children: Vec<Element<'a, ()>> = Vec::new();
        if let Some(p) = &self.prefix {
            value_row_children.push(
                text(p.clone())
                    .color(affix_color)
                    .size(16.0)
                    .into(),
            );
        }
        value_row_children.push(
            text(self.formatted_value())
                .color(value_color)
                .size(24.0)
                .into(),
        );
        if let Some(s) = &self.suffix {
            value_row_children.push(
                text(s.clone())
                    .color(affix_color)
                    .size(16.0)
                    .into(),
            );
        }

        let value_row = iced::widget::Row::with_children(value_row_children)
            .align_y(iced::alignment::Vertical::Center)
            .spacing(4);
        col_children.push(value_row.into());

        let col = iced::widget::Column::with_children(col_children).spacing(0);

        container(col)
            .width(Length::Fill)
            .padding(Padding::from([8u16, 0u16]))
            .into()
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_statistic_default() {
        let s = Statistic::default();
        assert!(s.title().is_none());
        assert!(s.prefix().is_none());
        assert!(s.suffix().is_none());
        assert!(!s.grouping());
    }

    #[test]
    fn test_statistic_int_no_grouping() {
        let s = Statistic::new(StatisticValue::Int(1234567));
        assert_eq!(s.formatted_value(), "1234567");
    }

    #[test]
    fn test_statistic_int_grouping() {
        let s = Statistic::new(StatisticValue::Int(1234567)).with_grouping(true);
        assert_eq!(s.formatted_value(), "1,234,567");
    }

    #[test]
    fn test_statistic_float_precision() {
        let s = Statistic::new(StatisticValue::Float(3.14159265)).with_precision(2);
        assert_eq!(s.formatted_value(), "3.14");
    }

    #[test]
    fn test_statistic_float_precision_grouping() {
        let s = Statistic::new(StatisticValue::Float(1234567.89))
            .with_precision(2)
            .with_grouping(true);
        assert_eq!(s.formatted_value(), "1,234,567.89");
    }

    #[test]
    fn test_statistic_negative_grouping() {
        let s = Statistic::new(StatisticValue::Int(-1234567)).with_grouping(true);
        assert_eq!(s.formatted_value(), "-1,234,567");
    }

    #[test]
    fn test_statistic_text_value() {
        let s = Statistic::new(StatisticValue::Text("自定义".to_string()));
        assert_eq!(s.formatted_value(), "自定义");
    }
}
