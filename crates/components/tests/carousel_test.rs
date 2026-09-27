//! Carousel 走马灯 — 参考 Element Plus `<el-carousel>`。
//!
//! 覆盖：基础轮播、自动播放、间隔、循环、方向、指示器、箭头。

use har_ui_components::carousel::{Carousel, CarouselDirection, CarouselMessage};

#[test]
fn test_carousel_empty() {
    let c = Carousel::new();
    assert_eq!(c.slides().len(), 0);
    assert_eq!(c.current_index(), 0);
}

#[test]
fn test_carousel_with_slides() {
    let c = Carousel::new()
        .with_slide("slide-1")
        .with_slide("slide-2")
        .with_slide("slide-3");
    assert_eq!(c.slides().len(), 3);
    assert_eq!(c.current_index(), 0);
}

#[test]
fn test_carousel_next_wrap_around() {
    let mut c = Carousel::new()
        .with_slide("a")
        .with_slide("b")
        .with_slide("c");
    c.handle(CarouselMessage::Next);
    assert_eq!(c.current_index(), 1);
    c.handle(CarouselMessage::Next);
    assert_eq!(c.current_index(), 2);
    c.handle(CarouselMessage::Next);
    // 循环回到 0
    assert_eq!(c.current_index(), 0);
}

#[test]
fn test_carousel_prev_wrap_around() {
    let mut c = Carousel::new()
        .with_slide("a")
        .with_slide("b")
        .with_slide("c");
    c.handle(CarouselMessage::Prev);
    // 从 0 回到末尾
    assert_eq!(c.current_index(), 2);
}

#[test]
fn test_carousel_jump_to_index() {
    let mut c = Carousel::new()
        .with_slide("a")
        .with_slide("b")
        .with_slide("c");
    c.handle(CarouselMessage::JumpTo(2));
    assert_eq!(c.current_index(), 2);
    c.handle(CarouselMessage::JumpTo(99));
    // 越界 → 钳制到最后
    assert_eq!(c.current_index(), 2);
}

#[test]
fn test_carousel_autoplay_flag() {
    let c = Carousel::new().with_autoplay(true);
    assert!(c.autoplay());
    let c2 = Carousel::new();
    assert!(!c2.autoplay());
}

#[test]
fn test_carousel_interval() {
    let c = Carousel::new().with_interval(5000);
    assert_eq!(c.interval(), 5000);
}

#[test]
fn test_carousel_direction_vertical() {
    let c = Carousel::new().with_direction(CarouselDirection::Vertical);
    assert_eq!(c.direction(), CarouselDirection::Vertical);
}

#[test]
fn test_carousel_loop_toggle() {
    let mut c = Carousel::new()
        .with_slide("a")
        .with_slide("b")
        .with_slide("c")
        .with_loop(false);
    c.handle(CarouselMessage::Next);
    c.handle(CarouselMessage::Next);
    c.handle(CarouselMessage::Next);
    // 非循环下，到末尾后不再前进
    assert_eq!(c.current_index(), 2);
}

#[test]
fn test_carousel_tick_auto_advance() {
    let mut c = Carousel::new()
        .with_slide("a")
        .with_slide("b")
        .with_autoplay(true)
        .with_interval(3000);
    // 模拟 3000ms 过去 → 自动前进
    c.tick(3000);
    assert_eq!(c.current_index(), 1);
    // 再过 1500ms 不前进
    c.tick(1500);
    assert_eq!(c.current_index(), 1);
    // 再过 1500ms 累计 3000ms → 前进
    c.tick(1500);
    assert_eq!(c.current_index(), 0); // 循环回 0
}

#[test]
fn test_carousel_indicator_visible() {
    let c = Carousel::new().with_slide("a").with_slide("b");
    assert!(c.show_indicator());
    let c2 = Carousel::new().with_show_indicator(false);
    assert!(!c2.show_indicator());
}

#[test]
fn test_carousel_arrow_visible() {
    let c = Carousel::new().with_show_arrow(false);
    assert!(!c.show_arrow());
}

#[test]
fn test_carousel_pause_on_hover() {
    let mut c = Carousel::new()
        .with_slide("a")
        .with_slide("b")
        .with_autoplay(true)
        .with_interval(1000)
        .with_pause_on_hover(true);
    c.set_hovered(true);
    c.tick(1000);
    // hover 时不前进
    assert_eq!(c.current_index(), 0);
    c.set_hovered(false);
    c.tick(1000);
    assert_eq!(c.current_index(), 1);
}
