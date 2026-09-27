//! Timeline 时间线 — 参考 Element Plus `<el-timeline>`。
//!
//! 覆盖：items、reverse、type、color、size、placement、hollow。

use har_ui_components::timeline::{
    Timeline, TimelineItem, TimelineItemPlacement, TimelineItemSize, TimelineItemType,
    TimelineMessage,
};

#[test]
fn test_timeline_default() {
    let t = Timeline::new();
    assert!(!t.reverse());
    assert!(t.items().is_empty());
}

#[test]
fn test_timeline_with_reverse() {
    let t = Timeline::new().with_reverse(true);
    assert!(t.reverse());
}

#[test]
fn test_timeline_add_item() {
    let mut t = Timeline::new();
    t.handle(TimelineMessage::AddItem(TimelineItem::new(
        "2024-01-01",
        "上线",
    )));
    t.handle(TimelineMessage::AddItem(TimelineItem::new(
        "2024-02-01",
        "迭代",
    )));
    assert_eq!(t.items().len(), 2);
    assert_eq!(t.items()[0].timestamp(), "2024-01-01");
    assert_eq!(t.items()[1].content(), "迭代");
}

#[test]
fn test_timeline_remove_at() {
    let mut t = Timeline::new();
    t.handle(TimelineMessage::AddItem(TimelineItem::new("t1", "A")));
    t.handle(TimelineMessage::AddItem(TimelineItem::new("t2", "B")));
    t.handle(TimelineMessage::AddItem(TimelineItem::new("t3", "C")));
    t.handle(TimelineMessage::RemoveAt(1));
    assert_eq!(t.items().len(), 2);
    assert_eq!(t.items()[0].content(), "A");
    assert_eq!(t.items()[1].content(), "C");
}

#[test]
fn test_timeline_remove_at_out_of_bounds_noop() {
    let mut t = Timeline::new();
    t.handle(TimelineMessage::AddItem(TimelineItem::new("t1", "A")));
    t.handle(TimelineMessage::RemoveAt(5));
    assert_eq!(t.items().len(), 1);
}

#[test]
fn test_timeline_clear() {
    let mut t = Timeline::new();
    t.handle(TimelineMessage::AddItem(TimelineItem::new("t1", "A")));
    t.handle(TimelineMessage::AddItem(TimelineItem::new("t2", "B")));
    t.handle(TimelineMessage::Clear);
    assert!(t.items().is_empty());
}

#[test]
fn test_timeline_item_with_type() {
    let item = TimelineItem::new("t1", "A").with_type(TimelineItemType::Success);
    assert_eq!(item.item_type(), TimelineItemType::Success);
}

#[test]
fn test_timeline_item_with_color_override() {
    let item = TimelineItem::new("t1", "A").with_color("#ff0000");
    assert_eq!(item.color(), Some("#ff0000"));
}

#[test]
fn test_timeline_item_with_size() {
    let item = TimelineItem::new("t1", "A").with_size(TimelineItemSize::Large);
    assert_eq!(item.size(), TimelineItemSize::Large);
}

#[test]
fn test_timeline_item_with_placement() {
    let item = TimelineItem::new("t1", "A").with_placement(TimelineItemPlacement::Bottom);
    assert_eq!(item.placement(), TimelineItemPlacement::Bottom);
}

#[test]
fn test_timeline_item_hollow() {
    let item = TimelineItem::new("t1", "A").with_hollow(true);
    assert!(item.hollow());
}

#[test]
fn test_timeline_set_reverse() {
    let mut t = Timeline::new();
    t.handle(TimelineMessage::AddItem(TimelineItem::new("t1", "A")));
    t.handle(TimelineMessage::AddItem(TimelineItem::new("t2", "B")));
    t.handle(TimelineMessage::AddItem(TimelineItem::new("t3", "C")));
    t.handle(TimelineMessage::SetReverse(true));
    assert!(t.reverse());
    // 视图层应反向遍历，items 内部仍保留插入顺序
    assert_eq!(t.items()[0].content(), "A");
    assert_eq!(t.items()[2].content(), "C");
}
