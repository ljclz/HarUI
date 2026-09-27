//! Menu 组件 — 导航菜单
//!
//! 参考 Element Plus `<el-menu>`。
//! 支持：
//! - horizontal/vertical 两种 mode
//! - collapse 折叠（vertical 模式）
//! - 多级嵌套（支持三级）
//! - 选中态高亮
//! - disabled 项
//! - unique_opened（手风琴模式，同时只展开一个子菜单）

use har_ui_components::menu::{MenuItem, MenuMessage, MenuMode, MenuState};

// ---------- 基础构造 ----------

#[test]
fn test_menu_default() {
    let s = MenuState::new();
    assert_eq!(s.mode(), MenuMode::Vertical);
    assert!(!s.collapsed());
    assert!(!s.unique_opened());
    assert_eq!(s.active(), None);
    assert!(s.opened_submenus().is_empty());
}

#[test]
fn test_menu_with_mode_horizontal() {
    let s = MenuState::new().with_mode(MenuMode::Horizontal);
    assert_eq!(s.mode(), MenuMode::Horizontal);
}

#[test]
fn test_menu_with_collapse() {
    let s = MenuState::new().with_collapse(true);
    assert!(s.collapsed());
}

#[test]
fn test_menu_with_unique_opened() {
    let s = MenuState::new().with_unique_opened(true);
    assert!(s.unique_opened());
}

// ---------- 选中态 ----------

#[test]
fn test_menu_select() {
    let mut s = MenuState::new();
    s.handle(MenuMessage::Select("menu-1".to_string()));
    assert_eq!(s.active(), Some(&"menu-1".to_string()));
}

#[test]
fn test_menu_select_disabled_ignored() {
    // 禁用项不能被选中
    let mut s = MenuState::new();
    s.register_item(MenuItem::new("menu-1", "Menu 1").disabled(true));
    s.handle(MenuMessage::Select("menu-1".to_string()));
    assert_eq!(s.active(), None);
}

// ---------- 子菜单展开/收起 ----------

#[test]
fn test_menu_toggle_submenu() {
    let mut s = MenuState::new();
    s.handle(MenuMessage::ToggleSubmenu("sub-1".to_string()));
    assert!(s.opened_submenus().contains(&"sub-1".to_string()));

    s.handle(MenuMessage::ToggleSubmenu("sub-1".to_string()));
    assert!(!s.opened_submenus().contains(&"sub-1".to_string()));
}

#[test]
fn test_menu_unique_opened_only_one_expanded() {
    // 手风琴模式：同时只展开一个子菜单
    let mut s = MenuState::new().with_unique_opened(true);
    s.handle(MenuMessage::ToggleSubmenu("sub-1".to_string()));
    s.handle(MenuMessage::ToggleSubmenu("sub-2".to_string()));
    // sub-1 应被关闭
    assert!(!s.opened_submenus().contains(&"sub-1".to_string()));
    assert!(s.opened_submenus().contains(&"sub-2".to_string()));
}

// ---------- collapse 折叠 ----------

#[test]
fn test_menu_collapse_closes_all_submenus() {
    let mut s = MenuState::new();
    s.handle(MenuMessage::ToggleSubmenu("sub-1".to_string()));
    s.handle(MenuMessage::ToggleSubmenu("sub-2".to_string()));
    s.handle(MenuMessage::Collapse(true));
    assert!(s.opened_submenus().is_empty());
    assert!(s.collapsed());
}

#[test]
fn test_menu_collapse_horizontal_ignored() {
    // horizontal 模式下 collapse 无效
    let s = MenuState::new()
        .with_mode(MenuMode::Horizontal)
        .with_collapse(true);
    assert!(!s.collapsed());
}

// ---------- close_other_submenus ----------

#[test]
fn test_menu_close_others() {
    let mut s = MenuState::new();
    s.handle(MenuMessage::ToggleSubmenu("sub-1".to_string()));
    s.handle(MenuMessage::ToggleSubmenu("sub-2".to_string()));
    s.handle(MenuMessage::CloseOthers("sub-1".to_string()));
    assert!(s.opened_submenus().contains(&"sub-1".to_string()));
    assert!(!s.opened_submenus().contains(&"sub-2".to_string()));
}

// ---------- MenuItem ----------

#[test]
fn test_menu_item_default() {
    let item = MenuItem::new("m1", "Menu 1");
    assert_eq!(item.id, "m1");
    assert_eq!(item.label, "Menu 1");
    assert!(!item.disabled);
    assert!(item.children.is_empty());
}

#[test]
fn test_menu_item_with_children() {
    let parent = MenuItem::new("parent", "Parent")
        .with_child(MenuItem::new("child1", "Child 1"))
        .with_child(MenuItem::new("child2", "Child 2"));
    assert_eq!(parent.children.len(), 2);
    assert_eq!(parent.children[0].id, "child1");
}

#[test]
fn test_menu_item_disabled() {
    let item = MenuItem::new("m1", "Menu 1").disabled(true);
    assert!(item.disabled);
}

// ---------- 三级嵌套 ----------

#[test]
fn test_menu_three_level_nesting() {
    let l3 = MenuItem::new("l3", "Level 3");
    let l2 = MenuItem::new("l2", "Level 2").with_child(l3);
    let l1 = MenuItem::new("l1", "Level 1").with_child(l2);
    assert_eq!(l1.children[0].children[0].id, "l3");
}
