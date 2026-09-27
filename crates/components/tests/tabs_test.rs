//! Tabs 组件 — 标签页
//!
//! 参考 Element Plus `<el-tabs>`。
//! 支持：
//! - default/card/border-card 三种 type
//! - top/bottom/left/right 四种 tab 位置
//! - closable（可关闭）
//! - addable（可新增）
//! - lazy（延迟渲染）

use har_ui_components::tabs::{TabItem, TabPosition, Tabs, TabsMessage, TabsType};

// ---------- 基础构造 ----------

#[test]
fn test_tabs_default() {
    let t = Tabs::new();
    assert_eq!(t.tabs_type(), TabsType::Default);
    assert_eq!(t.position(), TabPosition::Top);
    assert!(!t.closable());
    assert!(!t.addable());
    assert!(!t.lazy());
    assert_eq!(t.active(), None);
    assert!(t.items().is_empty());
}

#[test]
fn test_tabs_with_type() {
    let t = Tabs::new().with_type(TabsType::Card);
    assert_eq!(t.tabs_type(), TabsType::Card);

    let t2 = Tabs::new().with_type(TabsType::BorderCard);
    assert_eq!(t2.tabs_type(), TabsType::BorderCard);
}

#[test]
fn test_tabs_with_position() {
    let t = Tabs::new().with_position(TabPosition::Bottom);
    assert_eq!(t.position(), TabPosition::Bottom);

    let t2 = Tabs::new().with_position(TabPosition::Left);
    assert_eq!(t2.position(), TabPosition::Left);

    let t3 = Tabs::new().with_position(TabPosition::Right);
    assert_eq!(t3.position(), TabPosition::Right);
}

#[test]
fn test_tabs_with_closable_addable_lazy() {
    let t = Tabs::new()
        .with_closable(true)
        .with_addable(true)
        .with_lazy(true);
    assert!(t.closable());
    assert!(t.addable());
    assert!(t.lazy());
}

// ---------- TabItem ----------

#[test]
fn test_tab_item_default() {
    let item = TabItem::new("tab-1", "Tab 1");
    assert_eq!(item.id, "tab-1");
    assert_eq!(item.label, "Tab 1");
    assert!(!item.disabled);
    assert!(!item.closable);
}

#[test]
fn test_tab_item_disabled() {
    let item = TabItem::new("tab-1", "Tab 1").disabled(true);
    assert!(item.disabled);
}

#[test]
fn test_tab_item_closable_override() {
    let item = TabItem::new("tab-1", "Tab 1").closable(true);
    assert!(item.closable);
}

// ---------- 添加 tab ----------

#[test]
fn test_tabs_add_items() {
    let t = Tabs::new()
        .with_item(TabItem::new("t1", "Tab 1"))
        .with_item(TabItem::new("t2", "Tab 2"))
        .with_item(TabItem::new("t3", "Tab 3"));
    assert_eq!(t.items().len(), 3);
    // 默认激活第一个
    assert_eq!(t.active(), Some(&"t1".to_string()));
}

#[test]
fn test_tabs_active_default_first() {
    let t = Tabs::new().with_item(TabItem::new("first", "First"));
    assert_eq!(t.active(), Some(&"first".to_string()));
}

// ---------- 切换 tab ----------

#[test]
fn test_tabs_switch_active() {
    let mut t = Tabs::new()
        .with_item(TabItem::new("t1", "Tab 1"))
        .with_item(TabItem::new("t2", "Tab 2"));
    t.handle(TabsMessage::Select("t2".to_string()));
    assert_eq!(t.active(), Some(&"t2".to_string()));
}

#[test]
fn test_tabs_switch_to_disabled_ignored() {
    let mut t = Tabs::new()
        .with_item(TabItem::new("t1", "Tab 1"))
        .with_item(TabItem::new("t2", "Tab 2").disabled(true));
    t.handle(TabsMessage::Select("t2".to_string()));
    // disabled 不能切换
    assert_eq!(t.active(), Some(&"t1".to_string()));
}

#[test]
fn test_tabs_switch_to_nonexistent_ignored() {
    let mut t = Tabs::new().with_item(TabItem::new("t1", "Tab 1"));
    t.handle(TabsMessage::Select("nonexistent".to_string()));
    assert_eq!(t.active(), Some(&"t1".to_string()));
}

// ---------- 关闭 tab ----------

#[test]
fn test_tabs_close_tab() {
    let mut t = Tabs::new()
        .with_item(TabItem::new("t1", "Tab 1").closable(true))
        .with_item(TabItem::new("t2", "Tab 2").closable(true));
    t.handle(TabsMessage::Select("t1".to_string()));
    t.handle(TabsMessage::Close("t1".to_string()));
    assert_eq!(t.items().len(), 1);
    // 关闭当前激活 tab 后应自动激活相邻 tab
    assert_eq!(t.active(), Some(&"t2".to_string()));
}

#[test]
fn test_tabs_close_nonexistent_ignored() {
    let mut t = Tabs::new().with_item(TabItem::new("t1", "Tab 1"));
    t.handle(TabsMessage::Close("nonexistent".to_string()));
    assert_eq!(t.items().len(), 1);
}

#[test]
fn test_tabs_close_last_tab() {
    let mut t = Tabs::new().with_item(TabItem::new("t1", "Tab 1").closable(true));
    t.handle(TabsMessage::Close("t1".to_string()));
    assert_eq!(t.items().len(), 0);
    assert_eq!(t.active(), None);
}

// ---------- 新增 tab ----------

#[test]
fn test_tabs_add_tab() {
    let mut t = Tabs::new().with_item(TabItem::new("t1", "Tab 1"));
    t.handle(TabsMessage::Add(TabItem::new("t2", "Tab 2")));
    assert_eq!(t.items().len(), 2);
    // 新增后激活新 tab
    assert_eq!(t.active(), Some(&"t2".to_string()));
}

#[test]
fn test_tabs_add_duplicate_id_rejected() {
    let mut t = Tabs::new().with_item(TabItem::new("t1", "Tab 1"));
    t.handle(TabsMessage::Add(TabItem::new("t1", "Duplicate")));
    assert_eq!(t.items().len(), 1);
    assert_eq!(t.items()[0].label, "Tab 1");
}

// ---------- lazy 延迟渲染（已访问的 tab 集合） ----------

#[test]
fn test_tabs_lazy_visited_set() {
    let mut t = Tabs::new()
        .with_lazy(true)
        .with_item(TabItem::new("t1", "Tab 1"))
        .with_item(TabItem::new("t2", "Tab 2"))
        .with_item(TabItem::new("t3", "Tab 3"));
    // 初始：t1 被访问
    assert!(t.is_visited("t1"));
    assert!(!t.is_visited("t2"));
    // 切换到 t2 后 t2 被访问
    t.handle(TabsMessage::Select("t2".to_string()));
    assert!(t.is_visited("t2"));
    // t3 仍未访问
    assert!(!t.is_visited("t3"));
}

#[test]
fn test_tabs_non_lazy_all_visited() {
    // 非 lazy 模式下所有 tab 视为已访问
    let t = Tabs::new()
        .with_item(TabItem::new("t1", "Tab 1"))
        .with_item(TabItem::new("t2", "Tab 2"));
    assert!(t.is_visited("t1"));
    assert!(t.is_visited("t2"));
}
