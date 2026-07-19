//! Space view() 测试 — TDD RED 阶段

use har_ui_components::space::{Space, SpaceAlignment, SpaceDirection, SpaceMessage};
use har_ui_core::theme::Theme;
use iced::widget::text;
use iced::Element;

fn make_children<'a>() -> Vec<Element<'a, ()>> {
    vec![
        text("A").into(),
        text("B").into(),
        text("C").into(),
    ]
}

#[test]
fn test_space_view_empty_renders() {
    let theme = Theme::element_light();
    let s = Space::new();
    let empty: Vec<Element<()>> = vec![];
    let _element = s.view(&theme, empty);
}

#[test]
fn test_space_view_horizontal_renders() {
    let theme = Theme::element_light();
    let s = Space::new().with_direction(SpaceDirection::Horizontal);
    let _element = s.view(&theme, make_children());
}

#[test]
fn test_space_view_vertical_renders() {
    let theme = Theme::element_light();
    let s = Space::new().with_direction(SpaceDirection::Vertical);
    let _element = s.view(&theme, make_children());
}

#[test]
fn test_space_view_with_custom_size_renders() {
    let theme = Theme::element_light();
    let s = Space::new().with_size(24);
    let _element = s.view(&theme, make_children());
}

#[test]
fn test_space_view_with_wrap_renders() {
    let theme = Theme::element_light();
    let s = Space::new().with_wrap(true);
    let _element = s.view(&theme, make_children());
}

#[test]
fn test_space_view_with_fill_renders() {
    let theme = Theme::element_light();
    let s = Space::new().with_fill(true);
    let _element = s.view(&theme, make_children());
}

#[test]
fn test_space_view_alignment_start_renders() {
    let theme = Theme::element_light();
    let s = Space::new().with_alignment(SpaceAlignment::Start);
    let _element = s.view(&theme, make_children());
}

#[test]
fn test_space_view_alignment_end_renders() {
    let theme = Theme::element_light();
    let s = Space::new().with_alignment(SpaceAlignment::End);
    let _element = s.view(&theme, make_children());
}

#[test]
fn test_space_view_with_managed_items_renders() {
    let theme = Theme::element_light();
    let mut s = Space::new();
    s.handle(SpaceMessage::AddItem("X".to_string()));
    s.handle(SpaceMessage::AddItem("Y".to_string()));
    let _element = s.view(&theme, make_children());
}

#[test]
fn test_space_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let s = Space::new().with_direction(SpaceDirection::Vertical);
    let _element = s.view(&theme, make_children());
}
