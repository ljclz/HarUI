//! R.2.P1.9 Descriptions view() 测试 — TDD RED 阶段

use har_ui_components::descriptions::{Descriptions, DescriptionsDirection, DescriptionsItem};
use har_ui_core::theme::Theme;

#[test]
fn test_descriptions_view_empty_renders() {
    let theme = Theme::element_light();
    let d = Descriptions::new();
    let _element = d.view(&theme);
}

#[test]
fn test_descriptions_view_with_items_renders() {
    let theme = Theme::element_light();
    let d = Descriptions::new()
        .with_item(DescriptionsItem::new("Name", "Alice"))
        .with_item(DescriptionsItem::new("Age", "30"));
    let _element = d.view(&theme);
}

#[test]
fn test_descriptions_view_with_title_renders() {
    let theme = Theme::element_light();
    let d = Descriptions::new()
        .with_title("User Info")
        .with_item(DescriptionsItem::new("Name", "Alice"));
    let _element = d.view(&theme);
}

#[test]
fn test_descriptions_view_no_border_renders() {
    let theme = Theme::element_light();
    let d = Descriptions::new()
        .with_border(false)
        .with_item(DescriptionsItem::new("Name", "Alice"));
    let _element = d.view(&theme);
}

#[test]
fn test_descriptions_view_vertical_direction_renders() {
    let theme = Theme::element_light();
    let d = Descriptions::new()
        .with_direction(DescriptionsDirection::Vertical)
        .with_item(DescriptionsItem::new("Name", "Alice"));
    let _element = d.view(&theme);
}

#[test]
fn test_descriptions_view_with_column_renders() {
    let theme = Theme::element_light();
    let d = Descriptions::new()
        .with_column(2)
        .with_item(DescriptionsItem::new("A", "1"))
        .with_item(DescriptionsItem::new("B", "2"))
        .with_item(DescriptionsItem::new("C", "3"))
        .with_item(DescriptionsItem::new("D", "4"));
    let _element = d.view(&theme);
}

#[test]
fn test_descriptions_view_with_span_renders() {
    let theme = Theme::element_light();
    let d = Descriptions::new()
        .with_column(3)
        .with_item(DescriptionsItem::new("A", "1").with_span(2))
        .with_item(DescriptionsItem::new("B", "2"));
    let _element = d.view(&theme);
}

#[test]
fn test_descriptions_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let d = Descriptions::new().with_item(DescriptionsItem::new("Name", "Alice"));
    let _element = d.view(&theme);
}
