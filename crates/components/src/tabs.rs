//! Tabs 组件 — 标签页
//!
//! 参考 Element Plus `<el-tabs>`。
//! 支持：default/card/border-card 三种 type，top/bottom/left/right 四种位置，
//! closable/addable/lazy，切换/关闭/新增。

use har_ui_core::theme::Theme;
use iced::widget::{button, container, text};
use iced::{Color, Element, Length, Padding};

/// Tabs 类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TabsType {
    #[default]
    Default,
    Card,
    BorderCard,
}

/// Tab 位置
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TabPosition {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

/// 单个 Tab 项
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TabItem {
    pub id: String,
    pub label: String,
    pub disabled: bool,
    pub closable: bool,
}

impl TabItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            disabled: false,
            closable: false,
        }
    }

    pub fn disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }

    pub fn closable(mut self, v: bool) -> Self {
        self.closable = v;
        self
    }
}

/// Tabs 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TabsMessage {
    /// 切换到指定 tab
    Select(String),
    /// 关闭指定 tab
    Close(String),
    /// 新增 tab
    Add(TabItem),
}

/// Tabs 组件
#[derive(Debug, Clone)]
pub struct Tabs {
    items: Vec<TabItem>,
    tabs_type: TabsType,
    position: TabPosition,
    closable: bool,
    addable: bool,
    lazy: bool,
    active: Option<String>,
    /// lazy 模式下已访问的 tab id
    visited: Vec<String>,
}

impl Default for Tabs {
    fn default() -> Self {
        Self::new()
    }
}

