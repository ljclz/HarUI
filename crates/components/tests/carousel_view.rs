//! Carousel view() 测试 — TDD RED 阶段

use har_ui_components::carousel::{Carousel, CarouselDirection, CarouselMessage};
use har_ui_core::theme::Theme;

#[test]
fn test_carousel_view_default_renders() {
    let theme = Theme::element_light();
    let c = Carousel::new();
    let _element = c.view(&theme, || (), || ());
}

#[test]
fn test_carousel_view_with_slides_renders() {
    let theme = Theme::element_light();
    let c = Carousel::new()
        .with_slide("slide-1")
        .with_slide("slide-2")
        .with_slide("slide-3");
    let _element = c.view(&theme, || (), || ());
}

#[test]
fn test_carousel_view_after_next_renders() {
    let theme = Theme::element_light();
    let mut c = Carousel::new().with_slide("a").with_slide("b").with_slide("c");
    c.handle(CarouselMessage::Next);
    let _element = c.view(&theme, || (), || ());
}

#[test]
fn test_carousel_view_after_prev_renders() {
    let theme = Theme::element_light();
    let mut c = Carousel::new().with_slide("a").with_slide("b").with_slide("c");
    c.handle(CarouselMessage::Prev);
    let _element = c.view(&theme, || (), || ());
}

#[test]
fn test_carousel_view_hide_indicator_renders() {
    let theme = Theme::element_light();
    let c = Carousel::new()
        .with_slide("a")
        .with_slide("b")
        .with_show_indicator(false);
    let _element = c.view(&theme, || (), || ());
}

#[test]
fn test_carousel_view_hide_arrow_renders() {
    let theme = Theme::element_light();
    let c = Carousel::new()
        .with_slide("a")
        .with_slide("b")
        .with_show_arrow(false);
    let _element = c.view(&theme, || (), || ());
}

#[test]
fn test_carousel_view_vertical_direction_renders() {
    let theme = Theme::element_light();
    let c = Carousel::new()
        .with_slide("a")
        .with_slide("b")
        .with_direction(CarouselDirection::Vertical);
    let _element = c.view(&theme, || (), || ());
}

#[test]
fn test_carousel_view_no_loop_renders() {
    let theme = Theme::element_light();
    let c = Carousel::new()
        .with_slide("a")
        .with_slide("b")
        .with_loop(false);
    let _element = c.view(&theme, || (), || ());
}

#[test]
fn test_carousel_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let c = Carousel::new().with_slide("a").with_slide("b");
    let _element = c.view(&theme, || (), || ());
}

#[test]
fn test_carousel_view_custom_message_type() {
    let theme = Theme::element_light();
    let c = Carousel::new().with_slide("a");
    #[derive(Clone, Debug)]
    #[allow(dead_code)]
    enum AppMsg {
        Prev,
        Next,
    }
    let _element = c.view(&theme, || AppMsg::Prev, || AppMsg::Next);
}
