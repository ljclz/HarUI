//! Toast/Message 组件 — 全局消息提示
//!
//! 参考 Element Plus `ElMessage`。
//! 支持：4 种 type、自动关闭(duration)、手动关闭、叠加展示、垂直堆叠 offset。
//! 通过 MessageManager 集中管理多条消息的创建/关闭/自动过期。

use har_ui_core::theme::Theme;
use iced::widget::{container, text};
use iced::{Color, Element, Length, Padding};

/// 消息类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MessageType {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

/// 单条消息
#[derive(Debug, Clone)]
pub struct MessageItem {
    text: String,
    msg_type: MessageType,
    /// 自动关闭时长（毫秒），0 表示不自动关闭
    duration: u64,
    /// 已经过的时间（毫秒）
    elapsed: u64,
    closed: bool,
}

impl MessageItem {
    pub fn new(text: impl Into<String>, msg_type: MessageType) -> Self {
        Self {
            text: text.into(),
            msg_type,
            duration: 3000,
            elapsed: 0,
            closed: false,
        }
    }

    pub fn with_duration(mut self, ms: u64) -> Self {
        self.duration = ms;
        self
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn msg_type(&self) -> MessageType {
        self.msg_type
    }

    pub fn duration(&self) -> u64 {
        self.duration
    }

    pub fn closed(&self) -> bool {
        self.closed
    }

    /// 推进时间，返回是否应被关闭
    pub fn tick(&mut self, ms: u64) -> bool {
        if self.duration == 0 {
            return false; // 不自动关闭
        }
        self.elapsed += ms;
        if self.elapsed >= self.duration {
            self.closed = true;
            true
        } else {
            false
        }
    }

    /// 渲染 MessageItem 为 iced::Element
    ///
    /// - `closed == true` 时返回空容器
    /// - 否则按 msg_type 上色：图标 + 文本
    pub fn view<'a>(&'a self, theme: &'a Theme) -> Element<'a, ()> {
        if self.closed {
            return container(text("")).into();
        }

        let (icon, accent) = match self.msg_type {
            MessageType::Info => ("ℹ", Color::from(theme.info.base)),
            MessageType::Success => ("✓", Color::from(theme.success.base)),
            MessageType::Warning => ("⚠", Color::from(theme.warning.base)),
            MessageType::Error => ("✕", Color::from(theme.danger.base)),
        };
        let text_color = Color::from(theme.neutral.text_primary);
        let bg = Color::from(theme.neutral.bg_overlay);

        let row = iced::widget::Row::new()
            .push(text(icon).color(accent).size(16.0))
            .push(iced::widget::Space::with_width(Length::Fixed(8.0)))
            .push(text(self.text.clone()).color(text_color).size(14.0))
            .align_y(iced::Alignment::Center)
            .padding(Padding::from([10u16, 16u16]));

        container(row)
            .width(Length::Fill)
            .style(move |_t| iced::widget::container::Style {
                text_color: None,
                background: Some(iced::Background::Color(bg)),
                border: iced::Border {
                    color: accent,
                    width: 1.0,
                    radius: iced::border::radius(4.0),
                },
                shadow: iced::Shadow::default(),
            })
            .into()
    }
}

/// 默认首条 offset
const DEFAULT_OFFSET: u32 = 20;
/// 默认间距
const DEFAULT_GAP: u32 = 16;
/// 默认条目高度
const DEFAULT_ITEM_HEIGHT: u32 = 40;

/// 消息管理器（全局单例）
#[derive(Debug, Clone, Default)]
pub struct MessageManager {
    items: Vec<(u64, MessageItem)>,
    next_id: u64,
}

impl MessageManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn items(&self) -> Vec<&MessageItem> {
        self.items.iter().map(|(_, i)| i).collect()
    }

    /// 显示一条消息，返回 ID
    pub fn show(&mut self, item: MessageItem) -> u64 {
        self.next_id += 1;
        let id = self.next_id;
        self.items.push((id, item));
        id
    }

    /// 关闭指定 ID 的消息，返回是否成功
    pub fn close(&mut self, id: u64) -> bool {
        let before = self.items.len();
        self.items.retain(|(i, _)| *i != id);
        self.items.len() < before
    }

    /// 关闭所有消息
    pub fn close_all(&mut self) {
        self.items.clear();
    }

    /// 推进时间，自动清理过期消息
    pub fn tick(&mut self, ms: u64) {
        let items = &mut self.items;
        for (_, item) in items.iter_mut() {
            item.tick(ms);
        }
        items.retain(|(_, item)| !item.closed());
    }

    /// 计算每条消息的垂直 offset
    pub fn offsets(&self) -> Vec<u32> {
        let mut result = Vec::with_capacity(self.items.len());
        let mut acc = DEFAULT_OFFSET;
        for (_, _) in &self.items {
            result.push(acc);
            acc += DEFAULT_ITEM_HEIGHT + DEFAULT_GAP;
        }
        result
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_message_item_default_duration() {
        let item = MessageItem::new("hi", MessageType::Info);
        assert_eq!(item.duration(), 3000);
    }

    #[test]
    fn test_message_item_tick_auto_close() {
        let mut item = MessageItem::new("hi", MessageType::Info).with_duration(1000);
        assert!(!item.tick(500));
        assert!(item.tick(500));
        assert!(item.closed());
    }

    #[test]
    fn test_message_manager_close_removes_item() {
        let mut m = MessageManager::new();
        let id = m.show(MessageItem::new("hi", MessageType::Info));
        assert!(m.close(id));
        assert_eq!(m.items().len(), 0);
    }
}
