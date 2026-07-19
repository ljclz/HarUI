//! HangOrder 挂单面板组件 — POS 挂单/取单管理
//!
//! 支持：挂起、取单、删除、清空、按时间倒序排列。

use har_ui_core::theme::Theme;
use iced::widget::{button, container, text};
use iced::{Color, Element, Length, Padding};

/// 挂单项
#[derive(Debug, Clone, PartialEq)]
pub struct HangOrderItem {
    pub id: String,
    pub customer_name: String,
    pub total: f64,
    pub item_count: usize,
    pub timestamp: i64,
}

/// 挂单消息
///
/// - `Hang(item)`: 挂起新订单（重复 ID 会被拒绝）
/// - `Take(id)`: 取单（恢复到当前购物车），返回 Option<HangOrderItem>
/// - `Delete(id)`: 删除挂单，返回 bool
/// - `ClearAll`: 清空所有挂单
#[derive(Debug, Clone, PartialEq)]
pub enum HangOrderMessage {
    Hang(HangOrderItem),
    Take(String),
    Delete(String),
    ClearAll,
}

/// HangOrder 挂单面板
#[derive(Debug, Clone, Default)]
pub struct HangOrder {
    items: Vec<HangOrderItem>,
}

impl HangOrder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn items(&self) -> &[HangOrderItem] {
        &self.items
    }

    pub fn count(&self) -> usize {
        self.items.len()
    }

    /// 处理消息，返回操作结果
    /// - Hang: 返回 None（无返回值）
    /// - Take: 返回 Some(item) 或 None
    /// - Delete: 返回 Some(()) 表示成功，None 表示失败
    /// - ClearAll: 返回 None
    pub fn handle(&mut self, msg: HangOrderMessage) -> Option<HangOrderItem> {
        match msg {
            HangOrderMessage::Hang(item) => {
                // 重复 ID 拒绝
                if self.items.iter().any(|i| i.id == item.id) {
                    return None;
                }
                // 按时间倒序插入（最新在前）
                let pos = self
                    .items
                    .iter()
                    .position(|i| i.timestamp < item.timestamp)
                    .unwrap_or(self.items.len());
                self.items.insert(pos, item);
                None
            }
            HangOrderMessage::Take(id) => {
                if let Some(pos) = self.items.iter().position(|i| i.id == id) {
                    Some(self.items.remove(pos))
                } else {
                    None
                }
            }
            HangOrderMessage::Delete(id) => {
                if let Some(pos) = self.items.iter().position(|i| i.id == id) {
                    self.items.remove(pos);
                    Some(HangOrderItem {
                        id: String::new(),
                        customer_name: String::new(),
                        total: 0.0,
                        item_count: 0,
                        timestamp: 0,
                    })
                } else {
                    None
                }
            }
            HangOrderMessage::ClearAll => {
                self.items.clear();
                None
            }
        }
    }

    /// 渲染 HangOrder 为 iced::Element
    ///
    /// # 参数
    /// - `theme`: HarUI 主题引用
    /// - `on_resume`: 点击恢复按钮时发出消息，参数为订单号
    /// - `on_delete`: 点击删除按钮时发出消息，参数为订单号
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        on_resume: impl Fn(String) -> Message + 'a,
        on_delete: impl Fn(String) -> Message + 'a,
    ) -> Element<'a, Message> {
        let text_placeholder = Color::from(theme.neutral.text_placeholder);
        let border_lighter = Color::from(theme.neutral.border_lighter);

        if self.items.is_empty() {
            return container(
                text("暂无挂单".to_string())
                    .color(text_placeholder)
                    .size(14.0),
            )
            .width(Length::Fill)
            .padding(Padding::from([20u16, 12u16]))
            .style(move |_t| iced::widget::container::Style {
                text_color: None,
                background: None,
                border: iced::Border {
                    color: border_lighter,
                    width: 1.0,
                    radius: iced::border::radius(4.0),
                },
                shadow: iced::Shadow::default(),
            })
            .into();
        }

        let mut children: Vec<Element<'a, Message>> = Vec::new();
        for item in &self.items {
            children.push(Self::render_item(
                item, theme, &on_resume, &on_delete,
            ));
        }

        let col = iced::widget::Column::with_children(children).spacing(4);
        container(col)
            .width(Length::Fill)
            .padding(Padding::from(4u16))
            .style(move |_t| iced::widget::container::Style {
                text_color: None,
                background: None,
                border: iced::Border {
                    color: border_lighter,
                    width: 1.0,
                    radius: iced::border::radius(4.0),
                },
                shadow: iced::Shadow::default(),
            })
            .into()
    }

    fn render_item<'a, Message: Clone + 'a>(
        item: &'a HangOrderItem,
        theme: &'a Theme,
        on_resume: &dyn Fn(String) -> Message,
        on_delete: &dyn Fn(String) -> Message,
    ) -> Element<'a, Message> {
        let primary = Color::from(theme.primary.base);
        let danger = Color::from(theme.danger.base);
        let text_primary = Color::from(theme.neutral.text_primary);
        let text_secondary = Color::from(theme.neutral.text_secondary);
        let border_lighter = Color::from(theme.neutral.border_lighter);

        let header = iced::widget::Row::new()
            .push(text(format!("#{}", item.id)).color(primary).size(14.0))
            .push(iced::widget::Space::with_width(Length::Fill))
            .push(text(format!("{} 件", item.item_count)).color(text_secondary).size(12.0))
            .align_y(iced::Alignment::Center);

        let body = iced::widget::Row::new()
            .push(text("客户".to_string()).color(text_secondary).size(12.0))
            .push(iced::widget::Space::with_width(Length::Fixed(4.0)))
            .push(text(item.customer_name.clone()).color(text_primary).size(13.0))
            .push(iced::widget::Space::with_width(Length::Fixed(12.0)))
            .push(text("金额".to_string()).color(text_secondary).size(12.0))
            .push(iced::widget::Space::with_width(Length::Fixed(4.0)))
            .push(text(format!("¥ {:.2}", item.total)).color(primary).size(13.0))
            .align_y(iced::Alignment::Center);

        let resume_msg = on_resume(item.id.clone());
        let delete_msg = on_delete(item.id.clone());
        let actions = iced::widget::Row::new()
            .push(
                button(text("恢复".to_string()).color(primary).size(12.0))
                    .padding(Padding::from([4u16, 8u16]))
                    .on_press(resume_msg)
                    .style(move |_t, _status| iced::widget::button::Style {
                        background: None,
                        text_color: primary,
                        border: iced::Border {
                            color: primary,
                            width: 1.0,
                            radius: iced::border::radius(4.0),
                        },
                        shadow: iced::Shadow::default(),
                    }),
            )
            .push(iced::widget::Space::with_width(Length::Fixed(4.0)))
            .push(
                button(text("删除".to_string()).color(danger).size(12.0))
                    .padding(Padding::from([4u16, 8u16]))
                    .on_press(delete_msg)
                    .style(move |_t, _status| iced::widget::button::Style {
                        background: None,
                        text_color: danger,
                        border: iced::Border {
                            color: danger,
                            width: 1.0,
                            radius: iced::border::radius(4.0),
                        },
                        shadow: iced::Shadow::default(),
                    }),
            )
            .spacing(0);

        let inner = iced::widget::Column::new()
            .push(header)
            .push(iced::widget::Space::with_height(Length::Fixed(4.0)))
            .push(body)
            .push(iced::widget::Space::with_height(Length::Fixed(4.0)))
            .push(actions);

        container(inner)
            .width(Length::Fill)
            .padding(Padding::from([8u16, 12u16]))
            .style(move |_t| iced::widget::container::Style {
                text_color: None,
                background: None,
                border: iced::Border {
                    color: border_lighter,
                    width: 1.0,
                    radius: iced::border::radius(4.0),
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
    fn test_hang_order_default_empty() {
        let h = HangOrder::new();
        assert_eq!(h.count(), 0);
        assert!(h.items().is_empty());
    }

    #[test]
    fn test_hang_order_take_returns_item() {
        let mut h = HangOrder::new();
        let item = HangOrderItem {
            id: "H001".to_string(),
            customer_name: "张三".to_string(),
            total: 88.50,
            item_count: 3,
            timestamp: 1700000000,
        };
        h.handle(HangOrderMessage::Hang(item.clone()));
        let taken = h.handle(HangOrderMessage::Take("H001".to_string()));
        assert_eq!(taken, Some(item));
    }

    #[test]
    fn test_hang_order_delete_returns_some_on_success() {
        let mut h = HangOrder::new();
        let item = HangOrderItem {
            id: "H001".to_string(),
            customer_name: "张三".to_string(),
            total: 88.50,
            item_count: 3,
            timestamp: 1700000000,
        };
        h.handle(HangOrderMessage::Hang(item));
        let result = h.handle(HangOrderMessage::Delete("H001".to_string()));
        assert!(result.is_some());
    }
}
