//! Tree 树形组件 — 参考 Element Plus `<el-tree>`。
//!
//! 覆盖：基础树/展开折叠/复选框/父子联动/半选/过滤/懒加载/可见节点/1000 节点性能。

use har_ui_components::tree::{Tree, TreeNode, TreeMessage};
use std::time::Instant;

// ---------- 工具：构造测试用树 ----------

/// 构造一棵 3 层 × 3 子节点的小树：
/// root
/// ├── a
/// │   ├── a1
/// │   └── a2
/// └── b
///     └── b1
fn sample_tree() -> Vec<TreeNode> {
    vec![TreeNode::new("root", "Root")
        .with_child(TreeNode::new("a", "A")
            .with_child(TreeNode::new("a1", "A1"))
            .with_child(TreeNode::new("a2", "A2")))
        .with_child(TreeNode::new("b", "B")
            .with_child(TreeNode::new("b1", "B1")))]
}

// ---------- 基础构造 ----------

#[test]
fn test_tree_empty() {
    let t = Tree::new();
    assert!(t.roots().is_empty());
    assert!(t.visible_nodes().is_empty());
}

#[test]
fn test_tree_with_data() {
    let t = Tree::new().with_data(sample_tree());
    assert_eq!(t.roots().len(), 1);
    // 默认折叠，只有根节点可见
    assert_eq!(t.visible_nodes().len(), 1);
    assert_eq!(t.visible_nodes()[0].id(), "root");
}

// ---------- 展开 / 折叠 ----------

#[test]
fn test_tree_toggle_expand() {
    let mut t = Tree::new().with_data(sample_tree());
    // 默认 root 折叠
    assert!(!t.is_expanded("root"));
    assert_eq!(t.visible_nodes().len(), 1);

    // 展开 root
    t.handle(TreeMessage::ToggleExpand("root".to_string()));
    assert!(t.is_expanded("root"));
    // root + a + b = 3 个可见
    assert_eq!(t.visible_nodes().len(), 3);

    // 展开 a
    t.handle(TreeMessage::ToggleExpand("a".to_string()));
    // root + a + a1 + a2 + b = 5
    assert_eq!(t.visible_nodes().len(), 5);

    // 折叠 a
    t.handle(TreeMessage::ToggleExpand("a".to_string()));
    assert!(!t.is_expanded("a"));
    assert_eq!(t.visible_nodes().len(), 3);
}

#[test]
fn test_tree_default_expand_all() {
    let t = Tree::new().with_data(sample_tree()).with_default_expand_all(true);
    // 全部展开：root + a + a1 + a2 + b + b1 = 6
    assert_eq!(t.visible_nodes().len(), 6);
}

// ---------- 复选框 ----------

#[test]
fn test_tree_checkbox_toggle() {
    let mut t = Tree::new()
        .with_data(sample_tree())
        .with_show_checkbox(true);
    // 默认无选中
    assert!(t.checked().is_empty());
    assert!(!t.is_checked("root"));

    // 勾选 root
    t.handle(TreeMessage::ToggleCheck("root".to_string()));
    // 父选中 → 所有子节点（递归）全部选中
    assert!(t.is_checked("root"));
    assert!(t.is_checked("a"));
    assert!(t.is_checked("a1"));
    assert!(t.is_checked("b"));
    assert!(t.is_checked("b1"));

    // 再次点击 root 取消
    t.handle(TreeMessage::ToggleCheck("root".to_string()));
    assert!(!t.is_checked("root"));
    assert!(!t.is_checked("a1"));
}

#[test]
fn test_tree_checkbox_partial_check_parent() {
    // 子部分选中 → 父半选
    let mut t = Tree::new()
        .with_data(sample_tree())
        .with_show_checkbox(true);

    // 勾选 a1（叶子）
    t.handle(TreeMessage::ToggleCheck("a1".to_string()));
    assert!(t.is_checked("a1"));
    assert!(!t.is_checked("a"));          // a 未全选
    assert!(t.is_indeterminate("a"));     // a 半选
    assert!(!t.is_checked("root"));
    assert!(t.is_indeterminate("root"));  // root 半选

    // 再勾选 a2 → a 全选，root 仍半选（b 未选）
    t.handle(TreeMessage::ToggleCheck("a2".to_string()));
    assert!(t.is_checked("a"));
    assert!(!t.is_indeterminate("a"));
    assert!(t.is_indeterminate("root"));
}

