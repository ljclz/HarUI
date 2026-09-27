//! R.2.P1.5 Skeleton view() 测试 — TDD RED 阶段

use har_ui_components::skeleton::{Skeleton, SkeletonItem};
use har_ui_core::theme::Theme;

fn make_loading() -> Skeleton {
    Skeleton::new().with_loading(true)
}

#[test]
fn test_skeleton_view_empty_renders() {
    let theme = Theme::element_light();
    let s = make_loading();
    let _element = s.view(&theme);
}

#[test]
fn test_skeleton_view_with_paragraph_renders() {
    let theme = Theme::element_light();
    let s = make_loading().with_item(SkeletonItem::paragraph());
    let _element = s.view(&theme);
}

#[test]
fn test_skeleton_view_with_title_renders() {
    let theme = Theme::element_light();
    let s = make_loading().with_item(SkeletonItem::title());
    let _element = s.view(&theme);
}

#[test]
fn test_skeleton_view_with_avatar_renders() {
    let theme = Theme::element_light();
    let s = make_loading().with_item(SkeletonItem::avatar());
    let _element = s.view(&theme);
}

#[test]
fn test_skeleton_view_with_image_renders() {
    let theme = Theme::element_light();
    let s = make_loading().with_item(SkeletonItem::image());
    let _element = s.view(&theme);
}

#[test]
fn test_skeleton_view_multiple_items_renders() {
    let theme = Theme::element_light();
    let s = make_loading()
        .with_item(SkeletonItem::title())
        .with_item(SkeletonItem::paragraph())
        .with_item(SkeletonItem::paragraph());
    let _element = s.view(&theme);
}

#[test]
fn test_skeleton_view_animated_renders() {
    let theme = Theme::element_light();
    let s = make_loading()
        .with_item(SkeletonItem::paragraph())
        .with_animated(true);
    let _element = s.view(&theme);
}

#[test]
fn test_skeleton_view_count_renders() {
    let theme = Theme::element_light();
    let s = make_loading()
        .with_item(SkeletonItem::paragraph())
        .with_count(3);
    let _element = s.view(&theme);
}

#[test]
fn test_skeleton_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let s = make_loading().with_item(SkeletonItem::paragraph());
    let _element = s.view(&theme);
}
