//! R.3.1 IME 桥接事件测试
//!
//! 验证 [`ImeBridgeEvent`] 枚举各变体可构造，[`subscription()`] 返回有效 [`Subscription`]。
//!
//! ## 测试范围
//! - 各变体的构造与字段访问
//! - `Clone` / `PartialEq` / `Debug` trait 实现
//! - `DEFAULT_IME_ID` 常量值
//! - `subscription()` 函数签名与返回类型
//!
//! ## 不测试的内容
//! `Subscription` 实际事件流需要 iced runtime 驱动，无法在普通单元测试中运行。
//! 此处仅验证类型可构造、函数签名正确。

use har_ui_core::utils::ime::{DEFAULT_IME_ID, ImeBridgeEvent, subscription};
use iced::Subscription;
use iced::advanced::widget::Id;

#[test]
fn test_ime_bridge_event_enabled_constructible() {
    let event = ImeBridgeEvent::Enabled {
        id: Id::new("input-1"),
    };
    assert!(matches!(event, ImeBridgeEvent::Enabled { .. }));
}

#[test]
fn test_ime_bridge_event_disabled_constructible() {
    let event = ImeBridgeEvent::Disabled {
        id: Id::new("input-1"),
    };
    assert!(matches!(event, ImeBridgeEvent::Disabled { .. }));
}

#[test]
fn test_ime_bridge_event_preedit_constructible_with_cursor() {
    let event = ImeBridgeEvent::Preedit {
        id: Id::new("input-1"),
        text: "你好".to_string(),
        cursor: Some((0, 2)),
    };
    assert!(matches!(event, ImeBridgeEvent::Preedit { .. }));
}

#[test]
fn test_ime_bridge_event_preedit_with_none_cursor() {
    let event = ImeBridgeEvent::Preedit {
        id: Id::unique(),
        text: "中".to_string(),
        cursor: None,
    };
    if let ImeBridgeEvent::Preedit { text, cursor, .. } = event {
        assert_eq!(text, "中");
        assert_eq!(cursor, None);
    } else {
        panic!("expected Preedit variant");
    }
}

#[test]
fn test_ime_bridge_event_commit_constructible() {
    let event = ImeBridgeEvent::Commit {
        id: Id::new("input-1"),
        text: "你好".to_string(),
    };
    if let ImeBridgeEvent::Commit { text, .. } = event {
        assert_eq!(text, "你好");
    } else {
        panic!("expected Commit variant");
    }
}

#[test]
fn test_ime_bridge_event_clone_and_eq() {
    let event1 = ImeBridgeEvent::Commit {
        id: Id::new("input-1"),
        text: "测试".to_string(),
    };
    let event2 = event1.clone();
    assert_eq!(event1, event2);
}

#[test]
fn test_ime_bridge_event_inequality() {
    let event1 = ImeBridgeEvent::Commit {
        id: Id::new("input-1"),
        text: "测试".to_string(),
    };
    let event2 = ImeBridgeEvent::Commit {
        id: Id::new("input-2"),
        text: "测试".to_string(),
    };
    assert_ne!(event1, event2);
}

#[test]
fn test_ime_bridge_event_debug_format() {
    let event = ImeBridgeEvent::Enabled {
        id: Id::new("test-id"),
    };
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("Enabled"));
}

#[test]
fn test_default_ime_id_constant() {
    assert_eq!(DEFAULT_IME_ID, "har-ui-ime");
}

#[test]
fn test_subscription_returns_subscription() {
    let sub: Subscription<ImeBridgeEvent> = subscription();
    // Subscription 是 #[must_use]，绑定到变量以证明可构造
    // 实际运行需要 iced runtime，此处仅验证类型签名
    drop(sub);
}

#[test]
fn test_subscription_can_be_batched() {
    // 验证多次调用 subscription() 返回的 Subscription 可 batch
    let sub1 = subscription();
    let sub2 = subscription();
    let batched = Subscription::batch([sub1, sub2]);
    drop(batched);
}

#[test]
fn test_subscription_can_be_mapped() {
    // 验证 subscription() 返回的 Subscription 支持 with（附加上下文值）
    // with 返回 Subscription<(A, T)>，其中 A 是附加值，T 是原输出
    let sub = subscription();
    let mapped: Subscription<(u8, ImeBridgeEvent)> = sub.with(42u8);
    drop(mapped);
}
