//! Timeline 时间线 — 参考 Element Plus `<el-timeline>`。
//!
//! 支持：items、reverse、item type/color/size/placement/hollow、增删清空。

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
