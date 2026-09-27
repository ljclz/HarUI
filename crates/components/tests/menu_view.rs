//! Menu view() 测试 — TDD RED 阶段

use har_ui_components::menu::{MenuItem, MenuMessage, MenuMode, MenuState};
use har_ui_core::theme::Theme;

#[test]
fn test_menu_view_empty_renders() {
    let theme = Theme::element_light();
    let s = MenuState::new();
    let _element = s.view(&theme, |_| ());
}

#[test]
fn test_menu_view_with_items_renders() {
    let theme = Theme::element_light();
    let mut s = MenuState::new();
    s.register_item(MenuItem::new("m1", "Menu 1"));
    s.register_item(MenuItem::new("m2", "Menu 2"));
    let _element = s.view(&theme, |_| ());
}

#[test]
fn test_menu_view_with_submenu_renders() {
    let theme = Theme::element_light();
    let mut s = MenuState::new();
    let parent = MenuItem::new("parent", "Parent")
        .with_child(MenuItem::new("child1", "Child 1"))
        .with_child(MenuItem::new("child2", "Child 2"));
    s.register_item(parent);
    let _element = s.view(&theme, |_| ());
}

#[test]
fn test_menu_view_submenu_expanded_renders() {
    let theme = Theme::element_light();
    let mut s = MenuState::new();
    let parent = MenuItem::new("parent", "Parent").with_child(MenuItem::new("child1", "Child 1"));
    s.register_item(parent);
    s.handle(MenuMessage::ToggleSubmenu("parent".to_string()));
    let _element = s.view(&theme, |_| ());
}

#[test]
fn test_menu_view_horizontal_mode_renders() {
    let theme = Theme::element_light();
    let mut s = MenuState::new().with_mode(MenuMode::Horizontal);
    s.register_item(MenuItem::new("m1", "M1"));
    s.register_item(MenuItem::new("m2", "M2"));
    let _element = s.view(&theme, |_| ());
}

#[test]
fn test_menu_view_collapsed_renders() {
    let theme = Theme::element_light();
    let mut s = MenuState::new().with_collapse(true);
    s.register_item(MenuItem::new("m1", "M1"));
    let _element = s.view(&theme, |_| ());
}

#[test]
fn test_menu_view_disabled_item_renders() {
    let theme = Theme::element_light();
    let mut s = MenuState::new();
    s.register_item(MenuItem::new("m1", "M1").disabled(true));
    let _element = s.view(&theme, |_| ());
}

#[test]
fn test_menu_view_active_item_renders() {
    let theme = Theme::element_light();
    let mut s = MenuState::new();
    s.register_item(MenuItem::new("m1", "M1"));
    s.register_item(MenuItem::new("m2", "M2"));
    s.handle(MenuMessage::Select("m1".to_string()));
    let _element = s.view(&theme, |_| ());
}

#[test]
fn test_menu_view_unique_opened_renders() {
    let theme = Theme::element_light();
    let mut s = MenuState::new().with_unique_opened(true);
    s.register_item(MenuItem::new("p1", "P1").with_child(MenuItem::new("c1", "C1")));
    s.register_item(MenuItem::new("p2", "P2").with_child(MenuItem::new("c2", "C2")));
    s.handle(MenuMessage::ToggleSubmenu("p1".to_string()));
    s.handle(MenuMessage::ToggleSubmenu("p2".to_string()));
    let _element = s.view(&theme, |_| ());
}

#[test]
fn test_menu_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let mut s = MenuState::new();
    s.register_item(MenuItem::new("m1", "M1"));
    let _element = s.view(&theme, |_| ());
}
