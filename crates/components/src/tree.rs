//! Tree 树形组件 — 参考 Element Plus `<el-tree>`。
//!
//! 支持：基础树/展开折叠/复选框/父子联动/半选/过滤/懒加载/自定义节点数据/1000 节点性能。

use std::collections::HashMap;
use std::collections::HashSet;

use har_ui_core::theme::Theme;
use iced::widget::{button, container, text};
use iced::{Color, Element, Length, Padding};

/// 树节点
#[derive(Debug, Clone)]
pub struct TreeNode {
    id: String,
    label: String,
    children: Vec<TreeNode>,
    disabled: bool,
    /// 懒加载标记：true 表示尚未加载子节点
    lazy: bool,
    /// 自定义节点附加数据（如 icon、type 等）
    extra: HashMap<String, String>,
}

impl TreeNode {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            children: Vec::new(),
            disabled: false,
            lazy: false,
            extra: HashMap::new(),
        }
    }

    /// 创建一个懒加载节点（尚未加载子节点）
    pub fn new_lazy(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            children: Vec::new(),
            disabled: false,
            lazy: true,
            extra: HashMap::new(),
        }
    }

    pub fn with_child(mut self, child: TreeNode) -> Self {
        self.children.push(child);
        self
    }

    pub fn with_disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }

    pub fn with_extra(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.extra.insert(key.into(), value.into());
        self
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn children(&self) -> &[TreeNode] {
        &self.children
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn lazy(&self) -> bool {
        self.lazy
    }

    pub fn extra(&self, key: &str) -> Option<&str> {
        self.extra.get(key).map(|s| s.as_str())
    }

    pub fn is_leaf(&self) -> bool {
        self.children.is_empty() && !self.lazy
    }
}

/// Tree 消息
#[derive(Debug, Clone)]
pub enum TreeMessage {
    /// 切换节点展开/折叠
    ToggleExpand(String),
    /// 切换节点勾选（仅 show_checkbox 时生效）
    ToggleCheck(String),
    /// 过滤节点（保留匹配节点及其祖先链）
    Filter(String),
    /// 懒加载：为指定节点设置子节点
    LoadChildren(String, Vec<TreeNode>),
    /// 设置整棵树的数据
    SetData(Vec<TreeNode>),
}

/// Tree 组件
#[derive(Debug, Clone)]
pub struct Tree {
    roots: Vec<TreeNode>,
    /// 展开的节点 id 集合
    expanded: HashSet<String>,
    /// 选中的节点 id 集合（仅叶子节点真正选中，父节点由计算得出）
    checked: HashSet<String>,
    show_checkbox: bool,
    default_expand_all: bool,
    /// 当前过滤关键字
    filter: String,
}

impl Default for Tree {
    fn default() -> Self {
        Self::new()
    }
}

impl Tree {
    pub fn new() -> Self {
        Self {
            roots: Vec::new(),
            expanded: HashSet::new(),
            checked: HashSet::new(),
            show_checkbox: false,
            default_expand_all: false,
            filter: String::new(),
        }
    }

    pub fn with_data(mut self, roots: Vec<TreeNode>) -> Self {
        self.roots = roots;
        if self.default_expand_all {
            self.expand_all_recursive(&self.roots.clone());
        }
        self
    }

    pub fn with_show_checkbox(mut self, v: bool) -> Self {
        self.show_checkbox = v;
        self
    }

    pub fn with_default_expand_all(mut self, v: bool) -> Self {
        self.default_expand_all = v;
        if v {
            self.expand_all_recursive(&self.roots.clone());
        }
        self
    }

    fn expand_all_recursive(&mut self, nodes: &[TreeNode]) {
        for n in nodes {
            if !n.is_leaf() {
                self.expanded.insert(n.id.clone());
                self.expand_all_recursive(&n.children);
            }
        }
    }

    pub fn roots(&self) -> &[TreeNode] {
        &self.roots
    }

    pub fn is_expanded(&self, id: &str) -> bool {
        self.expanded.contains(id)
    }

    pub fn is_checked(&self, id: &str) -> bool {
        let node = match self.find_node(id) {
            Some(n) => n,
            None => return false,
        };
        let mut leaves = Vec::new();
        Self::collect_leaf_ids(node, &mut leaves);
        if leaves.is_empty() {
            // 自身就是叶子
            self.checked.contains(id)
        } else {
            // 非叶子：所有后代叶子都被选中才算 checked
            !leaves.is_empty() && leaves.iter().all(|l| self.checked.contains(l))
        }
    }

    pub fn is_lazy(&self, id: &str) -> bool {
        self.find_node(id).map(|n| n.lazy).unwrap_or(false)
    }

    pub fn is_filtered(&self) -> bool {
        !self.filter.is_empty()
    }

    pub fn checked(&self) -> &HashSet<String> {
        &self.checked
    }

    pub fn show_checkbox(&self) -> bool {
        self.show_checkbox
    }

    /// 在整棵树中查找节点
    fn find_node(&self, id: &str) -> Option<&TreeNode> {
        fn dfs<'a>(nodes: &'a [TreeNode], id: &str) -> Option<&'a TreeNode> {
            for n in nodes {
                if n.id == id {
                    return Some(n);
                }
                if let Some(found) = dfs(&n.children, id) {
                    return Some(found);
                }
            }
            None
        }
        dfs(&self.roots, id)
    }

    /// 收集指定节点的所有叶子节点 id（递归）
    fn collect_leaf_ids(node: &TreeNode, out: &mut Vec<String>) {
        if node.is_leaf() {
            out.push(node.id.clone());
        } else {
            for c in &node.children {
                Self::collect_leaf_ids(c, out);
            }
        }
    }

    /// 计算节点是否半选（有部分但非全部叶子被选中）
    pub fn is_indeterminate(&self, id: &str) -> bool {
        let node = match self.find_node(id) {
            Some(n) => n,
            None => return false,
        };
        let mut leaves = Vec::new();
        Self::collect_leaf_ids(node, &mut leaves);
        if leaves.is_empty() {
            return false;
        }
        let checked_count = leaves.iter().filter(|l| self.checked.contains(*l)).count();
        checked_count > 0 && checked_count < leaves.len()
    }

    /// 处理消息
    pub fn handle(&mut self, msg: TreeMessage) {
        match msg {
            TreeMessage::ToggleExpand(id) => {
                if self.expanded.contains(&id) {
                    self.expanded.remove(&id);
                } else {
                    self.expanded.insert(id);
                }
            }
            TreeMessage::ToggleCheck(id) => {
                if !self.show_checkbox {
                    return;
                }
                let node = match self.find_node(&id) {
                    Some(n) => n.clone(),
                    None => return,
                };
                if node.disabled {
                    return;
                }
                // 收集该节点下的所有叶子（含自身如果它是叶子）
                let mut leaves = Vec::new();
                Self::collect_leaf_ids(&node, &mut leaves);
                if leaves.is_empty() {
                    // 自身就是叶子
                    leaves.push(id.clone());
                }
                // 若当前节点（及其叶子）全部已选中 → 取消；否则全选
                let all_checked = leaves.iter().all(|l| self.checked.contains(l));
                for l in &leaves {
                    if all_checked {
                        self.checked.remove(l);
                    } else {
                        self.checked.insert(l.clone());
                    }
                }
            }
            TreeMessage::Filter(keyword) => {
                self.filter = keyword;
            }
            TreeMessage::LoadChildren(id, children) => {
                if let Some(node) = find_node_mut(&mut self.roots, &id) {
                    node.lazy = false;
                    node.children = children;
                }
            }
            TreeMessage::SetData(roots) => {
                self.roots = roots;
                self.expanded.clear();
                self.checked.clear();
                if self.default_expand_all {
                    let snapshot = self.roots.clone();
                    self.expand_all_recursive(&snapshot);
                }
            }
        }
    }

    /// 返回当前可见节点列表（按 DFS 顺序）
    pub fn visible_nodes(&self) -> Vec<&TreeNode> {
        let mut out = Vec::new();
        let filter = self.filter.clone();
        for root in &self.roots {
            collect_visible(root, self, &filter, &mut out);
        }
        out
    }

    /// 渲染 Tree 为 iced::Element
    ///
    /// - roots 为空时返回空容器
    /// - 否则递归渲染所有 roots，按 expanded 状态展开子节点
    /// - on_toggle 参数为节点 id（用于切换展开/折叠）
    /// - on_select 参数为节点 id（用于选中节点）
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        on_toggle: impl Fn(String) -> Message + 'a + Clone,
        on_select: impl Fn(String) -> Message + 'a + Clone,
    ) -> Element<'a, Message> {
        if self.roots.is_empty() {
            return container(text("")).into();
        }

        let children: Vec<Element<'a, Message>> = self
            .roots
            .iter()
            .map(|n| Self::render_node(n, self, theme, &on_toggle, &on_select, 0))
            .collect();

        iced::widget::Column::with_children(children)
            .spacing(2)
            .into()
    }

    /// 递归渲染单个 TreeNode
    fn render_node<'a, Message: Clone + 'a>(
        node: &'a TreeNode,
        tree: &'a Tree,
        theme: &'a Theme,
        on_toggle: &(impl Fn(String) -> Message + 'a + Clone),
        on_select: &(impl Fn(String) -> Message + 'a + Clone),
        depth: u32,
    ) -> Element<'a, Message> {
        let text_regular = Color::from(theme.neutral.text_regular);
        let text_disabled = Color::from(theme.neutral.text_disabled);
        let primary = Color::from(theme.primary.base);

        let is_expanded = tree.is_expanded(&node.id);
        let is_disabled = node.disabled;
        let label_color = if is_disabled {
            text_disabled
        } else {
            text_regular
        };

        // 展开/折叠指示器
        let indicator_str = if node.is_leaf() {
            if node.lazy { "…" } else { "•" }
        } else if is_expanded {
            "▼"
        } else {
            "▶"
        };

        let mut row_children: Vec<Element<'a, Message>> = Vec::new();
        if depth > 0 {
            row_children
                .push(iced::widget::Space::with_width(Length::Fixed(depth as f32 * 16.0)).into());
        }

        // 指示器按钮（点击切换展开/折叠）
        let indicator_text = text(indicator_str).color(primary).size(12.0);
        let mut indicator_btn = button(indicator_text)
            .padding(Padding::from([2u16, 4u16]))
            .style(move |_t, _status| iced::widget::button::Style {
                background: None,
                text_color: primary,
                border: iced::Border::default(),
                shadow: iced::Shadow::default(),
            });
        if !node.is_leaf() && !is_disabled {
            indicator_btn = indicator_btn.on_press((on_toggle)(node.id.clone()));
        }
        row_children.push(indicator_btn.into());

        // 标签按钮（点击选中）
        let label_text = text(node.label.clone()).color(label_color).size(14.0);
        let mut label_btn = button(label_text)
            .padding(Padding::from([4u16, 8u16]))
            .style(move |_t, _status| iced::widget::button::Style {
                background: None,
                text_color: label_color,
                border: iced::Border::default(),
                shadow: iced::Shadow::default(),
            });
        if !is_disabled {
            label_btn = label_btn.on_press((on_select)(node.id.clone()));
        }
        row_children.push(label_btn.into());

        let row = iced::widget::Row::with_children(row_children)
            .align_y(iced::Alignment::Center)
            .spacing(0);

        let mut col_children: Vec<Element<'a, Message>> = Vec::new();
        col_children.push(row.into());

        // 展开时渲染子节点
        if is_expanded && !node.is_leaf() {
            for child in &node.children {
                col_children.push(Self::render_node(
                    child,
                    tree,
                    theme,
                    on_toggle,
                    on_select,
                    depth + 1,
                ));
            }
        }

        iced::widget::Column::with_children(col_children)
            .spacing(0)
            .into()
    }
}

