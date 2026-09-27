//! Card 组件 — 卡片容器
//!
//! 参考 Element Plus `<el-card>`。
//! 支持：header/footer 插槽、shadow(hover/always/never)、body_style、图片卡片。

use har_ui_components::card::{Card, CardMessage, CardShadow};

// ---------- 基础构造 ----------

#[test]
fn test_card_default() {
    let c = Card::new("Hello");
    assert_eq!(c.body(), "Hello");
    assert_eq!(c.header(), None);
    assert_eq!(c.footer(), None);
    assert_eq!(c.shadow(), CardShadow::Always);
}

#[test]
fn test_card_with_shadow() {
    let c = Card::new("body").with_shadow(CardShadow::Hover);
    assert_eq!(c.shadow(), CardShadow::Hover);

    let c2 = Card::new("body").with_shadow(CardShadow::Never);
    assert_eq!(c2.shadow(), CardShadow::Never);
}

#[test]
fn test_card_with_header_footer() {
    let c = Card::new("body").with_header("Title").with_footer("Footer");
    assert_eq!(c.header(), Some(&"Title".to_string()));
    assert_eq!(c.footer(), Some(&"Footer".to_string()));
}

// ---------- 图片卡片 ----------

#[test]
fn test_card_with_image() {
    let c = Card::new("body").with_image("/path/to/img.png");
    assert_eq!(c.image(), Some(&"/path/to/img.png".to_string()));
}

#[test]
fn test_card_image_none_by_default() {
    let c = Card::new("body");
    assert_eq!(c.image(), None);
}

// ---------- hover 状态 ----------

#[test]
fn test_card_hover_state() {
    let mut c = Card::new("body").with_shadow(CardShadow::Hover);
    assert!(!c.is_hovered());
    c.handle(CardMessage::Hovered);
    assert!(c.is_hovered());
    c.handle(CardMessage::Unhovered);
    assert!(!c.is_hovered());
}

#[test]
fn test_card_shadow_always_independent_of_hover() {
    let mut c = Card::new("body").with_shadow(CardShadow::Always);
    c.handle(CardMessage::Hovered);
    // shadow=Always 时不论是否 hover 都显示阴影
    assert!(c.is_hovered());
    assert_eq!(c.shadow(), CardShadow::Always);
}

#[test]
fn test_card_shadow_never_no_shadow_even_on_hover() {
    let mut c = Card::new("body").with_shadow(CardShadow::Never);
    c.handle(CardMessage::Hovered);
    // shadow=Never 时即使 hover 也无阴影
    assert_eq!(c.shadow(), CardShadow::Never);
}

// ---------- should_show_shadow 计算属性 ----------

#[test]
fn test_card_should_show_shadow_always() {
    let c = Card::new("body").with_shadow(CardShadow::Always);
    assert!(c.should_show_shadow());
}

#[test]
fn test_card_should_show_shadow_never() {
    let c = Card::new("body").with_shadow(CardShadow::Never);
    assert!(!c.should_show_shadow());
}

#[test]
fn test_card_should_show_shadow_hover_only_when_hovered() {
    let mut c = Card::new("body").with_shadow(CardShadow::Hover);
    assert!(!c.should_show_shadow());
    c.handle(CardMessage::Hovered);
    assert!(c.should_show_shadow());
}

// ---------- body 更新 ----------

#[test]
fn test_card_update_body() {
    let mut c = Card::new("old");
    c.handle(CardMessage::UpdateBody("new".to_string()));
    assert_eq!(c.body(), "new");
}

// ---------- 空 body ----------

#[test]
fn test_card_empty_body() {
    let c = Card::new("");
    assert_eq!(c.body(), "");
}
