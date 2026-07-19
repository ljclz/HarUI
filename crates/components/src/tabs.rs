//! Tabs 组件 — 标签页
//!
//! 参考 Element Plus `<el-tabs>`。
//! 支持：default/card/border-card 三种 type，top/bottom/left/right 四种位置，
//! closable/addable/lazy，切换/关闭/新增。

/// Tabs 类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TabsType {
    #[default]
    Default,
    Card,
    BorderCard,
}

/// Tab 位置
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TabPosition {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

/// 单个 Tab 项
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TabItem {
    pub id: String,
    pub label: String,
    pub disabled: bool,
    pub closable: bool,
}

impl TabItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            disabled: false,
            closable: false,
        }
    }

    pub fn disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }

    pub fn closable(mut self, v: bool) -> Self {
        self.closable = v;
        self
    }
}

/// Tabs 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TabsMessage {
    /// 切换到指定 tab
    Select(String),
    /// 关闭指定 tab
    Close(String),
    /// 新增 tab
    Add(TabItem),
}

/// Tabs 组件
#[derive(Debug, Clone)]
pub struct Tabs {
    items: Vec<TabItem>,
    tabs_type: TabsType,
    position: TabPosition,
    closable: bool,
    addable: bool,
    lazy: bool,
    active: Option<String>,
    /// lazy 模式下已访问的 tab id
    visited: Vec<String>,
}

impl Default for Tabs {
    fn default() -> Self {
        Self::new()
    }
}

impl Tabs {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            tabs_type: TabsType::Default,
            position: TabPosition::Top,
            closable: false,
            addable: false,
            lazy: false,
            active: None,
            visited: Vec::new(),
        }
    }

    pub fn with_type(mut self, t: TabsType) -> Self {
        self.tabs_type = t;
        self
    }

    pub fn with_position(mut self, p: TabPosition) -> Self {
        self.position = p;
        self
    }

    pub fn with_closable(mut self, v: bool) -> Self {
        self.closable = v;
        self
    }

    pub fn with_addable(mut self, v: bool) -> Self {
        self.addable = v;
        self
    }

    pub fn with_lazy(mut self, v: bool) -> Self {
        self.lazy = v;
        self
    }

    pub fn with_item(mut self, item: TabItem) -> Self {
        if self.items.is_empty() {
            // 第一个 tab 默认激活
            self.active = Some(item.id.clone());
            self.visited.push(item.id.clone());
        }
        self.items.push(item);
        self
    }

    pub fn items(&self) -> &[TabItem] {
        &self.items
    }

    pub fn tabs_type(&self) -> TabsType {
        self.tabs_type
    }

    pub fn position(&self) -> TabPosition {
        self.position
    }

    pub fn closable(&self) -> bool {
        self.closable
    }

    pub fn addable(&self) -> bool {
        self.addable
    }

    pub fn lazy(&self) -> bool {
        self.lazy
    }

    pub fn active(&self) -> Option<&String> {
        self.active.as_ref()
    }

    /// 判断 tab 是否已访问（lazy 模式有效）
    pub fn is_visited(&self, id: &str) -> bool {
        if !self.lazy {
            return true;
        }
        self.visited.iter().any(|v| v == id)
    }

    /// 处理消息
    pub fn handle(&mut self, msg: TabsMessage) {
        match msg {
            TabsMessage::Select(id) => {
                // 检查是否存在且非 disabled
                let item = match self.items.iter().find(|i| i.id == id) {
                    Some(i) if !i.disabled => i.clone(),
                    _ => return,
                };
                self.active = Some(id.clone());
                if self.lazy && !self.visited.iter().any(|v| v == &id) {
                    self.visited.push(id);
                }
                let _ = item;
            }
            TabsMessage::Close(id) => {
                if let Some(pos) = self.items.iter().position(|i| i.id == id) {
                    let was_active = self.active.as_ref() == Some(&id);
                    self.items.remove(pos);
                    self.visited.retain(|v| v != &id);
                    if was_active {
                        // 自动激活相邻 tab
                        if !self.items.is_empty() {
                            let new_pos = pos.min(self.items.len() - 1);
                            self.active = Some(self.items[new_pos].id.clone());
                            if let Some(active_id) = self.active.as_ref() {
                                if self.lazy
                                    && !self.visited.iter().any(|v| v == active_id)
                                {
                                    self.visited.push(active_id.clone());
                                }
                            }
                        } else {
                            self.active = None;
                        }
                    }
                }
            }
            TabsMessage::Add(item) => {
                // 重复 ID 拒绝
                if self.items.iter().any(|i| i.id == item.id) {
                    return;
                }
                let new_id = item.id.clone();
                self.items.push(item);
                self.active = Some(new_id.clone());
                if self.lazy {
                    self.visited.push(new_id);
                }
            }
        }
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_tabs_default_no_items_no_active() {
        let t = Tabs::new();
        assert!(t.items().is_empty());
        assert_eq!(t.active(), None);
    }

    #[test]
    fn test_tabs_first_item_becomes_active() {
        let t = Tabs::new().with_item(TabItem::new("first", "First"));
        assert_eq!(t.active(), Some(&"first".to_string()));
    }

    #[test]
    fn test_tabs_close_active_falls_back_to_neighbor() {
        let mut t = Tabs::new()
            .with_item(TabItem::new("t1", "1").closable(true))
            .with_item(TabItem::new("t2", "2").closable(true))
            .with_item(TabItem::new("t3", "3").closable(true));
        t.handle(TabsMessage::Select("t2".to_string()));
        t.handle(TabsMessage::Close("t2".to_string()));
        // t2 关闭后应激活 t1 或 t3
        let active = t.active().unwrap();
        assert!(active == "t1" || active == "t3");
    }
}
