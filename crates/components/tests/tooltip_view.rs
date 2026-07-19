//! Tooltip view() 测试

use har_ui_components::tooltip::{
    Tooltip, TooltipEffect, TooltipMessage, TooltipPlacement, TooltipTrigger,
};
use har_ui_core::theme::Theme;
use iced::widget::text;
use iced::Element;

fn make_content<'a>() -> Element<'a, ()> {
    text("Hover me").into()
}

#[test]
fn test_tooltip_view_hidden_renders() {
    let theme = Theme::element_light();
    let t = Tooltip::new("提示");
    let _element = t.view(&theme, make_content());
}

#[test]
fn test_tooltip_view_visible_renders() {
    let theme = Theme::element_light();
    let mut t = Tooltip::new("提示");
    t.handle(TooltipMessage::MouseEnter);
    assert!(t.visible());
    let _element = t.view(&theme, make_content());
}

#[test]
fn test_tooltip_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let mut t = Tooltip::new("提示");
    t.handle(TooltipMessage::MouseEnter);
    let _element = t.view(&theme, make_content());
}

#[test]
fn test_tooltip_view_disabled_renders() {
    let theme = Theme::element_light();
    let mut t = Tooltip::new("提示").with_disabled(true);
    t.handle(TooltipMessage::MouseEnter);
    assert!(!t.visible());
    let _element = t.view(&theme, make_content());
}

#[test]
fn test_tooltip_view_light_effect_renders() {
    let theme = Theme::element_light();
    let mut t = Tooltip::new("提示").with_effect(TooltipEffect::Light);
    t.handle(TooltipMessage::MouseEnter);
    let _element = t.view(&theme, make_content());
}

#[test]
fn test_tooltip_view_dark_effect_renders() {
    let theme = Theme::element_light();
    let mut t = Tooltip::new("提示").with_effect(TooltipEffect::Dark);
    t.handle(TooltipMessage::MouseEnter);
    let _element = t.view(&theme, make_content());
}

#[test]
fn test_tooltip_view_no_arrow_renders() {
    let theme = Theme::element_light();
    let mut t = Tooltip::new("提示").with_show_arrow(false);
    t.handle(TooltipMessage::MouseEnter);
    let _element = t.view(&theme, make_content());
}

#[test]
fn test_tooltip_view_click_trigger_renders() {
    let theme = Theme::element_light();
    let mut t = Tooltip::new("提示").with_trigger(TooltipTrigger::Click);
    t.handle(TooltipMessage::Click);
    let _element = t.view(&theme, make_content());
}

#[test]
fn test_tooltip_view_focus_trigger_renders() {
    let theme = Theme::element_light();
    let mut t = Tooltip::new("提示").with_trigger(TooltipTrigger::Focus);
    t.handle(TooltipMessage::Focus);
    let _element = t.view(&theme, make_content());
}

#[test]
fn test_tooltip_view_manual_trigger_renders() {
    let theme = Theme::element_light();
    let mut t = Tooltip::new("提示").with_trigger(TooltipTrigger::Manual);
    t.handle(TooltipMessage::Show);
    let _element = t.view(&theme, make_content());
}

#[test]
fn test_tooltip_view_placements_renders() {
    let theme = Theme::element_light();
    let placements = [
        TooltipPlacement::Top,
        TooltipPlacement::Bottom,
        TooltipPlacement::Left,
        TooltipPlacement::Right,
    ];
    for p in placements {
        let mut t = Tooltip::new("x").with_placement(p);
        t.handle(TooltipMessage::MouseEnter);
        let _element = t.view(&theme, make_content());
    }
}

#[test]
fn test_tooltip_view_hide_after_renders() {
    let theme = Theme::element_light();
    let mut t = Tooltip::new("提示").with_hide_after(500);
    t.handle(TooltipMessage::MouseEnter);
    t.tick(600);
    assert!(!t.visible());
    let _element = t.view(&theme, make_content());
}

#[test]
fn test_tooltip_view_custom_message_type_renders() {
    let theme = Theme::element_light();
    let mut t = Tooltip::new("提示");
    t.handle(TooltipMessage::MouseEnter);
    #[derive(Clone, Debug)]
    enum AppMsg {}
    let content: Element<AppMsg> = text("custom").into();
    let _element = t.view(&theme, content);
}