impl Tabs {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            tabs_type: TabsType::Default,
            position: TabPosition::Top,
            closable: false,
            addable: false,
            lazy: false,
            active: None,
            visited: Vec::new(),
        }
    }

    pub fn with_type(mut self, t: TabsType) -> Self {
        self.tabs_type = t;
        self
    }

    pub fn with_position(mut self, p: TabPosition) -> Self {
        self.position = p;
        self
    }

    pub fn with_closable(mut self, v: bool) -> Self {
        self.closable = v;
        self
    }

    pub fn with_addable(mut self, v: bool) -> Self {
        self.addable = v;
        self
    }

    pub fn with_lazy(mut self, v: bool) -> Self {
        self.lazy = v;
        self
    }

    pub fn with_item(mut self, item: TabItem) -> Self {
        if self.items.is_empty() {
            // 第一个 tab 默认激活
            self.active = Some(item.id.clone());
            self.visited.push(item.id.clone());
        }
        self.items.push(item);
        self
    }

    pub fn items(&self) -> &[TabItem] {
        &self.items
    }

    pub fn tabs_type(&self) -> TabsType {
        self.tabs_type
    }

    pub fn position(&self) -> TabPosition {
        self.position
    }

    pub fn closable(&self) -> bool {
        self.closable
    }

    pub fn addable(&self) -> bool {
        self.addable
    }

    pub fn lazy(&self) -> bool {
        self.lazy
    }

    pub fn active(&self) -> Option<&String> {
        self.active.as_ref()
    }

    /// 判断 tab 是否已访问（lazy 模式有效）
    pub fn is_visited(&self, id: &str) -> bool {
        if !self.lazy {
            return true;
        }
        self.visited.iter().any(|v| v == id)
    }

    /// 处理消息
    pub fn handle(&mut self, msg: TabsMessage) {
        match msg {
            TabsMessage::Select(id) => {
                // 检查是否存在且非 disabled
                let item = match self.items.iter().find(|i| i.id == id) {
                    Some(i) if !i.disabled => i.clone(),
                    _ => return,
                };
                self.active = Some(id.clone());
                if self.lazy && !self.visited.iter().any(|v| v == &id) {
                    self.visited.push(id);
                }
                let _ = item;
            }
            TabsMessage::Close(id) => {
                if let Some(pos) = self.items.iter().position(|i| i.id == id) {
                    let was_active = self.active.as_ref() == Some(&id);
                    self.items.remove(pos);
                    self.visited.retain(|v| v != &id);
                    if was_active {
                        // 自动激活相邻 tab
                        if !self.items.is_empty() {
                            let new_pos = pos.min(self.items.len() - 1);
                            self.active = Some(self.items[new_pos].id.clone());
                            if let Some(active_id) = self.active.as_ref() {
                                if self.lazy
                                    && !self.visited.iter().any(|v| v == active_id)
                                {
                                    self.visited.push(active_id.clone());
                                }
                            }
                        } else {
                            self.active = None;
                        }
                    }
                }
            }
            TabsMessage::Add(item) => {
                // 重复 ID 拒绝
                if self.items.iter().any(|i| i.id == item.id) {
                    return;
                }
                let new_id = item.id.clone();
                self.items.push(item);
                self.active = Some(new_id.clone());
                if self.lazy {
                    self.visited.push(new_id);
                }
            }
        }
    }

    /// 渲染 Tabs 为 iced::Element
    ///
    /// # 参数
    /// - `theme`: HarUI 主题引用
    /// - `on_select`: 点击某个 tab 时发出消息，参数为该 tab 的 id
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        on_select: impl Fn(String) -> Message + 'a,
    ) -> Element<'a, Message> {
        if self.items.is_empty() {
            return container(text("")).width(Length::Fill).into();
        }

        let text_primary = Color::from(theme.neutral.text_primary);
        let text_regular = Color::from(theme.neutral.text_regular);
        let text_disabled = Color::from(theme.neutral.text_disabled);
        let border_light = Color::from(theme.neutral.border_light);
        let primary = Color::from(theme.primary.base);

        let is_card = matches!(self.tabs_type, TabsType::Card | TabsType::BorderCard);

        let mut children: Vec<Element<'a, Message>> = Vec::new();
        for item in &self.items {
            let is_active = self.active.as_deref() == Some(item.id.as_str());
            let is_disabled = item.disabled;

            let label_color = if is_disabled {
                text_disabled
            } else if is_active {
                if is_card { Color::WHITE } else { primary }
            } else {
                text_regular
            };

            let mut label_row = iced::widget::Row::new()
                .push(text(item.label.clone()).color(label_color).size(14.0))
                .align_y(iced::Alignment::Center);
            if item.closable && !is_disabled {
                label_row = label_row.push(
                    iced::widget::Space::with_width(Length::Fixed(6.0)),
                ).push(
                    text("×").color(text_regular).size(14.0),
                );
            }

            let active_bg = if is_active && is_card {
                Some(iced::Background::Color(primary))
            } else {
                None
            };

            let mut btn = button(label_row)
                .padding(Padding::from([8u16, 16u16]))
                .style(move |_t, _status| iced::widget::button::Style {
                    background: active_bg,
                    text_color: label_color,
                    border: iced::Border {
                        color: border_light,
                        width: 0.0,
                        radius: iced::border::radius(4.0),
                    },
                    shadow: iced::Shadow::default(),
                });
            if !is_disabled {
                btn = btn.on_press(on_select(item.id.clone()));
            }
            children.push(btn.into());
        }

        let header_row = iced::widget::Row::with_children(children)
            .spacing(4)
            .align_y(iced::Alignment::Center);

        let header_wrap = container(header_row)
            .width(Length::Fill)
            .padding(Padding::from(0u16))
            .style(move |_t| iced::widget::container::Style {
                text_color: None,
                background: None,
                border: iced::Border {
                    color: border_light,
                    width: 0.0,
                    radius: iced::border::radius(0.0),
                },
                shadow: iced::Shadow::default(),
            });

        let mut outer: Vec<Element<'a, Message>> = Vec::new();
        outer.push(header_wrap.into());

        // 当前 active tab 的内容占位
        if let Some(active_id) = self.active.as_ref() {
            if let Some(item) = self.items.iter().find(|i| &i.id == active_id) {
                let body = container(
                    text(format!("[{} content]", item.label))
                        .color(text_primary)
                        .size(13.0),
                )
                .width(Length::Fill)
                .padding(Padding::from(12u16));
                outer.push(body.into());
            }
        }

        iced::widget::Column::with_children(outer)
            .spacing(0)
            .into()
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_tabs_default_no_items_no_active() {
        let t = Tabs::new();
        assert!(t.items().is_empty());
        assert_eq!(t.active(), None);
    }

    #[test]
    fn test_tabs_first_item_becomes_active() {
        let t = Tabs::new().with_item(TabItem::new("first", "First"));
        assert_eq!(t.active(), Some(&"first".to_string()));
    }

    #[test]
    fn test_tabs_close_active_falls_back_to_neighbor() {
        let mut t = Tabs::new()
            .with_item(TabItem::new("t1", "1").closable(true))
            .with_item(TabItem::new("t2", "2").closable(true))
            .with_item(TabItem::new("t3", "3").closable(true));
        t.handle(TabsMessage::Select("t2".to_string()));
        t.handle(TabsMessage::Close("t2".to_string()));
        // t2 关闭后应激活 t1 或 t3
        let active = t.active().unwrap();
        assert!(active == "t1" || active == "t3");
    }
}
