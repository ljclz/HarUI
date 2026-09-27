//! 键盘工具函数
//!
//! 提供按键修饰符、快捷键注册表等功能。
//!
//! ## R.3.2 键盘事件桥接（subscription）
//!
//! [`subscription`] 提供 iced 键盘事件流到 [`KeyEvent`] 的桥接，
//! 通过 `iced::event::listen_with` 捕获 `KeyPressed` / `KeyReleased` 事件。

use std::collections::HashMap;

use iced::Subscription;
use iced::event::{self, Event, Status};
use iced::keyboard::{self, Key, Modifiers};
use iced::window::Id as WindowId;

/// 按键修饰符状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct KeyModifiers {
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    pub meta: bool,
}

/// 快捷键定义
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Shortcut {
    pub key: char,
    pub modifiers: KeyModifiers,
}

impl Shortcut {
    pub fn new(key: char, modifiers: KeyModifiers) -> Self {
        // 快捷键字符统一大写存储，便于匹配
        Self {
            key: key.to_ascii_uppercase(),
            modifiers,
        }
    }
}

/// 快捷键注册表
#[derive(Debug, Clone, Default)]
pub struct ShortcutRegistry {
    /// key → action name 的映射
    bindings: HashMap<Shortcut, &'static str>,
}

impl ShortcutRegistry {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    /// 注册快捷键 → action
    pub fn register(&mut self, action: &'static str, shortcut: Shortcut) {
        // 覆盖同名快捷键的 action
        self.bindings.insert(shortcut, action);
    }

    /// 匹配快捷键，返回对应的 action 名称
    pub fn match_shortcut(&self, shortcut: &Shortcut) -> Option<&'static str> {
        self.bindings.get(shortcut).copied()
    }

    /// 已注册的快捷键数量
    pub fn len(&self) -> usize {
        self.bindings.len()
    }

    /// 是否为空
    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty()
    }
}

// ============================================================================
// R.3.2 键盘事件桥接（iced Subscription）
// ============================================================================

/// 键盘事件（R.3.2）
///
/// 通过 iced 事件流捕获的键盘按键事件，使用 iced 原生 [`Key`] / [`Modifiers`] 类型。
/// 与本模块的 [`KeyModifiers`]（bool 字段结构）不同，
/// [`Modifiers`] 是 iced 的 bitflags 类型。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyEvent {
    /// 按键按下
    Pressed { key: Key, modifiers: Modifiers },
    /// 按键释放
    Released { key: Key, modifiers: Modifiers },
}

/// 键盘事件订阅（R.3.2）
///
/// 返回 [`Subscription<KeyEvent>`]，接入 iced 事件流并过滤键盘事件。
///
/// ## 实现细节
/// 通过 [`iced::event::listen_with`] 订阅所有 iced 事件，
/// 过滤 `keyboard::Event::KeyPressed` / `KeyReleased` 并转换为 [`KeyEvent`]。
///
/// ## 与 `iced::keyboard::on_key_press` 的区别
/// `on_key_press` 仅在事件 `Status::Ignored` 时触发（被 widget 捕获的按键不通知）。
/// 本订阅使用 `listen_with` 对所有状态（含 `Captured`）的键盘事件都触发，
/// 便于应用层做全局快捷键监听。
pub fn subscription() -> Subscription<KeyEvent> {
    event::listen_with(filter_key_event)
}

/// `listen_with` 的过滤函数（函数指针，非闭包，符合 iced 0.13.x 签名约束）
fn filter_key_event(event: Event, _status: Status, _window: WindowId) -> Option<KeyEvent> {
    match event {
        Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }) => {
            Some(KeyEvent::Pressed { key, modifiers })
        }
        Event::Keyboard(keyboard::Event::KeyReleased { key, modifiers, .. }) => {
            Some(KeyEvent::Released { key, modifiers })
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortcut_normalizes_key_case() {
        let s1 = Shortcut::new('a', KeyModifiers::default());
        let s2 = Shortcut::new('A', KeyModifiers::default());
        assert_eq!(s1, s2);
    }
}
