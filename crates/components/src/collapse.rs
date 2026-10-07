//! Collapse 折叠面板 — 参考 Element Plus `<el-collapse>`。
//!
//! 支持：accordion 手风琴、active_keys、disabled item、Toggle/Open/Close/OpenAll/CloseAll。

use har_ui_core::theme::Theme;
use iced::widget::{button, container, text};
use iced::{Color, Element, Length, Padding};
use std::collections::HashSet;

/// Collapse 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CollapseMessage {
    /// 切换某项展开/折叠
    Toggle(String),
    /// 打开某项
    Open(String),
    /// 关闭某项
    Close(String),
    /// 全部打开（手风琴模式下被忽略）
    OpenAll,
    /// 全部关闭
    CloseAll,
}

/// Collapse 项
#[derive(Debug, Clone)]
pub struct CollapseItem {
    name: String,
    title: String,
    disabled: bool,
    default_active: bool,
}

impl CollapseItem {
    pub fn new(name: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            title: title.into(),
            disabled: false,
            default_active: false,
        }
    }

    pub fn with_disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }

    pub fn with_default_active(mut self, v: bool) -> Self {
        self.default_active = v;
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn default_active(&self) -> bool {
        self.default_active
    }
}

/// Collapse 组件
#[derive(Debug, Clone)]
pub struct Collapse {
    accordion: bool,
    items: Vec<CollapseItem>,
    active_keys: HashSet<String>,
}

impl Default for Collapse {
    fn default() -> Self {
        Self::new()
    }
}

impl Collapse {
    pub fn new() -> Self {
        Self {
            accordion: false,
            items: Vec::new(),
            active_keys: HashSet::new(),
        }
    }

    pub fn with_accordion(mut self, v: bool) -> Self {
        self.accordion = v;
        self
    }

    pub fn accordion(&self) -> bool {
        self.accordion
    }

    pub fn items(&self) -> &[CollapseItem] {
        &self.items
    }

    pub fn active_keys(&self) -> &HashSet<String> {
        &self.active_keys
    }

    pub fn is_active(&self, name: &str) -> bool {
        self.active_keys.contains(name)
    }

    pub fn add_item(&mut self, item: CollapseItem) {
        // 应用默认 active
        if item.default_active && !item.disabled {
            if self.accordion {
                // 手风琴模式：先清空，再激活
                self.active_keys.clear();
            }
            self.active_keys.insert(item.name.clone());
        }
        self.items.push(item);
    }

    fn find_item(&self, name: &str) -> Option<&CollapseItem> {
        self.items.iter().find(|i| i.name == name)
    }

    pub fn handle(&mut self, msg: CollapseMessage) {
        match msg {
            CollapseMessage::Toggle(name) => {
                let disabled = self.find_item(&name).map(|i| i.disabled).unwrap_or(true);
                if disabled {
                    return;
                }
                if self.active_keys.contains(&name) {
                    self.active_keys.remove(&name);
                } else {
                    if self.accordion {
                        self.active_keys.clear();
                    }
                    self.active_keys.insert(name);
                }
            }
            CollapseMessage::Open(name) => {
                let disabled = self.find_item(&name).map(|i| i.disabled).unwrap_or(true);
                if disabled {
                    return;
                }
                if self.accordion {
                    self.active_keys.clear();
                }
                self.active_keys.insert(name);
            }
            CollapseMessage::Close(name) => {
                self.active_keys.remove(&name);
            }
            CollapseMessage::OpenAll => {
                if self.accordion {
                    // 手风琴模式忽略 OpenAll
                    return;
                }
                for item in &self.items {
                    if !item.disabled {
                        self.active_keys.insert(item.name.clone());
                    }
                }
            }
            CollapseMessage::CloseAll => {
                self.active_keys.clear();
            }
        }
    }

    /// 渲染 Collapse 为 iced::Element
    ///
    /// # 参数
    /// - `theme`: HarUI 主题引用
    /// - `on_toggle`: 点击某项标题时发出消息，参数为该项 name
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        on_toggle: impl Fn(String) -> Message + 'a,
    ) -> Element<'a, Message> {
        if self.items.is_empty() {
            return container(text("")).into();
        }

        let text_primary = Color::from(theme.neutral.text_primary);
        let text_regular = Color::from(theme.neutral.text_regular);
        let text_disabled = Color::from(theme.neutral.text_disabled);
        let border_lighter = Color::from(theme.neutral.border_lighter);
        let primary = Color::from(theme.primary.base);

        let mut children: Vec<Element<'a, Message>> = Vec::new();

        for item in &self.items {
            let is_active = self.active_keys.contains(&item.name);
            let is_disabled = item.disabled;

            // 展开指示器：▶ 折叠 / ▼ 展开
            let indicator_str = if is_active { "▼" } else { "▶" };
            let indicator_color = if is_disabled {
                text_disabled
            } else if is_active {
                primary
            } else {
                text_regular
            };
            let title_color = if is_disabled {
                text_disabled
            } else {
                text_primary
            };

            let header_content = iced::widget::Row::new()
                .push(text(indicator_str).color(indicator_color).size(14.0))
                .push(iced::widget::Space::new().width(Length::Fixed(8.0)))
                .push(text(item.title.clone()).color(title_color).size(14.0))
                .align_y(iced::Alignment::Center);

            let mut header_btn = button(header_content)
                .padding(Padding::from([12u16, 16u16]))
                .style(move |_t, _status| iced::widget::button::Style {
                    background: None,
                    text_color: title_color,
                    border: iced::Border::default(),
                    shadow: iced::Shadow::default(),
                    snap: false,
                });
            if !is_disabled {
                header_btn = header_btn.on_press(on_toggle(item.name.clone()));
            }

            let mut item_children: Vec<Element<'a, Message>> = Vec::new();
            // header + 顶部边框
            let header_wrap = container(header_btn).width(Length::Fill).style(move |_t| {
                iced::widget::container::Style {
                    text_color: None,
                    background: None,
                    border: iced::Border {
                        color: border_lighter,
                        width: 1.0,
                        radius: iced::border::radius(0.0),
                    },
                    shadow: iced::Shadow::default(),
                    snap: false,
                }
            });
            item_children.push(header_wrap.into());

            // 展开内容（占位）
            if is_active {
                let content = container(
                    text("[content]")
                        .color(Color::from(theme.neutral.text_placeholder))
                        .size(13.0),
                )
                .width(Length::Fill)
                .padding(Padding::from([12u16, 40u16]))
                .style(move |_t| iced::widget::container::Style {
                    text_color: None,
                    background: None,
                    border: iced::Border {
                        color: border_lighter,
                        width: 1.0,
                        radius: iced::border::radius(0.0),
                    },
                    shadow: iced::Shadow::default(),
                    snap: false,
                });
                item_children.push(content.into());
            }

            let item_col = iced::widget::Column::with_children(item_children).spacing(0);
            children.push(item_col.into());
        }

        iced::widget::Column::with_children(children)
            .spacing(0)
            .into()
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_collapse_item_new() {
        let item = CollapseItem::new("a", "标题");
        assert_eq!(item.name(), "a");
        assert_eq!(item.title(), "标题");
        assert!(!item.disabled());
        assert!(!item.default_active());
    }
}
