//! 焦点环 / Tab 序管理 — 无样式纯逻辑，零渲染依赖（ADR-009）
//!
//! 维护一组可聚焦项的 Tab 顺序：前进/后退环绕、跳过禁用项、动态增删。
//! 调用方负责把焦点 key 映射到实际组件（本层不感知渲染）。

/// 焦点环 — Tab 序注册表
///
/// 项按注册顺序形成环绕序列；`enabled` 控制该项是否可接收焦点
/// （禁用项在 next/prev 中被跳过，但仍保留在序列中以便恢复）。
#[derive(Debug, Clone, Default)]
pub struct FocusRing {
    order: Vec<String>,
    enabled: Vec<bool>,
    current: Option<usize>,
}

impl FocusRing {
    pub fn new() -> Self {
        Self::default()
    }

    /// 按注册顺序追加可聚焦项（默认启用）
    pub fn register(&mut self, key: impl Into<String>) {
        self.register_with(key, true);
    }

    /// 追加项并指定初始启用状态
    pub fn register_with(&mut self, key: impl Into<String>, enabled: bool) {
        self.order.push(key.into());
        self.enabled.push(enabled);
    }

    /// 设置某项启用状态；禁用当前焦点项时同时让出焦点
    pub fn set_enabled(&mut self, key: &str, enabled: bool) {
        if let Some(i) = self.index_of(key) {
            self.enabled[i] = enabled;
            if !enabled && self.current == Some(i) {
                self.current = None;
            }
        }
    }

    /// 移除项；被移除的是当前焦点时让出焦点
    pub fn remove(&mut self, key: &str) {
        if let Some(i) = self.index_of(key) {
            self.order.remove(i);
            self.enabled.remove(i);
            self.current = match self.current {
                Some(c) if c == i => None,
                Some(c) if c > i => Some(c - 1),
                other => other,
            };
        }
    }

    /// 当前焦点 key
    pub fn focused(&self) -> Option<&str> {
        self.current
            .and_then(|i| self.order.get(i).map(String::as_str))
    }

    /// 尝试聚焦某项；项不存在或被禁用时返回 false
    pub fn focus(&mut self, key: &str) -> bool {
        match self.index_of(key) {
            Some(i) if self.enabled[i] => {
                self.current = Some(i);
                true
            }
            _ => false,
        }
    }

    /// Tab 前进（环绕，跳过禁用项）；无可聚焦项时保持不变并返回 None
    ///
    /// 命名 `focus_next` 以避免与 `Iterator::next` 混淆
    pub fn focus_next(&mut self) -> Option<&str> {
        self.step(1)
    }

    /// Shift+Tab 后退（环绕，跳过禁用项）
    pub fn focus_prev(&mut self) -> Option<&str> {
        self.step(-1)
    }

    /// 让出焦点
    pub fn reset(&mut self) {
        self.current = None;
    }

    /// 序列内项数
    pub fn len(&self) -> usize {
        self.order.len()
    }

    pub fn is_empty(&self) -> bool {
        self.order.is_empty()
    }

    // ---------- 内部 ----------

    fn index_of(&self, key: &str) -> Option<usize> {
        self.order.iter().position(|k| k == key)
    }

    /// 从 current 起步进 delta（环绕），返回新的焦点 key
    fn step(&mut self, delta: isize) -> Option<&str> {
        if self.order.is_empty() || !self.enabled.iter().any(|&e| e) {
            return None;
        }
        let n = self.order.len() as isize;
        // 起点：无焦点时，正向从 -1（首个匹配即 0）、反向从 0（首个匹配即 n-1）出发
        let start = match self.current {
            Some(c) => c as isize,
            None if delta > 0 => -1,
            None => n,
        };
        let mut i = start;
        for _ in 0..n {
            i = (i + delta).rem_euclid(n);
            if self.enabled[i as usize] {
                self.current = Some(i as usize);
                return Some(self.order[i as usize].as_str());
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_and_sequential_next() {
        let mut ring = FocusRing::new();
        ring.register("a");
        ring.register("b");
        ring.register("c");
        assert_eq!(ring.focus_next(), Some("a"));
        assert_eq!(ring.focus_next(), Some("b"));
        assert_eq!(ring.focus_next(), Some("c"));
        assert_eq!(ring.focus_next(), Some("a")); // 环绕
        assert_eq!(ring.focused(), Some("a"));
    }

    #[test]
    fn test_prev_wraps_backward() {
        let mut ring = FocusRing::new();
        ring.register("a");
        ring.register("b");
        assert_eq!(ring.focus_prev(), Some("b")); // 反向从末尾开始
        assert_eq!(ring.focus_prev(), Some("a"));
    }

    #[test]
    fn test_disabled_items_are_skipped() {
        let mut ring = FocusRing::new();
        ring.register("a");
        ring.register_with("b", false);
        ring.register("c");
        assert_eq!(ring.focus_next(), Some("a"));
        assert_eq!(ring.focus_next(), Some("c")); // b 被跳过
        assert_eq!(ring.focus_next(), Some("a"));
        // 直接聚焦禁用项被拒绝
        assert!(!ring.focus("b"));
        assert_eq!(ring.focused(), Some("a"));
    }

    #[test]
    fn test_disable_current_yields_focus() {
        let mut ring = FocusRing::new();
        ring.register("a");
        ring.register("b");
        ring.focus_next();
        assert_eq!(ring.focused(), Some("a"));
        ring.set_enabled("a", false);
        assert_eq!(ring.focused(), None);
        assert_eq!(ring.focus_next(), Some("b"));
    }

    #[test]
    fn test_remove_adjusts_current() {
        let mut ring = FocusRing::new();
        ring.register("a");
        ring.register("b");
        ring.register("c");
        ring.focus("b");
        ring.remove("b");
        // 被移除的是当前焦点 → 焦点让出，Tab 从头开始
        assert_eq!(ring.focused(), None);
        assert_eq!(ring.focus_next(), Some("a"));
        assert_eq!(ring.len(), 2);
        // 移除非焦点项不影响当前焦点
        ring.focus("c");
        ring.remove("a");
        assert_eq!(ring.focused(), Some("c"));
    }

    #[test]
    fn test_no_enabled_items_returns_none() {
        let mut ring = FocusRing::new();
        ring.register_with("a", false);
        assert_eq!(ring.focus_next(), None);
        assert_eq!(ring.focus_prev(), None);
        // 空环
        assert_eq!(FocusRing::new().focus_next(), None);
    }

    #[test]
    fn test_focus_direct_and_reset() {
        let mut ring = FocusRing::new();
        ring.register("a");
        ring.register("b");
        assert!(ring.focus("b"));
        assert_eq!(ring.focused(), Some("b"));
        ring.reset();
        assert_eq!(ring.focused(), None);
        assert_eq!(ring.focus_next(), Some("a")); // reset 后正向从首个开始
    }
}
