//! PageHeader 页头 — 参考 Element Plus `<el-page-header>`。
//!
//! 支持：标题、副标题、内容、icon、返回事件、额外 slot。

use har_ui_core::theme::Theme;
use iced::widget::{button, container, text};
use iced::{Color, Element, Length, Padding};

/// PageHeader 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PageHeaderMessage {
    Back,
    ResetBack,
}

/// PageHeader 组件
#[derive(Debug, Clone)]
pub struct PageHeader {
    title: String,
    subtitle: Option<String>,
    content: Option<String>,
    icon: String,
    has_extra: bool,
    back_clicked: bool,
}

impl Default for PageHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl PageHeader {
    pub fn new() -> Self {
        Self {
            title: "返回".to_string(),
            subtitle: None,
            content: None,
            icon: "arrow-left".to_string(),
            has_extra: false,
            back_clicked: false,
        }
    }

    pub fn with_title(mut self, t: impl Into<String>) -> Self {
        self.title = t.into();
        self
    }

    pub fn with_subtitle(mut self, s: impl Into<String>) -> Self {
        self.subtitle = Some(s.into());
        self
    }

    pub fn with_content(mut self, c: impl Into<String>) -> Self {
        self.content = Some(c.into());
        self
    }

    pub fn with_icon(mut self, i: impl Into<String>) -> Self {
        self.icon = i.into();
        self
    }

    pub fn with_has_extra(mut self, v: bool) -> Self {
        self.has_extra = v;
        self
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn subtitle(&self) -> Option<&str> {
        self.subtitle.as_deref()
    }

    pub fn content(&self) -> Option<&str> {
        self.content.as_deref()
    }

    pub fn icon(&self) -> &str {
        &self.icon
    }

    pub fn has_extra(&self) -> bool {
        self.has_extra
    }

    pub fn back_clicked(&self) -> bool {
        self.back_clicked
    }

    pub fn handle(&mut self, msg: PageHeaderMessage) {
        match msg {
            PageHeaderMessage::Back => self.back_clicked = true,
            PageHeaderMessage::ResetBack => self.back_clicked = false,
        }
    }

    /// 渲染 PageHeader 为 iced::Element
    ///
    /// - 左侧：返回图标 + 标题 + 副标题
    /// - 下方：content
    /// - 右侧：extra slot 占位（has_extra=true 时显示）
    /// - on_back：点击返回图标触发
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        on_back: impl Fn() -> Message + 'a,
    ) -> Element<'a, Message> {
        let title_color = Color::from(theme.neutral.text_primary);
        let subtitle_color = Color::from(theme.neutral.text_secondary);
        let content_color = Color::from(theme.neutral.text_regular);
        let placeholder_color = Color::from(theme.neutral.text_placeholder);
        let border_lighter = Color::from(theme.neutral.border_lighter);

        // 返回按钮（使用 icon 字符串作为图标，默认 "arrow-left" → 用 "←" 渲染）
        let icon_str: &str = if self.icon == "arrow-left" {
            "←"
        } else {
            self.icon.as_str()
        };
        // 取首个字符作为图标（处理多字节字符串）
        let icon_char = icon_str.chars().next().unwrap_or('←');
        let back_btn = button(text(icon_char.to_string()).color(title_color).size(18.0))
            .padding(Padding::from([4u16, 8u16]))
            .on_press(on_back())
            .style(move |_t, _status| iced::widget::button::Style {
                background: None,
                text_color: title_color,
                border: iced::Border::default(),
                shadow: iced::Shadow::default(),
            });

        // 标题 + 副标题
        let title_text = text(self.title.clone()).color(title_color).size(18.0);
        let mut title_children: Vec<Element<'a, Message>> = Vec::new();
        title_children.push(back_btn.into());
        title_children.push(iced::widget::Space::with_width(Length::Fixed(8.0)).into());

        let mut title_text_col_children: Vec<Element<'a, Message>> = Vec::new();
        title_text_col_children.push(title_text.into());
        if let Some(sub) = &self.subtitle {
            title_text_col_children.push(text(sub.clone()).color(subtitle_color).size(12.0).into());
        }
        let title_text_col =
            iced::widget::Column::with_children(title_text_col_children).spacing(2);
        title_children.push(title_text_col.into());

        // 顶部行：左 (返回+标题) + 右 (extra)
        let mut top_row_children: Vec<Element<'a, Message>> = title_children;
        top_row_children.push(iced::widget::Space::with_width(Length::Fill).into());
        if self.has_extra {
            top_row_children.push(text("[extra]").color(placeholder_color).size(12.0).into());
        }
        let top_row = iced::widget::Row::with_children(top_row_children)
            .align_y(iced::Alignment::Center)
            .spacing(0);

        // 整体列：top_row + 可选 content
        let mut col_children: Vec<Element<'a, Message>> = Vec::new();
        col_children.push(top_row.into());
        if let Some(c) = &self.content {
            col_children.push(iced::widget::Space::with_height(Length::Fixed(8.0)).into());
            col_children.push(text(c.clone()).color(content_color).size(14.0).into());
        }
        let col = iced::widget::Column::with_children(col_children).spacing(0);

        container(col)
            .width(Length::Fill)
            .padding(Padding::from([12u16, 16u16]))
            .style(move |_t| iced::widget::container::Style {
                text_color: None,
                background: None,
                border: iced::Border {
                    color: border_lighter,
                    width: 0.0,
                    radius: iced::border::radius(0.0),
                },
                shadow: iced::Shadow::default(),
            })
            .into()
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_page_header_default_icon_arrow_left() {
        let p = PageHeader::new();
        assert_eq!(p.icon(), "arrow-left");
        assert_eq!(p.title(), "返回");
    }
}
