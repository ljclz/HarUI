//! Popconfirm view() 测试

use har_ui_components::popconfirm::{
    Popconfirm, PopconfirmMessage, PopconfirmPlacement, PopconfirmTrigger,
};
use har_ui_core::theme::Theme;
use iced::Element;
use iced::widget::text;

fn make_content<'a>() -> Element<'a, ()> {
    text("Delete").into()
}

#[test]
fn test_popconfirm_view_hidden_renders() {
    let theme = Theme::element_light();
    let p = Popconfirm::new("确认删除?");
    let _element = p.view(&theme, make_content(), || (), || ());
}

#[test]
fn test_popconfirm_view_visible_renders() {
    let theme = Theme::element_light();
    let mut p = Popconfirm::new("确认删除?");
    p.handle(PopconfirmMessage::Click);
    assert!(p.visible());
    let _element = p.view(&theme, make_content(), || (), || ());
}

#[test]
fn test_popconfirm_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let mut p = Popconfirm::new("确认?");
    p.handle(PopconfirmMessage::Click);
    let _element = p.view(&theme, make_content(), || (), || ());
}

#[test]
fn test_popconfirm_view_with_width_renders() {
    let theme = Theme::element_light();
    let mut p = Popconfirm::new("确认?").with_width(200);
    p.handle(PopconfirmMessage::Click);
    let _element = p.view(&theme, make_content(), || (), || ());
}

#[test]
fn test_popconfirm_view_no_arrow_renders() {
    let theme = Theme::element_light();
    let mut p = Popconfirm::new("确认?").with_show_arrow(false);
    p.handle(PopconfirmMessage::Click);
    let _element = p.view(&theme, make_content(), || (), || ());
}

#[test]
fn test_popconfirm_view_disabled_renders() {
    let theme = Theme::element_light();
    let mut p = Popconfirm::new("确认?").with_disabled(true);
    p.handle(PopconfirmMessage::Click);
    assert!(!p.visible());
    let _element = p.view(&theme, make_content(), || (), || ());
}

#[test]
fn test_popconfirm_view_custom_button_text_renders() {
    let theme = Theme::element_light();
    let mut p = Popconfirm::new("确认?")
        .with_confirm_text("Yes")
        .with_cancel_text("No");
    p.handle(PopconfirmMessage::Click);
    let _element = p.view(&theme, make_content(), || (), || ());
}

#[test]
fn test_popconfirm_view_hover_trigger_renders() {
    let theme = Theme::element_light();
    let mut p = Popconfirm::new("确认?").with_trigger(PopconfirmTrigger::Hover);
    p.handle(PopconfirmMessage::MouseEnter);
    let _element = p.view(&theme, make_content(), || (), || ());
}

#[test]
fn test_popconfirm_view_focus_trigger_renders() {
    let theme = Theme::element_light();
    let mut p = Popconfirm::new("确认?").with_trigger(PopconfirmTrigger::Focus);
    p.handle(PopconfirmMessage::Focus);
    let _element = p.view(&theme, make_content(), || (), || ());
}

#[test]
fn test_popconfirm_view_manual_trigger_renders() {
    let theme = Theme::element_light();
    let mut p = Popconfirm::new("确认?").with_trigger(PopconfirmTrigger::Manual);
    p.handle(PopconfirmMessage::Show);
    let _element = p.view(&theme, make_content(), || (), || ());
}

#[test]
fn test_popconfirm_view_placements_renders() {
    let theme = Theme::element_light();
    for placement in [
        PopconfirmPlacement::Top,
        PopconfirmPlacement::Bottom,
        PopconfirmPlacement::Left,
        PopconfirmPlacement::Right,
    ] {
        let mut p = Popconfirm::new("x").with_placement(placement);
        p.handle(PopconfirmMessage::Click);
        let _element = p.view(&theme, make_content(), || (), || ());
    }
}

#[test]
fn test_popconfirm_view_after_confirm_renders() {
    let theme = Theme::element_light();
    let mut p = Popconfirm::new("确认?");
    p.handle(PopconfirmMessage::Click);
    p.handle(PopconfirmMessage::Confirm);
    assert!(!p.visible());
    let _element = p.view(&theme, make_content(), || (), || ());
}

#[test]
fn test_popconfirm_view_custom_message_type_renders() {
    let theme = Theme::element_light();
    let mut p = Popconfirm::new("确认?");
    p.handle(PopconfirmMessage::Click);
    #[derive(Clone, Debug)]
    enum AppMsg {
        Confirm,
        Cancel,
    }
    let content: Element<AppMsg> = text("content").into();
    let _element = p.view(&theme, content, || AppMsg::Confirm, || AppMsg::Cancel);
}
