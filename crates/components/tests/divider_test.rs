//! Divider 分割线 — 参考 Element Plus `<el-divider>`。
//!
//! 覆盖：方向、内容位置、样式、自定义内容。

use har_ui_components::divider::{Divider, DividerContentPosition, DividerDirection};

#[test]
fn test_divider_default() {
    let d = Divider::new();
    assert_eq!(d.direction(), DividerDirection::Horizontal);
    assert_eq!(d.content_position(), DividerContentPosition::Center);
    assert_eq!(d.text(), None);
}

#[test]
fn test_divider_with_text() {
    let d = Divider::new().with_text("分隔内容");
    assert_eq!(d.text(), Some("分隔内容"));
}

#[test]
fn test_divider_vertical_direction() {
    let d = Divider::new().with_direction(DividerDirection::Vertical);
    assert_eq!(d.direction(), DividerDirection::Vertical);
}

#[test]
fn test_divider_content_left() {
    let d = Divider::new()
        .with_text("标题")
        .with_content_position(DividerContentPosition::Left);
    assert_eq!(d.content_position(), DividerContentPosition::Left);
}

#[test]
fn test_divider_content_right() {
    let d = Divider::new().with_content_position(DividerContentPosition::Right);
    assert_eq!(d.content_position(), DividerContentPosition::Right);
}

#[test]
fn test_divider_border_style_dashed() {
    let d = Divider::new().with_border_dashed(true);
    assert!(d.border_dashed());
}

#[test]
fn test_divider_vertical_ignores_text() {
    // 垂直方向不显示文本（按 Element Plus 行为）
    let d = Divider::new()
        .with_text("ignored")
        .with_direction(DividerDirection::Vertical);
    // text 仍可访问，但渲染层应忽略
    assert_eq!(d.text(), Some("ignored"));
    assert_eq!(d.direction(), DividerDirection::Vertical);
}

#[test]
fn test_divider_clear_text() {
    let mut d = Divider::new().with_text("a");
    d.clear_text();
    assert_eq!(d.text(), None);
}
