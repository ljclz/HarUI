//! Timeline 时间线 — 参考 Element Plus `<el-timeline>`。
//!
//! 支持：items、reverse、item type/color/size/placement/hollow、增删清空。

use har_ui_core::theme::Theme;
use iced::widget::{container, text};
use iced::{Color, Element, Length, Padding};

/// 节点类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TimelineItemType {
    #[default]
    Primary,
    Success,
    Warning,
    Danger,
    Info,
}

/// 节点尺寸
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TimelineItemSize {
    #[default]
    Normal,
    Large,
}

/// 节点位置
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TimelineItemPlacement {
    #[default]
    Top,
    Bottom,
}

/// Timeline 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimelineMessage {
    /// 追加一项
    AddItem(TimelineItem),
    /// 移除指定索引（越界无操作）
    RemoveAt(usize),
    /// 清空
    Clear,
    /// 设置是否反向
    SetReverse(bool),
}

/// Timeline 节点
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimelineItem {
    timestamp: String,
    content: String,
    item_type: TimelineItemType,
    color: Option<String>,
    size: TimelineItemSize,
    placement: TimelineItemPlacement,
    hollow: bool,
}

impl TimelineItem {
    pub fn new(timestamp: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            timestamp: timestamp.into(),
            content: content.into(),
            item_type: TimelineItemType::Primary,
            color: None,
            size: TimelineItemSize::Normal,
            placement: TimelineItemPlacement::Top,
            hollow: false,
        }
    }

    pub fn with_type(mut self, t: TimelineItemType) -> Self {
        self.item_type = t;
        self
    }

    pub fn with_color(mut self, c: impl Into<String>) -> Self {
        self.color = Some(c.into());
        self
    }

    pub fn with_size(mut self, s: TimelineItemSize) -> Self {
        self.size = s;
        self
    }

    pub fn with_placement(mut self, p: TimelineItemPlacement) -> Self {
        self.placement = p;
        self
    }

    pub fn with_hollow(mut self, v: bool) -> Self {
        self.hollow = v;
        self
    }

    pub fn timestamp(&self) -> &str {
        &self.timestamp
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn item_type(&self) -> TimelineItemType {
        self.item_type
    }

    pub fn color(&self) -> Option<&str> {
        self.color.as_deref()
    }

    pub fn size(&self) -> TimelineItemSize {
        self.size
    }

    pub fn placement(&self) -> TimelineItemPlacement {
        self.placement
    }

    pub fn hollow(&self) -> bool {
        self.hollow
    }
}

/// Timeline 组件
#[derive(Debug, Clone)]
pub struct Timeline {
    items: Vec<TimelineItem>,
    reverse: bool,
}

impl Default for Timeline {
    fn default() -> Self {
        Self::new()
    }
}

