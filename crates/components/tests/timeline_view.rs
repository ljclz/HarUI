//! R.2.P1.10 Timeline view() 测试 — TDD RED 阶段

use har_ui_components::timeline::{Timeline, TimelineItem, TimelineItemType, TimelineMessage};
use har_ui_core::theme::Theme;

#[test]
fn test_timeline_view_empty_renders() {
    let theme = Theme::element_light();
    let t = Timeline::new();
    let _element = t.view(&theme);
}

#[test]
fn test_timeline_view_with_items_renders() {
    let theme = Theme::element_light();
    let mut t = Timeline::new();
    t.handle(TimelineMessage::AddItem(TimelineItem::new(
        "2024-01-01",
        "事件 A",
    )));
    t.handle(TimelineMessage::AddItem(TimelineItem::new(
        "2024-01-02",
        "事件 B",
    )));
    let _element = t.view(&theme);
}

#[test]
fn test_timeline_view_with_type_renders() {
    let theme = Theme::element_light();
    let mut t = Timeline::new();
    t.handle(TimelineMessage::AddItem(
        TimelineItem::new("2024-01-01", "成功").with_type(TimelineItemType::Success),
    ));
    let _element = t.view(&theme);
}

#[test]
fn test_timeline_view_reverse_renders() {
    let theme = Theme::element_light();
    let mut t = Timeline::new().with_reverse(true);
    t.handle(TimelineMessage::AddItem(TimelineItem::new(
        "2024-01-01",
        "A",
    )));
    t.handle(TimelineMessage::AddItem(TimelineItem::new(
        "2024-01-02",
        "B",
    )));
    let _element = t.view(&theme);
}

#[test]
fn test_timeline_view_with_custom_color_renders() {
    let theme = Theme::element_light();
    let mut t = Timeline::new();
    t.handle(TimelineMessage::AddItem(
        TimelineItem::new("2024-01-01", "事件").with_color("#FF0000"),
    ));
    let _element = t.view(&theme);
}

#[test]
fn test_timeline_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let mut t = Timeline::new();
    t.handle(TimelineMessage::AddItem(TimelineItem::new(
        "2024-01-01",
        "事件",
    )));
    let _element = t.view(&theme);
}
