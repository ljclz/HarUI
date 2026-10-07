//! Notification 通知组件 — 参考 Element Plus `ElNotification`。
//! 支持：4 种 type、4 种 position、duration 自动关闭（tick 模拟）、offset、show_close、NotificationList 多通知堆叠。

use har_ui_core::theme::Theme;
use iced::widget::{container, text};
use iced::{Color, Element, Length, Padding};

/// Notification 类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NotificationType {
    Success,
    Warning,
    #[default]
    Info,
    Error,
}

impl NotificationType {
    pub fn as_str(self) -> &'static str {
        match self {
            NotificationType::Success => "success",
            NotificationType::Warning => "warning",
            NotificationType::Info => "info",
            NotificationType::Error => "error",
        }
    }
}

/// Notification 显示位置
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NotificationPosition {
    #[default]
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
}

impl NotificationPosition {
    pub fn as_str(self) -> &'static str {
        match self {
            NotificationPosition::TopRight => "top-right",
            NotificationPosition::TopLeft => "top-left",
            NotificationPosition::BottomRight => "bottom-right",
            NotificationPosition::BottomLeft => "bottom-left",
        }
    }
}

/// Notification 消息
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationMessage {
    /// 关闭通知
    Close,
}

/// Notification 单条通知
#[derive(Debug, Clone)]
pub struct Notification {
    /// 标题
    title: String,
    /// 消息内容
    message: String,
    /// 类型
    msg_type: NotificationType,
    /// 显示时长（毫秒），0 表示不自动关闭
    duration: u64,
    /// 显示位置
    position: NotificationPosition,
    /// 是否显示关闭按钮
    show_close: bool,
    /// 相对偏移量（基础偏移）
    offset: u32,
    /// 是否已关闭
    closed: bool,
    /// 距显示以来的累计毫秒
    elapsed: u64,
}

impl Notification {
    pub fn new(title: &str, message: &str) -> Self {
        Self {
            title: title.to_string(),
            message: message.to_string(),
            msg_type: NotificationType::Info,
            duration: 4500,
            position: NotificationPosition::TopRight,
            show_close: true,
            offset: 0,
            closed: false,
            elapsed: 0,
        }
    }

    // ---------- Builder ----------

    pub fn with_type(mut self, t: NotificationType) -> Self {
        self.msg_type = t;
        self
    }

    pub fn with_duration(mut self, d: u64) -> Self {
        self.duration = d;
        self
    }

    pub fn with_position(mut self, p: NotificationPosition) -> Self {
        self.position = p;
        self
    }

    pub fn with_show_close(mut self, s: bool) -> Self {
        self.show_close = s;
        self
    }

    pub fn with_offset(mut self, o: u32) -> Self {
        self.offset = o;
        self
    }

    // ---------- Getter ----------

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn msg_type(&self) -> NotificationType {
        self.msg_type
    }

    pub fn duration(&self) -> u64 {
        self.duration
    }

    pub fn position(&self) -> NotificationPosition {
        self.position
    }

    pub fn show_close(&self) -> bool {
        self.show_close
    }

    pub fn offset(&self) -> u32 {
        self.offset
    }

    pub fn closed(&self) -> bool {
        self.closed
    }

    // ---------- 时间推进 ----------

    /// 推进时间，超过 duration 自动关闭（duration=0 永不关闭）
    pub fn tick(&mut self, ms: u64) {
        if self.closed {
            return;
        }
        if self.duration == 0 {
            return;
        }
        self.elapsed += ms;
        if self.elapsed >= self.duration {
            self.closed = true;
        }
    }

    // ---------- 消息处理 ----------

    pub fn handle(&mut self, msg: NotificationMessage) {
        match msg {
            NotificationMessage::Close => {
                self.closed = true;
            }
        }
    }

