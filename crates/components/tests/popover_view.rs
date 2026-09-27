//! Popover view() 测试

use har_ui_components::popover::{Popover, PopoverMessage, PopoverPlacement, PopoverTrigger};
use har_ui_core::theme::Theme;
use iced::Element;
use iced::widget::text;

fn make_content<'a>() -> Element<'a, ()> {
    text("Trigger").into()
}

#[test]
fn test_popover_view_default_hidden_renders() {
    let theme = Theme::element_light();
    let p = Popover::new();
    let _element = p.view(&theme, make_content(), || ());
}

#[test]
fn test_popover_view_visible_renders() {
    let theme = Theme::element_light();
    let mut p = Popover::new().with_title("标题").with_content("内容");
    p.handle(PopoverMessage::Click);
    assert!(p.visible());
    let _element = p.view(&theme, make_content(), || ());
}

#[test]
fn test_popover_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let mut p = Popover::new().with_title("标题");
    p.handle(PopoverMessage::Click);
    let _element = p.view(&theme, make_content(), || ());
}

#[test]
fn test_popover_view_with_title_only_renders() {
    let theme = Theme::element_light();
    let mut p = Popover::new().with_title("只有标题");
    p.handle(PopoverMessage::Click);
    let _element = p.view(&theme, make_content(), || ());
}

#[test]
fn test_popover_view_with_content_only_renders() {
    let theme = Theme::element_light();
    let mut p = Popover::new().with_content("只有内容");
    p.handle(PopoverMessage::Click);
    let _element = p.view(&theme, make_content(), || ());
}

#[test]
fn test_popover_view_no_title_no_content_renders() {
    let theme = Theme::element_light();
    let mut p = Popover::new();
    p.handle(PopoverMessage::Click);
    let _element = p.view(&theme, make_content(), || ());
}

#[test]
fn test_popover_view_with_width_renders() {
    let theme = Theme::element_light();
    let mut p = Popover::new()
        .with_title("宽度")
        .with_content("200px")
        .with_width(200);
    p.handle(PopoverMessage::Click);
    let _element = p.view(&theme, make_content(), || ());
}

#[test]
fn test_popover_view_no_arrow_renders() {
    let theme = Theme::element_light();
    let mut p = Popover::new()
        .with_title("无箭头")
        .with_content("内容")
        .with_show_arrow(false);
    p.handle(PopoverMessage::Click);
    let _element = p.view(&theme, make_content(), || ());
}

#[test]
fn test_popover_view_disabled_renders() {
    let theme = Theme::element_light();
    let mut p = Popover::new().with_title("禁用").with_disabled(true);
    p.handle(PopoverMessage::Click);
    assert!(!p.visible());
    let _element = p.view(&theme, make_content(), || ());
}

#[test]
fn test_popover_view_hover_trigger_renders() {
    let theme = Theme::element_light();
    let mut p = Popover::new()
        .with_trigger(PopoverTrigger::Hover)
        .with_title("悬停");
    p.handle(PopoverMessage::MouseEnter);
    let _element = p.view(&theme, make_content(), || ());
}

#[test]
fn test_popover_view_focus_trigger_renders() {
    let theme = Theme::element_light();
    let mut p = Popover::new()
        .with_trigger(PopoverTrigger::Focus)
        .with_title("焦点");
    p.handle(PopoverMessage::Focus);
    let _element = p.view(&theme, make_content(), || ());
}

#[test]
fn test_popover_view_manual_trigger_renders() {
    let theme = Theme::element_light();
    let mut p = Popover::new()
        .with_trigger(PopoverTrigger::Manual)
        .with_title("手动");
    p.handle(PopoverMessage::Show);
    let _element = p.view(&theme, make_content(), || ());
}

#[test]
fn test_popover_view_placements_renders() {
    let theme = Theme::element_light();
    for placement in [
        PopoverPlacement::Top,
        PopoverPlacement::Bottom,
        PopoverPlacement::Left,
        PopoverPlacement::Right,
    ] {
        let mut p = Popover::new().with_title("x").with_placement(placement);
        p.handle(PopoverMessage::Click);
        let _element = p.view(&theme, make_content(), || ());
    }
}

#[test]
fn test_popover_view_custom_message_type_renders() {
    let theme = Theme::element_light();
    let mut p = Popover::new().with_title("自定义");
    p.handle(PopoverMessage::Click);
    #[derive(Clone, Debug)]
    enum AppMsg {
        Trigger,
    }
    let content: Element<AppMsg> = text("content").into();
    let _element = p.view(&theme, content, || AppMsg::Trigger);
}
