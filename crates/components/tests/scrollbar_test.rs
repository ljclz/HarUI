//! Scrollbar 滚动条 — 参考 Element Plus `<el-scrollbar>`。
//!
//! 覆盖：高度、最大高度、native、wrap-style、滚动位置、滚动事件。

use har_ui_components::scrollbar::{Scrollbar, ScrollbarMessage};

#[test]
fn test_scrollbar_default() {
    let s = Scrollbar::new();
    assert!(!s.always_visible());
    assert!(!s.native());
    assert_eq!(s.scroll_x(), 0);
    assert_eq!(s.scroll_y(), 0);
    assert_eq!(s.max_height(), None);
}

#[test]
fn test_scrollbar_with_height() {
    let s = Scrollbar::new().with_height(300);
    assert_eq!(s.height(), Some(300));
}

#[test]
fn test_scrollbar_with_max_height() {
    let s = Scrollbar::new().with_max_height(500);
    assert_eq!(s.max_height(), Some(500));
}

#[test]
fn test_scrollbar_always_visible() {
    let s = Scrollbar::new().with_always_visible(true);
    assert!(s.always_visible());
}

#[test]
fn test_scrollbar_native_mode() {
    let s = Scrollbar::new().with_native(true);
    assert!(s.native());
}

#[test]
fn test_scrollbar_set_scroll_position() {
    let mut s = Scrollbar::new();
    s.handle(ScrollbarMessage::ScrollTo { x: 100, y: 200 });
    assert_eq!(s.scroll_x(), 100);
    assert_eq!(s.scroll_y(), 200);
}

#[test]
fn test_scrollbar_scroll_by_delta() {
    let mut s = Scrollbar::new();
    s.handle(ScrollbarMessage::ScrollBy { dx: 50, dy: 30 });
    assert_eq!(s.scroll_x(), 50);
    assert_eq!(s.scroll_y(), 30);
    s.handle(ScrollbarMessage::ScrollBy { dx: 10, dy: -10 });
    assert_eq!(s.scroll_x(), 60);
    assert_eq!(s.scroll_y(), 20);
}

#[test]
fn test_scrollbar_clamp_negative() {
    let mut s = Scrollbar::new();
    s.handle(ScrollbarMessage::ScrollBy { dx: -100, dy: -100 });
    // 不能小于 0
    assert_eq!(s.scroll_x(), 0);
    assert_eq!(s.scroll_y(), 0);
}

#[test]
fn test_scrollbar_clamp_max() {
    let mut s = Scrollbar::new().with_max_scroll(1000, 800);
    s.handle(ScrollbarMessage::ScrollTo { x: 2000, y: 1500 });
    // 钳制到最大
    assert_eq!(s.scroll_x(), 1000);
    assert_eq!(s.scroll_y(), 800);
}

#[test]
fn test_scrollbar_reset() {
    let mut s = Scrollbar::new();
    s.handle(ScrollbarMessage::ScrollTo { x: 100, y: 200 });
    s.handle(ScrollbarMessage::Reset);
    assert_eq!(s.scroll_x(), 0);
    assert_eq!(s.scroll_y(), 0);
}

#[test]
fn test_scrollbar_set_max_scroll() {
    let mut s = Scrollbar::new();
    s.set_max_scroll(500, 400);
    assert_eq!(s.max_scroll_x(), 500);
    assert_eq!(s.max_scroll_y(), 400);
}