    /// 渲染 Notification 为 iced::Element
    ///
    /// - `closed == true` 时返回空容器
    /// - 否则按 msg_type 上色，渲染标题 + 内容 + 可选关闭按钮
    pub fn view<'a>(&'a self, theme: &'a Theme) -> Element<'a, ()> {
        if self.closed {
            return container(text("")).into();
        }

        let (icon, accent) = match self.msg_type {
            NotificationType::Success => ("✓", Color::from(theme.success.base)),
            NotificationType::Warning => ("⚠", Color::from(theme.warning.base)),
            NotificationType::Info => ("ℹ", Color::from(theme.info.base)),
            NotificationType::Error => ("✕", Color::from(theme.danger.base)),
        };
        let title_color = Color::from(theme.neutral.text_primary);
        let content_color = Color::from(theme.neutral.text_regular);
        let bg = Color::from(theme.neutral.bg_overlay);
        let border_color = Color::from(theme.neutral.border_lighter);

        let title_text = text(self.title.clone()).color(title_color).size(16.0);
        let content_text = text(self.message.clone()).color(content_color).size(14.0);

        let mut header_children: Vec<Element<'a, ()>> = vec![
            text(icon).color(accent).size(18.0).into(),
            iced::widget::Space::with_width(Length::Fixed(8.0)).into(),
            title_text.into(),
            iced::widget::Space::with_width(Length::Fill).into(),
        ];
        if self.show_close {
            header_children.push(text("×").color(content_color).size(16.0).into());
        }
        let header = iced::widget::Row::with_children(header_children)
            .align_y(iced::Alignment::Center)
            .spacing(0);

        let col = iced::widget::Column::new()
            .push(header)
            .push(iced::widget::Space::with_height(Length::Fixed(6.0)))
            .push(content_text);

        container(col)
            .width(Length::Fixed(320.0))
            .padding(Padding::from([14u16, 16u16]))
            .style(move |_t| iced::widget::container::Style {
                text_color: None,
                background: Some(iced::Background::Color(bg)),
                border: iced::Border {
                    color: border_color,
                    width: 1.0,
                    radius: iced::border::radius(4.0),
                },
                shadow: iced::Shadow::default(),
            })
            .into()
    }
}

/// Notification 列表（支持堆叠）
#[derive(Debug, Clone, Default)]
pub struct NotificationList {
    items: Vec<Notification>,
}

impl NotificationList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn push(&mut self, n: Notification) {
        self.items.push(n);
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = &Notification> {
        self.items.iter()
    }

    /// 关闭指定 index 的通知（移除）
    pub fn close(&mut self, index: usize) {
        if index < self.items.len() {
            self.items.remove(index);
        }
    }

    /// 推进所有通知的时间，自动移除已关闭的
    pub fn tick(&mut self, ms: u64) {
        for n in &mut self.items {
            n.tick(ms);
        }
        self.items.retain(|n| !n.closed());
    }

    /// 计算每个通知堆叠后的实际 offset
    /// gap: 通知之间的间距（实现委托 core 行为层 `behavior::stack`，ADR-009；
    /// 各项附加 offset 并入累加位，通知高度折算进 gap，与既有口径一致）
    pub fn stacked_offsets(&self, gap: u32) -> Vec<u32> {
        let extras: Vec<u32> = self.items.iter().map(|n| n.offset()).collect();
        har_ui_core::behavior::stack::stacked_offsets(&extras, gap)
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_notification_internal_default() {
        let n = Notification::new("t", "m");
        assert_eq!(n.title, "t");
        assert_eq!(n.message, "m");
        assert_eq!(n.duration, 4500);
        assert_eq!(n.msg_type, NotificationType::Info);
        assert!(!n.closed);
    }

    #[test]
    fn test_notification_internal_tick_zero_duration_skipped() {
        let mut n = Notification::new("x", "y").with_duration(0);
        n.tick(100000);
        assert_eq!(n.elapsed, 0); // duration=0 时不推进
        assert!(!n.closed);
    }

    #[test]
    fn test_notification_list_internal_stacked_offsets() {
        let mut list = NotificationList::new();
        list.push(Notification::new("t1", "m1").with_offset(0));
        list.push(Notification::new("t2", "m2").with_offset(0));
        let offsets = list.stacked_offsets(60);
        assert_eq!(offsets[0], 0);
        assert_eq!(offsets[1], 60);
    }
}