impl Timeline {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            reverse: false,
        }
    }

    pub fn with_reverse(mut self, v: bool) -> Self {
        self.reverse = v;
        self
    }

    pub fn reverse(&self) -> bool {
        self.reverse
    }

    pub fn items(&self) -> &[TimelineItem] {
        &self.items
    }

    pub fn handle(&mut self, msg: TimelineMessage) {
        match msg {
            TimelineMessage::AddItem(item) => {
                self.items.push(item);
            }
            TimelineMessage::RemoveAt(idx) => {
                if idx < self.items.len() {
                    self.items.remove(idx);
                }
            }
            TimelineMessage::Clear => {
                self.items.clear();
            }
            TimelineMessage::SetReverse(v) => {
                self.reverse = v;
            }
        }
    }

    /// 按 item_type 返回节点颜色
    fn type_color(t: TimelineItemType, theme: &Theme) -> Color {
        match t {
            TimelineItemType::Primary => Color::from(theme.primary.base),
            TimelineItemType::Success => Color::from(theme.success.base),
            TimelineItemType::Warning => Color::from(theme.warning.base),
            TimelineItemType::Danger => Color::from(theme.danger.base),
            TimelineItemType::Info => Color::from(theme.info.base),
        }
    }

    /// 解析 hex 颜色，失败返回 fallback
    fn parse_hex(hex: &str, fallback: Color) -> Color {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return fallback;
        }
        let r = u8::from_str_radix(&hex[0..2], 16).ok();
        let g = u8::from_str_radix(&hex[2..4], 16).ok();
        let b = u8::from_str_radix(&hex[4..6], 16).ok();
        match (r, g, b) {
            (Some(r), Some(g), Some(b)) => Color::from_rgb8(r, g, b), // HARUI-EXCEPTION: 用户自定义节点色动态转换
            _ => fallback,
        }
    }

    /// 渲染单个 Timeline 节点
    fn render_item<'a>(item: &'a TimelineItem, theme: &'a Theme) -> Element<'a, ()> {
        let fallback = Self::type_color(item.item_type(), theme);
        let node_color = item
            .color()
            .map(|c| Self::parse_hex(c, fallback))
            .unwrap_or(fallback);

        let node_size = match item.size() {
            TimelineItemSize::Normal => 12.0,
            TimelineItemSize::Large => 16.0,
        };
        let content_size = match item.size() {
            TimelineItemSize::Normal => 14.0,
            TimelineItemSize::Large => 16.0,
        };

        // 节点（实心/空心圆）
        let node_indicator = container(text(""))
            .width(Length::Fixed(node_size))
            .height(Length::Fixed(node_size))
            .style(move |_t| iced::widget::container::Style {
                text_color: None,
                background: if item.hollow() {
                    None
                } else {
                    Some(iced::Background::Color(node_color))
                },
                border: iced::Border {
                    color: node_color,
                    width: 2.0,
                    radius: iced::border::radius(node_size / 2.0),
                },
                shadow: iced::Shadow::default(),
            });

        // 时间戳 + 内容
        let timestamp_text = text(item.timestamp().to_string())
            .color(Color::from(theme.neutral.text_secondary))
            .size(content_size - 2.0);
        let content_text = text(item.content().to_string())
            .color(Color::from(theme.neutral.text_primary))
            .size(content_size);

        let text_col = iced::widget::Column::new()
            .push(timestamp_text)
            .push(iced::widget::Space::with_height(Length::Fixed(2.0)))
            .push(content_text)
            .spacing(0);

        // 整行：节点 + 文本
        let row = iced::widget::Row::new()
            .push(node_indicator)
            .push(iced::widget::Space::with_width(Length::Fixed(12.0)))
            .push(text_col)
            .align_y(iced::alignment::Vertical::Center)
            .spacing(0);

        container(row)
            .width(Length::Fill)
            .padding(Padding::from([8u16, 0u16]))
            .into()
    }

    /// 渲染 Timeline 为 iced::Element
    pub fn view<'a>(&'a self, theme: &'a Theme) -> Element<'a, ()> {
        if self.items.is_empty() {
            return container(text("")).width(Length::Fill).into();
        }

        // reverse: 反向显示
        let items: Vec<&TimelineItem> = if self.reverse {
            self.items.iter().rev().collect()
        } else {
            self.items.iter().collect()
        };

        let children: Vec<Element<'a, ()>> = items
            .iter()
            .map(|item| Self::render_item(item, theme))
            .collect();

        iced::widget::Column::with_children(children)
            .spacing(0)
            .into()
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_timeline_item_defaults() {
        let item = TimelineItem::new("2024-01-01", "事件");
        assert_eq!(item.timestamp(), "2024-01-01");
        assert_eq!(item.content(), "事件");
        assert_eq!(item.item_type(), TimelineItemType::Primary);
        assert!(item.color().is_none());
        assert_eq!(item.size(), TimelineItemSize::Normal);
        assert_eq!(item.placement(), TimelineItemPlacement::Top);
        assert!(!item.hollow());
    }
}
