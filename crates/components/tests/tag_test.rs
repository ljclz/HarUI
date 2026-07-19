//! Tag 组件 — 标签
//!
//! 参考 Element Plus `<el-tag>`。
//! 支持：6 种 type、3 种 effect、3 种 size、closable、自定义颜色。

use har_ui_components::tag::{Tag, TagEffect, TagMessage, TagSize, TagType};

// ---------- 基础构造 ----------

#[test]
fn test_tag_default() {
    let t = Tag::new("Label");
    assert_eq!(t.text(), "Label");
    assert_eq!(t.tag_type(), TagType::Default);
    assert_eq!(t.effect(), TagEffect::Light);
    assert_eq!(t.size(), TagSize::Default);
    assert!(!t.closable());
    assert!(!t.hit());
    assert_eq!(t.color(), None);
    assert!(!t.is_closed());
}

#[test]
fn test_tag_with_type() {
    let types = [
        TagType::Default,
        TagType::Primary,
        TagType::Success,
        TagType::Warning,
        TagType::Danger,
        TagType::Info,
    ];
    assert_eq!(types.len(), 6);
    for ty in types {
        let t = Tag::new("x").with_type(ty);
        assert_eq!(t.tag_type(), ty);
    }
}

#[test]
fn test_tag_with_effect() {
    let effects = [TagEffect::Dark, TagEffect::Light, TagEffect::Plain];
    assert_eq!(effects.len(), 3);
    for e in effects {
        let t = Tag::new("x").with_effect(e);
        assert_eq!(t.effect(), e);
    }
}

#[test]
fn test_tag_with_size() {
    let sizes = [TagSize::Large, TagSize::Default, TagSize::Small];
    assert_eq!(sizes.len(), 3);
    for s in sizes {
        let t = Tag::new("x").with_size(s);
        assert_eq!(t.size(), s);
    }
}

#[test]
fn test_tag_with_closable() {
    let t = Tag::new("x").with_closable(true);
    assert!(t.closable());
}

#[test]
fn test_tag_with_color() {
    let t = Tag::new("x").with_color("#ff0000");
    assert_eq!(t.color(), Some(&"#ff0000".to_string()));
}

#[test]
fn test_tag_with_hit() {
    let t = Tag::new("x").with_hit(true);
    assert!(t.hit());
}

// ---------- 关闭 ----------

#[test]
fn test_tag_close_message() {
    let mut t = Tag::new("x").with_closable(true);
    assert!(!t.is_closed());
    t.handle(TagMessage::Close);
    assert!(t.is_closed());
}

#[test]
fn test_tag_close_not_closable_ignored() {
    // 非 closable 时 Close 消息应被忽略
    let mut t = Tag::new("x");
    t.handle(TagMessage::Close);
    assert!(!t.is_closed());
}

// ---------- 重置 ----------

#[test]
fn test_tag_reset_closed() {
    let mut t = Tag::new("x").with_closable(true);
    t.handle(TagMessage::Close);
    assert!(t.is_closed());
    t.reset();
    assert!(!t.is_closed());
}

// ---------- 更新文本 ----------

#[test]
fn test_tag_update_text() {
    let mut t = Tag::new("old");
    t.handle(TagMessage::UpdateText("new".to_string()));
    assert_eq!(t.text(), "new");
}

// ---------- 空文本 ----------

#[test]
fn test_tag_empty_text() {
    let t = Tag::new("");
    assert_eq!(t.text(), "");
}
