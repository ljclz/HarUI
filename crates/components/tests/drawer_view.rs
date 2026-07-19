//! Drawer view() 测试 — TDD RED 阶段

use har_ui_components::drawer::{Drawer, DrawerDirection, DrawerMessage};
use har_ui_core::theme::Theme;

#[test]
fn test_drawer_view_closed_renders_empty() {
    let theme = Theme::element_light();
    let d = Drawer::new();
    assert!(!d.visible());
    let _element = d.view(&theme, || ());
}

#[test]
fn test_drawer_view_opened_renders() {
    let theme = Theme::element_light();
    let mut d = Drawer::new();
    d.handle(DrawerMessage::Open);
    d.handle(DrawerMessage::AnimationEnd);
    let _element = d.view(&theme, || ());
}

#[test]
fn test_drawer_view_with_title_renders() {
    let theme = Theme::element_light();
    let mut d = Drawer::new().with_title("Settings");
    d.handle(DrawerMessage::Open);
    d.handle(DrawerMessage::AnimationEnd);
    let _element = d.view(&theme, || ());
}

#[test]
fn test_drawer_view_rtl_direction_renders() {
    let theme = Theme::element_light();
    let mut d = Drawer::new().with_direction(DrawerDirection::Rtl);
    d.handle(DrawerMessage::Open);
    d.handle(DrawerMessage::AnimationEnd);
    let _element = d.view(&theme, || ());
}

#[test]
fn test_drawer_view_ltr_direction_renders() {
    let theme = Theme::element_light();
    let mut d = Drawer::new().with_direction(DrawerDirection::Ltr);
    d.handle(DrawerMessage::Open);
    d.handle(DrawerMessage::AnimationEnd);
    let _element = d.view(&theme, || ());
}

#[test]
fn test_drawer_view_ttb_direction_renders() {
    let theme = Theme::element_light();
    let mut d = Drawer::new().with_direction(DrawerDirection::Ttb);
    d.handle(DrawerMessage::Open);
    d.handle(DrawerMessage::AnimationEnd);
    let _element = d.view(&theme, || ());
}

#[test]
fn test_drawer_view_btt_direction_renders() {
    let theme = Theme::element_light();
    let mut d = Drawer::new().with_direction(DrawerDirection::Btt);
    d.handle(DrawerMessage::Open);
    d.handle(DrawerMessage::AnimationEnd);
    let _element = d.view(&theme, || ());
}

#[test]
fn test_drawer_view_without_close_button_renders() {
    let theme = Theme::element_light();
    let mut d = Drawer::new().with_show_close(false);
    d.handle(DrawerMessage::Open);
    d.handle(DrawerMessage::AnimationEnd);
    let _element = d.view(&theme, || ());
}

#[test]
fn test_drawer_view_opening_state_renders() {
    let theme = Theme::element_light();
    let mut d = Drawer::new();
    d.handle(DrawerMessage::Open);
    // 处于 Opening 状态时仍渲染（visible() == true）
    let _element = d.view(&theme, || ());
}

#[test]
fn test_drawer_view_closing_state_renders() {
    let theme = Theme::element_light();
    let mut d = Drawer::new();
    d.handle(DrawerMessage::Open);
    d.handle(DrawerMessage::AnimationEnd);
    d.handle(DrawerMessage::Close);
    // 处于 Closing 状态时仍渲染（visible() == true）
    let _element = d.view(&theme, || ());
}

#[test]
fn test_drawer_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let mut d = Drawer::new().with_title("T");
    d.handle(DrawerMessage::Open);
    d.handle(DrawerMessage::AnimationEnd);
    let _element = d.view(&theme, || ());
}
