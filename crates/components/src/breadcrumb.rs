//! Breadcrumb 面包屑 — 参考 Element Plus `<el-breadcrumb>`。
//!
//! 支持：分隔符、to 跳转、点击事件、icon、replace 末项、清除点击态。

use har_ui_core::theme::Theme;
use iced::widget::{button, container, text};
use iced::{Color, Element, Length, Padding};

/// 单个面包屑项
#[derive(Debug, Clone)]
pub struct BreadcrumbItem {
    text: String,
    to: Option<String>,
    icon: Option<String>,
}

impl BreadcrumbItem {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            to: None,
            icon: None,
        }
    }

    pub fn with_to(mut self, to: impl Into<String>) -> Self {
        self.to = Some(to.into());
        self
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn to(&self) -> Option<&str> {
        self.to.as_deref()
    }

    pub fn icon(&self) -> Option<&str> {
        self.icon.as_deref()
    }
}

/// Breadcrumb 消息
#[derive(Debug, Clone)]
pub enum BreadcrumbMessage {
    /// 点击指定索引的项
    Click(usize),
    /// 替换最后一个面包屑项
    Replace(BreadcrumbItem),
    /// 清除点击状态
    ClearClick,
}

/// Breadcrumb 组件
#[derive(Debug, Clone)]
pub struct Breadcrumb {
    items: Vec<BreadcrumbItem>,
    separator: String,
    last_clicked: Option<usize>,
    navigate_target: Option<String>,
}

impl Default for Breadcrumb {
    fn default() -> Self {
        Self::new()
    }
}

impl Breadcrumb {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            separator: "/".to_string(),
            last_clicked: None,
            navigate_target: None,
        }
    }

    pub fn with_separator(mut self, s: impl Into<String>) -> Self {
        self.separator = s.into();
        self
    }

    pub fn with_item(mut self, item: BreadcrumbItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn items(&self) -> &[BreadcrumbItem] {
        &self.items
    }

    pub fn separator(&self) -> &str {
        &self.separator
    }

    pub fn last_clicked(&self) -> Option<usize> {
        self.last_clicked
    }

    pub fn navigate_target(&self) -> Option<&str> {
        self.navigate_target.as_deref()
    }

    pub fn handle(&mut self, msg: BreadcrumbMessage) {
        match msg {
            BreadcrumbMessage::Click(idx) => {
                self.last_clicked = Some(idx);
                if let Some(item) = self.items.get(idx) {
                    self.navigate_target = item.to.clone();
                } else {
                    self.navigate_target = None;
                }
            }
            BreadcrumbMessage::Replace(item) => {
                if let Some(last) = self.items.last_mut() {
                    *last = item;
                } else {
                    self.items.push(item);
                }
            }
            BreadcrumbMessage::ClearClick => {
                self.last_clicked = None;
                self.navigate_target = None;
            }
        }
    }

    /// 渲染 Breadcrumb 为 iced::Element
    ///
    /// # 参数
    /// - `theme`: HarUI 主题引用
    /// - `on_click`: 点击某项时发出消息，参数为该项的 to（若无 to 则传 text）
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        on_click: impl Fn(String) -> Message + 'a,
    ) -> Element<'a, Message> {
        if self.items.is_empty() {
            return container(text("")).width(Length::Fill).into();
        }

        let text_primary = Color::from(theme.neutral.text_primary);
        let text_regular = Color::from(theme.neutral.text_regular);
        let text_secondary = Color::from(theme.neutral.text_secondary);
        let primary = Color::from(theme.primary.base);
        let separator_color = Color::from(theme.neutral.text_placeholder);

        let last_idx = self.items.len().saturating_sub(1);
        let mut children: Vec<Element<'a, Message>> = Vec::new();

        for (idx, item) in self.items.iter().enumerate() {
            let is_last = idx == last_idx;
            let has_to = item.to().is_some();

            // icon 前缀
            if let Some(icon) = item.icon() {
                children.push(
                    text(icon.to_string())
                        .color(text_secondary)
                        .size(14.0)
                        .into(),
                );
                children.push(iced::widget::Space::new().width(Length::Fixed(4.0)).into());
            }

            let label_color = if is_last {
                text_primary
            } else if has_to {
                primary
            } else {
                text_regular
            };

            let label_text = text(item.text().to_string()).color(label_color).size(14.0);

            if has_to && !is_last {
                if let Some(to) = item.to() {
                    let btn = button(label_text)
                        .padding(Padding::from(0u16))
                        .style(move |_t, _status| iced::widget::button::Style {
                            background: None,
                            text_color: label_color,
                            border: iced::Border::default(),
                            shadow: iced::Shadow::default(),
                            snap: false,
                        })
                        .on_press(on_click(to.to_string()));
                    children.push(btn.into());
                }
            } else {
                let label_wrap =
                    container(label_text)
                        .padding(Padding::from(0u16))
                        .style(move |_t| iced::widget::container::Style {
                            text_color: Some(label_color),
                            background: None,
                            border: iced::Border::default(),
                            shadow: iced::Shadow::default(),
                            snap: false,
                        });
                children.push(label_wrap.into());
            }

            // 分隔符（非最后一项）
            if !is_last {
                children.push(iced::widget::Space::new().width(Length::Fixed(6.0)).into());
                children.push(
                    text(self.separator.clone())
                        .color(separator_color)
                        .size(14.0)
                        .into(),
                );
                children.push(iced::widget::Space::new().width(Length::Fixed(6.0)).into());
            }
        }

        iced::widget::Row::with_children(children)
            .spacing(0)
            .align_y(iced::Alignment::Center)
            .into()
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_breadcrumb_default_separator_slash() {
        let b = Breadcrumb::new();
        assert_eq!(b.separator(), "/");
        assert!(b.items().is_empty());
    }
}