#[test]
fn test_tree_checkbox_disabled_not_checkable() {
    let data = vec![TreeNode::new("root", "Root")
        .with_disabled(true)
        .with_child(TreeNode::new("a", "A"))];
    let mut t = Tree::new().with_data(data).with_show_checkbox(true);

    // 勾选 disabled 节点 → 无效
    t.handle(TreeMessage::ToggleCheck("root".to_string()));
    assert!(!t.is_checked("root"));
    assert!(t.checked().is_empty());
}

// ---------- 过滤 ----------

#[test]
fn test_tree_filter_matching() {
    let mut t = Tree::new().with_data(sample_tree()).with_default_expand_all(true);
    // 过滤 "a"：保留 a、a1、a2，以及路径上的 root
    t.handle(TreeMessage::Filter("a".to_string()));

    let visible: Vec<&str> = t.visible_nodes().iter().map(|n| n.id()).collect();
    assert!(visible.contains(&"root")); // 路径保留
    assert!(visible.contains(&"a"));
    assert!(visible.contains(&"a1"));
    assert!(visible.contains(&"a2"));
    assert!(!visible.contains(&"b"));  // 不匹配
    assert!(!visible.contains(&"b1"));
}

#[test]
fn test_tree_filter_clear() {
    let mut t = Tree::new().with_data(sample_tree()).with_default_expand_all(true);
    t.handle(TreeMessage::Filter("a".to_string()));
    assert!(t.is_filtered());

    // 空字符串清除过滤
    t.handle(TreeMessage::Filter(String::new()));
    assert!(!t.is_filtered());
    assert_eq!(t.visible_nodes().len(), 6);
}

// ---------- 懒加载 ----------

#[test]
fn test_tree_lazy_load_children() {
    let mut t = Tree::new().with_data(vec![
        TreeNode::new_lazy("root", "Root"),
    ]);

    // root 是 lazy 节点，无 children
    assert!(t.is_lazy("root"));
    assert_eq!(t.visible_nodes().len(), 1);

    // 模拟懒加载：设置子节点
    t.handle(TreeMessage::LoadChildren("root".to_string(), vec![
        TreeNode::new("a", "A"),
        TreeNode::new("b", "B"),
    ]));
    assert!(!t.is_lazy("root"));

    // 展开 root
    t.handle(TreeMessage::ToggleExpand("root".to_string()));
    assert_eq!(t.visible_nodes().len(), 3);
}

// ---------- 1000 节点性能 ----------

/// 构造 1 个根 + 1000 个直接子节点（共 1001 节点）
fn big_tree() -> Vec<TreeNode> {
    let children: Vec<TreeNode> = (0..1000)
        .map(|i| TreeNode::new(format!("child_{i}"), format!("Child {i}")))
        .collect();
    let mut root = TreeNode::new("root", "Root");
    for c in children {
        root = root.with_child(c);
    }
    vec![root]
}

#[test]
fn test_tree_1000_nodes_expand_under_100ms() {
    let mut t = Tree::new().with_data(big_tree());
    let start = Instant::now();
    t.handle(TreeMessage::ToggleExpand("root".to_string()));
    let elapsed = start.elapsed();

    assert!(t.is_expanded("root"));
    assert_eq!(t.visible_nodes().len(), 1001); // root + 1000 children
    // 性能预算：1000 节点展开 < 100ms
    assert!(elapsed.as_millis() < 100,
        "expand 1000 nodes took {:?}, expected < 100ms", elapsed);
}

// ---------- 自定义节点内容 ----------

#[test]
fn test_tree_custom_node_data() {
    // 通过 with_data 额外携带自定义字段
    let node = TreeNode::new("root", "Root")
        .with_extra("icon", "folder");
    assert_eq!(node.extra("icon"), Some("folder"));
    assert_eq!(node.extra("missing"), None);
}
