//! Skeleton 骨架屏 — 参考 Element Plus `<el-skeleton>`。
//!
//! 覆盖：段落/标题/头像/图片 4 种样式、animated、count、loading 切换。

use har_ui_components::skeleton::{Skeleton, SkeletonItem, SkeletonVariant};

// ---------- 4 种样式 ----------

#[test]
fn test_skeleton_paragraph_variant() {
    let s = Skeleton::new().with_item(SkeletonItem::paragraph());
    assert_eq!(s.items().len(), 1);
    assert_eq!(s.items()[0].variant(), SkeletonVariant::Paragraph);
}

#[test]
fn test_skeleton_title_variant() {
    let s = Skeleton::new().with_item(SkeletonItem::title());
    assert_eq!(s.items()[0].variant(), SkeletonVariant::Title);
}

#[test]
fn test_skeleton_avatar_variant() {
    let s = Skeleton::new().with_item(SkeletonItem::avatar());
    assert_eq!(s.items()[0].variant(), SkeletonVariant::Avatar);
}

#[test]
fn test_skeleton_image_variant() {
    let s = Skeleton::new().with_item(SkeletonItem::image());
    assert_eq!(s.items()[0].variant(), SkeletonVariant::Image);
}

// ---------- animated / count ----------

#[test]
fn test_skeleton_animated_flag() {
    let s = Skeleton::new().with_animated(true);
    assert!(s.animated());

    let s2 = Skeleton::new();
    assert!(!s2.animated()); // 默认不动画
}

#[test]
fn test_skeleton_count_repeats_items() {
    // count=3 把每项重复 3 次
    let template = vec![SkeletonItem::paragraph(), SkeletonItem::title()];
    let s = Skeleton::new()
        .with_template(template)
        .with_count(3);
    // 2 项 × 3 次 = 6 项
    assert_eq!(s.items().len(), 6);
}

// ---------- loading 切换 ----------

#[test]
fn test_skeleton_loading_toggle() {
    let mut s = Skeleton::new()
        .with_item(SkeletonItem::paragraph())
        .with_loading(true);
    assert!(s.loading());

    // 模拟加载完成
    s.set_loading(false);
    assert!(!s.loading());
}

// ---------- 边界 ----------

#[test]
fn test_skeleton_empty_items_safe() {
    let s = Skeleton::new();
    assert!(s.items().is_empty()); // 无 item 不 panic
}

#[test]
fn test_skeleton_count_zero_safe() {
    let s = Skeleton::new()
        .with_template(vec![SkeletonItem::paragraph()])
        .with_count(0);
    assert_eq!(s.items().len(), 0); // count=0 不重复
}

#[test]
fn test_skeleton_custom_dimensions() {
    // 自定义段落宽度（百分比或像素）
    let item = SkeletonItem::paragraph().with_width("200px");
    assert_eq!(item.width(), Some("200px"));
    let item2 = SkeletonItem::image().with_height("100px");
    assert_eq!(item2.height(), Some("100px"));
}
