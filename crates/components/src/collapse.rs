//! Collapse 折叠面板 — 参考 Element Plus `<el-collapse>`。
//!
//! 支持：accordion 手风琴、active_keys、disabled item、Toggle/Open/Close/OpenAll/CloseAll。

use std::collections::HashSet;

/// Collapse 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CollapseMessage {
    /// 切换某项展开/折叠
    Toggle(String),
    /// 打开某项
    Open(String),
    /// 关闭某项
    Close(String),
    /// 全部打开（手风琴模式下被忽略）
    OpenAll,
    /// 全部关闭
    CloseAll,
}

/// Collapse 项
#[derive(Debug, Clone)]
pub struct CollapseItem {
    name: String,
    title: String,
    disabled: bool,
    default_active: bool,
}

impl CollapseItem {
    pub fn new(name: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            title: title.into(),
            disabled: false,
            default_active: false,
        }
    }

    pub fn with_disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }

    pub fn with_default_active(mut self, v: bool) -> Self {
        self.default_active = v;
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn default_active(&self) -> bool {
        self.default_active
    }
}

/// Collapse 组件
#[derive(Debug, Clone)]
pub struct Collapse {
    accordion: bool,
    items: Vec<CollapseItem>,
    active_keys: HashSet<String>,
}

impl Default for Collapse {
    fn default() -> Self {
        Self::new()
    }
}

impl Collapse {
    pub fn new() -> Self {
        Self {
            accordion: false,
            items: Vec::new(),
            active_keys: HashSet::new(),
        }
    }

    pub fn with_accordion(mut self, v: bool) -> Self {
        self.accordion = v;
        self
    }

    pub fn accordion(&self) -> bool {
        self.accordion
    }

    pub fn items(&self) -> &[CollapseItem] {
        &self.items
    }

    pub fn active_keys(&self) -> &HashSet<String> {
        &self.active_keys
    }

    pub fn is_active(&self, name: &str) -> bool {
        self.active_keys.contains(name)
    }

    pub fn add_item(&mut self, item: CollapseItem) {
        // 应用默认 active
        if item.default_active && !item.disabled {
            if self.accordion {
                // 手风琴模式：先清空，再激活
                self.active_keys.clear();
            }
            self.active_keys.insert(item.name.clone());
        }
        self.items.push(item);
    }

    fn find_item(&self, name: &str) -> Option<&CollapseItem> {
        self.items.iter().find(|i| i.name == name)
    }

    pub fn handle(&mut self, msg: CollapseMessage) {
        match msg {
            CollapseMessage::Toggle(name) => {
                let disabled = self
                    .find_item(&name)
                    .map(|i| i.disabled)
                    .unwrap_or(true);
                if disabled {
                    return;
                }
                if self.active_keys.contains(&name) {
                    self.active_keys.remove(&name);
                } else {
                    if self.accordion {
                        self.active_keys.clear();
                    }
                    self.active_keys.insert(name);
                }
            }
            CollapseMessage::Open(name) => {
                let disabled = self
                    .find_item(&name)
                    .map(|i| i.disabled)
                    .unwrap_or(true);
                if disabled {
                    return;
                }
                if self.accordion {
                    self.active_keys.clear();
                }
                self.active_keys.insert(name);
            }
            CollapseMessage::Close(name) => {
                self.active_keys.remove(&name);
            }
            CollapseMessage::OpenAll => {
                if self.accordion {
                    // 手风琴模式忽略 OpenAll
                    return;
                }
                for item in &self.items {
                    if !item.disabled {
                        self.active_keys.insert(item.name.clone());
                    }
                }
            }
            CollapseMessage::CloseAll => {
                self.active_keys.clear();
            }
        }
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_collapse_item_new() {
        let item = CollapseItem::new("a", "标题");
        assert_eq!(item.name(), "a");
        assert_eq!(item.title(), "标题");
        assert!(!item.disabled());
        assert!(!item.default_active());
    }
}
