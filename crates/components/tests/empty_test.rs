//! Empty 空状态 — 参考 Element Plus `<el-empty>`。
//!
//! 覆盖：默认空状态、自定义 image/description、尺寸。

use har_ui_components::empty::{Empty, EmptyImage, EmptySize};

#[test]
fn test_empty_default() {
    let e = Empty::new();
    assert_eq!(e.description(), "暂无数据");
    assert_eq!(e.image(), EmptyImage::Default);
    assert_eq!(e.size(), EmptySize::Normal);
}

#[test]
fn test_empty_custom_description() {
    let e = Empty::new().with_description("没有找到匹配的商品");
    assert_eq!(e.description(), "没有找到匹配的商品");
}

#[test]
fn test_empty_custom_image_url() {
    let e = Empty::new().with_image_url("/img/empty.png");
    assert_eq!(e.image(), EmptyImage::Custom);
    assert_eq!(e.image_url(), Some("/img/empty.png"));
}

#[test]
fn test_empty_size_small() {
    let e = Empty::new().with_size(EmptySize::Small);
    assert_eq!(e.size(), EmptySize::Small);
}

#[test]
fn test_empty_image_variants() {
    let e1 = Empty::new().with_image(EmptyImage::Default);
    assert_eq!(e1.image(), EmptyImage::Default);

    let e2 = Empty::new().with_image(EmptyImage::Error);
    assert_eq!(e2.image(), EmptyImage::Error);

    let e3 = Empty::new().with_image(EmptyImage::Network);
    assert_eq!(e3.image(), EmptyImage::Network);
}

#[test]
fn test_empty_safe_render() {
    // 空对象安全渲染（不 panic）
    let e = Empty::new();
    assert!(!e.description().is_empty());
}

#[test]
fn test_empty_with_extra_slot() {
    // 支持自定义底部内容标记（如按钮）
    let e = Empty::new().with_has_extra(true);
    assert!(e.has_extra());
}
