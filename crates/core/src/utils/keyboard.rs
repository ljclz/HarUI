//! 键盘工具函数
//!
//! 提供按键修饰符、快捷键注册表等功能。

use std::collections::HashMap;

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
