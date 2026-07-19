//! Backtop 回到顶部 — 参考 Element Plus `<el-backtop>`。
//!
//! 覆盖：visibility_height 阈值、scroll_y 跟踪、Click 回顶、right/bottom、smooth。

use har_ui_components::backtop::{Backtop, BacktopMessage};

#[test]
fn test_backtop_default() {
    let b = Backtop::new();
    assert_eq!(b.visibility_height(), 200);
    assert_eq!(b.right(), 40);
    assert_eq!(b.bottom(), 40);
    assert_eq!(b.scroll_y(), 0);
    assert!(!b.smooth());
    assert!(!b.visible());
}

#[test]
fn test_backtop_with_visibility_height() {
    let b = Backtop::new().with_visibility_height(100);
    assert_eq!(b.visibility_height(), 100);
}

#[test]
fn test_backtop_with_right_bottom() {
    let b = Backtop::new().with_right(80).with_bottom(60);
    assert_eq!(b.right(), 80);
    assert_eq!(b.bottom(), 60);
}

#[test]
fn test_backtop_with_smooth() {
    let b = Backtop::new().with_smooth(true);
    assert!(b.smooth());
}

#[test]
fn test_backtop_scroll_above_threshold_visible() {
    let mut b = Backtop::new().with_visibility_height(200);
    b.handle(BacktopMessage::Scroll(300));
    assert_eq!(b.scroll_y(), 300);
    assert!(b.visible());
}

#[test]
fn test_backtop_scroll_below_threshold_invisible() {
    let mut b = Backtop::new().with_visibility_height(200);
    b.handle(BacktopMessage::Scroll(100));
    assert!(!b.visible());
}

#[test]
fn test_backtop_scroll_equal_threshold_invisible() {
    let mut b = Backtop::new().with_visibility_height(200);
    b.handle(BacktopMessage::Scroll(200));
    // 等于阈值不显示，必须严格大于
    assert!(!b.visible());
}

#[test]
fn test_backtop_click_resets_scroll() {
    let mut b = Backtop::new().with_visibility_height(100);
    b.handle(BacktopMessage::Scroll(500));
    assert!(b.visible());
    b.handle(BacktopMessage::Click);
    assert_eq!(b.scroll_y(), 0);
    assert!(!b.visible());
}

#[test]
fn test_backtop_set_visibility_height_message() {
    let mut b = Backtop::new();
    b.handle(BacktopMessage::SetVisibilityHeight(50));
    assert_eq!(b.visibility_height(), 50);
}

#[test]
fn test_backtop_set_smooth_message() {
    let mut b = Backtop::new();
    b.handle(BacktopMessage::SetSmooth(true));
    assert!(b.smooth());
}

#[test]
fn test_backtop_visibility_changes_with_scroll() {
    let mut b = Backtop::new().with_visibility_height(200);
    b.handle(BacktopMessage::Scroll(100));
    assert!(!b.visible());
    b.handle(BacktopMessage::Scroll(300));
    assert!(b.visible());
    b.handle(BacktopMessage::Scroll(150));
    assert!(!b.visible());
}

#[test]
fn test_backtop_scroll_zero_keeps_invisible() {
    let mut b = Backtop::new();
    b.handle(BacktopMessage::Scroll(0));
    assert_eq!(b.scroll_y(), 0);
    assert!(!b.visible());
}
