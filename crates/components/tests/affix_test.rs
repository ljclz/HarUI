//! Affix 固钉 — 参考 Element Plus `<el-affix>`。
//!
//! 覆盖：固定位置、偏移、滚动事件触发、目标容器、状态变化。

use har_ui_components::affix::{Affix, AffixMessage, AffixPosition};

#[test]
fn test_affix_default() {
    let a = Affix::new();
    assert_eq!(a.position(), AffixPosition::Top);
    assert_eq!(a.offset(), 0);
    assert!(!a.is_fixed());
}

#[test]
fn test_affix_custom_offset() {
    let a = Affix::new().with_offset(100);
    assert_eq!(a.offset(), 100);
}

#[test]
fn test_affix_position_bottom() {
    let a = Affix::new().with_position(AffixPosition::Bottom);
    assert_eq!(a.position(), AffixPosition::Bottom);
}

#[test]
fn test_affix_becomes_fixed_on_scroll() {
    let mut a = Affix::new().with_offset(200);
    // 滚动到 250（超过 offset 200）→ 应该固定
    a.handle(AffixMessage::Scroll { scroll_y: 250 });
    assert!(a.is_fixed());
}

#[test]
fn test_affix_releases_when_scroll_back() {
    let mut a = Affix::new().with_offset(200);
    a.handle(AffixMessage::Scroll { scroll_y: 300 });
    assert!(a.is_fixed());
    a.handle(AffixMessage::Scroll { scroll_y: 100 });
    assert!(!a.is_fixed());
}

#[test]
fn test_affix_zindex() {
    let a = Affix::new().with_zindex(1000);
    assert_eq!(a.zindex(), 1000);
}

#[test]
fn test_affix_target_container() {
    let a = Affix::new().with_target("#content");
    assert_eq!(a.target(), Some("#content"));
}

#[test]
fn test_affix_change_event_fires_on_toggle() {
    let mut a = Affix::new().with_offset(100);
    assert!(!a.change_fired());
    a.handle(AffixMessage::Scroll { scroll_y: 200 });
    assert!(a.change_fired());
    assert!(a.is_fixed());
    // 重置标记
    a.clear_change_flag();
    assert!(!a.change_fired());
    // 已是 fixed 状态，再滚不会再次触发 change
    a.handle(AffixMessage::Scroll { scroll_y: 300 });
    assert!(!a.change_fired());
    // 滚回去触发 change
    a.handle(AffixMessage::Scroll { scroll_y: 50 });
    assert!(a.change_fired());
    assert!(!a.is_fixed());
}

#[test]
fn test_affix_at_exact_offset_not_fixed() {
    let mut a = Affix::new().with_offset(200);
    // 等于 offset 不固定，必须严格大于
    a.handle(AffixMessage::Scroll { scroll_y: 200 });
    assert!(!a.is_fixed());
}
