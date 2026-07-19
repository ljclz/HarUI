//! Loading view() 测试

use har_ui_components::loading::{Loading, LoadingMessage};
use har_ui_core::theme::Theme;
use iced::widget::text;
use iced::Element;

fn make_content<'a>() -> Element<'a, ()> {
    text("Content").into()
}

#[test]
fn test_loading_view_inactive_renders() {
    let theme = Theme::element_light();
    let l = Loading::new();
    let _element = l.view(&theme, make_content());
}

#[test]
fn test_loading_view_active_renders() {
    let theme = Theme::element_light();
    let mut l = Loading::new();
    l.handle(LoadingMessage::Start);
    let _element = l.view(&theme, make_content());
}

#[test]
fn test_loading_view_dark_theme_active_renders() {
    let theme = Theme::element_dark();
    let mut l = Loading::new();
    l.handle(LoadingMessage::Start);
    let _element = l.view(&theme, make_content());
}

#[test]
fn test_loading_view_with_text_renders() {
    let theme = Theme::element_light();
    let mut l = Loading::new().with_text("加载中...");
    l.handle(LoadingMessage::Start);
    let _element = l.view(&theme, make_content());
}

#[test]
fn test_loading_view_start_with_text_message_renders() {
    let theme = Theme::element_light();
    let mut l = Loading::new();
    l.handle(LoadingMessage::StartWithText("加载中".to_string()));
    let _element = l.view(&theme, make_content());
}

#[test]
fn test_loading_view_fullscreen_renders() {
    let theme = Theme::element_light();
    let mut l = Loading::new().with_fullscreen(true);
    l.handle(LoadingMessage::Start);
    let _element = l.view(&theme, make_content());
}

#[test]
fn test_loading_view_lock_renders() {
    let theme = Theme::element_light();
    let mut l = Loading::new().with_lock(true);
    l.handle(LoadingMessage::Start);
    let _element = l.view(&theme, make_content());
}

#[test]
fn test_loading_view_toggle_renders() {
    let theme = Theme::element_light();
    let mut l = Loading::new();
    l.handle(LoadingMessage::Toggle);
    let _element = l.view(&theme, make_content());
    drop(_element);
    l.handle(LoadingMessage::Toggle);
    let _element = l.view(&theme, make_content());
}

#[test]
fn test_loading_view_after_tick_renders() {
    let theme = Theme::element_light();
    let mut l = Loading::new();
    l.handle(LoadingMessage::Start);
    l.tick(500);
    let _element = l.view(&theme, make_content());
}

#[test]
fn test_loading_view_after_stop_renders() {
    let theme = Theme::element_light();
    let mut l = Loading::new();
    l.handle(LoadingMessage::Start);
    l.tick(500);
    l.handle(LoadingMessage::Stop);
    assert!(!l.active());
    let _element = l.view(&theme, make_content());
}

#[test]
fn test_loading_view_custom_message_type_renders() {
    let theme = Theme::element_light();
    let mut l = Loading::new();
    l.handle(LoadingMessage::Start);
    #[derive(Clone, Debug)]
    enum AppMsg {}
    let content: Element<AppMsg> = text("content").into();
    let _element = l.view(&theme, content);
}

#[test]
fn test_loading_view_fullscreen_with_text_dark_renders() {
    let theme = Theme::element_dark();
    let mut l = Loading::new()
        .with_fullscreen(true)
        .with_text("Loading");
    l.handle(LoadingMessage::Start);
    let _element = l.view(&theme, make_content());
}
