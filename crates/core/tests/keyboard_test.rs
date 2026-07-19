//! R.3.2 键盘事件桥接测试
//!
//! 验证 [`KeyEvent`] 枚举变体可构造，[`subscription()`] 返回有效 [`Subscription`]。
//!
//! ## 测试范围
//! - `Pressed` / `Released` 变体的构造与字段访问
//! - `Clone` / `PartialEq` / `Debug` trait 实现
//! - `subscription()` 函数签名与返回类型
//!
//! ## 不测试的内容
//! `Subscription` 实际事件流需要 iced runtime 驱动，无法在普通单元测试中运行。

use har_ui_core::utils::keyboard::{subscription, KeyEvent};
use iced::keyboard::{Key, Modifiers};
use iced::Subscription;

#[test]
fn test_key_event_pressed_constructible() {
    let event = KeyEvent::Pressed {
        key: Key::Character("a".into()),
        modifiers: Modifiers::default(),
    };
    assert!(matches!(event, KeyEvent::Pressed { .. }));
}

#[test]
fn test_key_event_released_constructible() {
    let event = KeyEvent::Released {
        key: Key::Character("a".into()),
        modifiers: Modifiers::default(),
    };
    assert!(matches!(event, KeyEvent::Released { .. }));
}

#[test]
fn test_key_event_pressed_with_modifiers() {
    let event = KeyEvent::Pressed {
        key: Key::Character("s".into()),
        modifiers: Modifiers::CTRL,
    };
    if let KeyEvent::Pressed { key, modifiers } = event {
        assert!(modifiers.contains(Modifiers::CTRL));
        assert!(!modifiers.contains(Modifiers::SHIFT));
        assert!(matches!(key, Key::Character(_)));
    } else {
        panic!("expected Pressed variant");
    }
}

#[test]
fn test_key_event_released_with_multiple_modifiers() {
    let mods = Modifiers::CTRL | Modifiers::SHIFT;
    let event = KeyEvent::Released {
        key: Key::Character("c".into()),
        modifiers: mods,
    };
    if let KeyEvent::Released { modifiers, .. } = event {
        assert!(modifiers.contains(Modifiers::CTRL));
        assert!(modifiers.contains(Modifiers::SHIFT));
    } else {
        panic!("expected Released variant");
    }
}

#[test]
fn test_key_event_clone_and_eq() {
    let event1 = KeyEvent::Pressed {
        key: Key::Character("a".into()),
        modifiers: Modifiers::default(),
    };
    let event2 = event1.clone();
    assert_eq!(event1, event2);
}

#[test]
fn test_key_event_inequality_on_different_key() {
    let event1 = KeyEvent::Pressed {
        key: Key::Character("a".into()),
        modifiers: Modifiers::default(),
    };
    let event2 = KeyEvent::Pressed {
        key: Key::Character("b".into()),
        modifiers: Modifiers::default(),
    };
    assert_ne!(event1, event2);
}

#[test]
fn test_key_event_inequality_on_different_modifiers() {
    let event1 = KeyEvent::Pressed {
        key: Key::Character("a".into()),
        modifiers: Modifiers::default(),
    };
    let event2 = KeyEvent::Pressed {
        key: Key::Character("a".into()),
        modifiers: Modifiers::SHIFT,
    };
    assert_ne!(event1, event2);
}

#[test]
fn test_key_event_inequality_on_pressed_vs_released() {
    let event1 = KeyEvent::Pressed {
        key: Key::Character("a".into()),
        modifiers: Modifiers::default(),
    };
    let event2 = KeyEvent::Released {
        key: Key::Character("a".into()),
        modifiers: Modifiers::default(),
    };
    assert_ne!(event1, event2);
}

#[test]
fn test_key_event_debug_format_pressed() {
    let event = KeyEvent::Pressed {
        key: Key::Character("a".into()),
        modifiers: Modifiers::default(),
    };
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("Pressed"));
}

#[test]
fn test_key_event_debug_format_released() {
    let event = KeyEvent::Released {
        key: Key::Character("a".into()),
        modifiers: Modifiers::default(),
    };
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("Released"));
}

#[test]
fn test_subscription_returns_subscription() {
    let sub: Subscription<KeyEvent> = subscription();
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
    let mapped: Subscription<(u8, KeyEvent)> = sub.with(42u8);
    drop(mapped);
}