/// 递归收集可见节点（考虑展开状态和过滤）
fn collect_visible<'a>(node: &'a TreeNode, tree: &Tree, filter: &str, out: &mut Vec<&'a TreeNode>) {
    let filter_match = if filter.is_empty() {
        true
    } else {
        node_matches(node, filter)
    };
    let descendant_match = if filter.is_empty() {
        false
    } else {
        any_descendant_matches(node, filter)
    };

    // 节点可见条件：
    //  - 无过滤：父链展开则可见
    //  - 有过滤：自身或后代匹配，且父链展开
    if filter.is_empty() {
        out.push(node);
        if tree.is_expanded(&node.id) && !node.is_leaf() {
            for c in &node.children {
                collect_visible(c, tree, filter, out);
            }
        }
    } else {
        // 过滤模式：默认全部展开（按 Element Plus 行为）
        if filter_match || descendant_match {
            out.push(node);
            for c in &node.children {
                collect_visible(c, tree, filter, out);
            }
        }
    }
}

fn node_matches(node: &TreeNode, filter: &str) -> bool {
    let f = filter.to_lowercase();
    node.label.to_lowercase().contains(&f) || node.id.to_lowercase().contains(&f)
}

fn any_descendant_matches(node: &TreeNode, filter: &str) -> bool {
    for c in &node.children {
        if node_matches(c, filter) || any_descendant_matches(c, filter) {
            return true;
        }
    }
    false
}

/// 可变查找节点
fn find_node_mut<'a>(nodes: &'a mut [TreeNode], id: &str) -> Option<&'a mut TreeNode> {
    for n in nodes.iter_mut() {
        if n.id == id {
            return Some(n);
        }
        if let Some(found) = find_node_mut(&mut n.children, id) {
            return Some(found);
        }
    }
    None
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_tree_node_default() {
        let n = TreeNode::new("a", "A");
        assert_eq!(n.id(), "a");
        assert_eq!(n.label(), "A");
        assert!(n.is_leaf());
        assert!(!n.disabled());
        assert!(!n.lazy());
    }

    #[test]
    fn test_tree_node_lazy_flag() {
        let n = TreeNode::new_lazy("a", "A");
        assert!(n.lazy());
        // 懒加载节点虽然 children 为空但不视为 leaf
        assert!(!n.is_leaf());
    }
}
