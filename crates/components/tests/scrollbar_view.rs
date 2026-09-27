//! Scrollbar view() 测试

use har_ui_components::scrollbar::{Scrollbar, ScrollbarMessage};
use har_ui_core::theme::Theme;
use iced::Element;
use iced::widget::text;

fn make_content<'a>() -> Element<'a, ()> {
    iced::widget::Column::new()
        .push(text("line 1"))
        .push(text("line 2"))
        .push(text("line 3"))
        .push(text("line 4"))
        .push(text("line 5"))
        .into()
}

#[test]
fn test_scrollbar_view_default_renders() {
    let theme = Theme::element_light();
    let s = Scrollbar::new();
    let _element = s.view(&theme, make_content());
}

#[test]
fn test_scrollbar_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let s = Scrollbar::new();
    let _element = s.view(&theme, make_content());
}

#[test]
fn test_scrollbar_view_with_height_renders() {
    let theme = Theme::element_light();
    let s = Scrollbar::new().with_height(200);
    let _element = s.view(&theme, make_content());
}

#[test]
fn test_scrollbar_view_with_max_height_renders() {
    let theme = Theme::element_light();
    let s = Scrollbar::new().with_max_height(150);
    let _element = s.view(&theme, make_content());
}

#[test]
fn test_scrollbar_view_with_height_and_max_height_renders() {
    let theme = Theme::element_light();
    let s = Scrollbar::new().with_height(200).with_max_height(300);
    let _element = s.view(&theme, make_content());
}

#[test]
fn test_scrollbar_view_always_visible_renders() {
    let theme = Theme::element_light();
    let s = Scrollbar::new().with_always_visible(true);
    let _element = s.view(&theme, make_content());
}

#[test]
fn test_scrollbar_view_native_renders() {
    let theme = Theme::element_light();
    let s = Scrollbar::new().with_native(true);
    let _element = s.view(&theme, make_content());
}

#[test]
fn test_scrollbar_view_after_scroll_to_renders() {
    let theme = Theme::element_light();
    let mut s = Scrollbar::new().with_max_scroll(1000, 1000);
    s.handle(ScrollbarMessage::ScrollTo { x: 100, y: 200 });
    assert_eq!(s.scroll_x(), 100);
    assert_eq!(s.scroll_y(), 200);
    let _element = s.view(&theme, make_content());
}

#[test]
fn test_scrollbar_view_after_scroll_by_renders() {
    let theme = Theme::element_light();
    let mut s = Scrollbar::new().with_max_scroll(1000, 1000);
    s.handle(ScrollbarMessage::ScrollBy { dx: 50, dy: 50 });
    let _element = s.view(&theme, make_content());
}

#[test]
fn test_scrollbar_view_after_reset_renders() {
    let theme = Theme::element_light();
    let mut s = Scrollbar::new().with_max_scroll(1000, 1000);
    s.handle(ScrollbarMessage::ScrollTo { x: 100, y: 100 });
    s.handle(ScrollbarMessage::Reset);
    assert_eq!(s.scroll_x(), 0);
    assert_eq!(s.scroll_y(), 0);
    let _element = s.view(&theme, make_content());
}

#[test]
fn test_scrollbar_view_clamp_position_renders() {
    let theme = Theme::element_light();
    let mut s = Scrollbar::new().with_max_scroll(500, 500);
    s.handle(ScrollbarMessage::ScrollTo { x: 1000, y: 1000 });
    assert_eq!(s.scroll_x(), 500);
    assert_eq!(s.scroll_y(), 500);
    let _element = s.view(&theme, make_content());
}

#[test]
fn test_scrollbar_view_custom_message_type_renders() {
    let theme = Theme::element_light();
    let s = Scrollbar::new().with_height(200);
    #[derive(Clone, Debug)]
    enum AppMsg {}
    let content: Element<AppMsg> = text("content").into();
    let _element = s.view(&theme, content);
}

#[test]
fn test_scrollbar_view_set_max_scroll_renders() {
    let theme = Theme::element_light();
    let mut s = Scrollbar::new().with_height(200);
    s.set_max_scroll(800, 800);
    let _element = s.view(&theme, make_content());
}
