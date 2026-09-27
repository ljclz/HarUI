//! Result 结果页 — 参考 Element Plus `<el-result>`。
//!
//! 支持：4 种 type（success/warning/info/error）、自定义 icon/title/subtitle、extra slot。

use har_ui_core::theme::Theme;
use iced::widget::{container, text};
use iced::{Color, Element, Length, Padding};

/// 结果类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ResultType {
    Success,
    Warning,
    #[default]
    Info,
    Error,
}

/// Result 组件（避免与 Rust 内置 `Result` 冲突，命名为 ResultPage）
#[derive(Debug, Clone)]
pub struct ResultPage {
    result_type: ResultType,
    title: String,
    sub_title: Option<String>,
    icon_url: Option<String>,
    has_extra: bool,
}

impl Default for ResultPage {
    fn default() -> Self {
        Self::new()
    }
}

impl ResultPage {
    pub fn new() -> Self {
        Self {
            result_type: ResultType::Info,
            title: "提示".to_string(),
            sub_title: None,
            icon_url: None,
            has_extra: false,
        }
    }

    pub fn with_type(mut self, t: ResultType) -> Self {
        self.result_type = t;
        self
    }

    pub fn with_title(mut self, t: impl Into<String>) -> Self {
        self.title = t.into();
        self
    }

    pub fn with_sub_title(mut self, s: impl Into<String>) -> Self {
        self.sub_title = Some(s.into());
        self
    }

    pub fn with_icon_url(mut self, url: impl Into<String>) -> Self {
        self.icon_url = Some(url.into());
        self
    }

    pub fn with_has_extra(mut self, v: bool) -> Self {
        self.has_extra = v;
        self
    }

    pub fn result_type(&self) -> ResultType {
        self.result_type
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn sub_title(&self) -> Option<&str> {
        self.sub_title.as_deref()
    }

    pub fn icon_url(&self) -> Option<&str> {
        self.icon_url.as_deref()
    }

    pub fn has_extra(&self) -> bool {
        self.has_extra
    }

    /// 按 result_type 返回 (emoji, 颜色)
    fn type_visual(t: ResultType, theme: &Theme) -> (&'static str, Color) {
        match t {
            ResultType::Success => ("✅", Color::from(theme.success.base)),
            ResultType::Warning => ("⚠️", Color::from(theme.warning.base)),
            ResultType::Info => ("ℹ️", Color::from(theme.info.base)),
            ResultType::Error => ("❌", Color::from(theme.danger.base)),
        }
    }

    /// 渲染 ResultPage 为 iced::Element
    pub fn view<'a>(&'a self, theme: &'a Theme) -> Element<'a, ()> {
        let (default_emoji, _accent) = Self::type_visual(self.result_type, theme);

        // icon: 自定义 URL 优先（iced 0.13 无内置 image，使用占位 emoji）
        let icon_str = if self.icon_url.is_some() {
            "🖼"
        } else {
            default_emoji
        };
        let icon_text = text(icon_str).size(64.0);

        let title_text = text(self.title.clone())
            .color(Color::from(theme.neutral.text_primary))
            .size(22.0);

        let mut col = iced::widget::Column::new()
            .push(icon_text)
            .push(iced::widget::Space::with_height(Length::Fixed(12.0)))
            .push(title_text)
            .spacing(0)
            .align_x(iced::alignment::Horizontal::Center);

        if let Some(sub) = &self.sub_title {
            col = col.push(iced::widget::Space::with_height(Length::Fixed(8.0)));
            col = col.push(
                text(sub.clone())
                    .color(Color::from(theme.neutral.text_secondary))
                    .size(14.0),
            );
        }

        if self.has_extra {
            col = col.push(iced::widget::Space::with_height(Length::Fixed(16.0)));
            col = col.push(
                text("[extra slot]")
                    .color(Color::from(theme.neutral.text_placeholder))
                    .size(12.0),
            );
        }

        container(col)
            .width(Length::Fill)
            .padding(Padding::from([32u16, 24u16]))
            .align_x(iced::alignment::Horizontal::Center)
            .into()
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_result_default_type_info() {
        let r = ResultPage::new();
        assert_eq!(r.result_type(), ResultType::Info);
    }
}
