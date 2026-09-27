//! Tree view() 测试 — TDD RED 阶段

use har_ui_components::tree::{Tree, TreeMessage, TreeNode};
use har_ui_core::theme::Theme;

fn build_sample_tree() -> Vec<TreeNode> {
    vec![
        TreeNode::new("root1", "Root 1")
            .with_child(TreeNode::new("child1-1", "Child 1-1"))
            .with_child(TreeNode::new("child1-2", "Child 1-2")),
        TreeNode::new("root2", "Root 2"),
    ]
}

#[test]
fn test_tree_view_empty_renders() {
    let theme = Theme::element_light();
    let t = Tree::new();
    let _element = t.view(&theme, |_| (), |_| ());
}

#[test]
fn test_tree_view_with_data_renders() {
    let theme = Theme::element_light();
    let t = Tree::new().with_data(build_sample_tree());
    let _element = t.view(&theme, |_| (), |_| ());
}

#[test]
fn test_tree_view_expanded_node_renders() {
    let theme = Theme::element_light();
    let mut t = Tree::new().with_data(build_sample_tree());
    t.handle(TreeMessage::ToggleExpand("root1".to_string()));
    let _element = t.view(&theme, |_| (), |_| ());
}

#[test]
fn test_tree_view_default_expand_all_renders() {
    let theme = Theme::element_light();
    let t = Tree::new()
        .with_default_expand_all(true)
        .with_data(build_sample_tree());
    let _element = t.view(&theme, |_| (), |_| ());
}

#[test]
fn test_tree_view_disabled_node_renders() {
    let theme = Theme::element_light();
    let t = Tree::new().with_data(vec![
        TreeNode::new("a", "A").with_disabled(true),
        TreeNode::new("b", "B"),
    ]);
    let _element = t.view(&theme, |_| (), |_| ());
}

#[test]
fn test_tree_view_with_checkbox_renders() {
    let theme = Theme::element_light();
    let t = Tree::new()
        .with_show_checkbox(true)
        .with_data(build_sample_tree());
    let _element = t.view(&theme, |_| (), |_| ());
}

#[test]
fn test_tree_view_with_lazy_node_renders() {
    let theme = Theme::element_light();
    let t = Tree::new().with_data(vec![TreeNode::new_lazy("lazy1", "Lazy 1")]);
    let _element = t.view(&theme, |_| (), |_| ());
}

#[test]
fn test_tree_view_filtered_renders() {
    let theme = Theme::element_light();
    let mut t = Tree::new().with_data(build_sample_tree());
    t.handle(TreeMessage::Filter("child1".to_string()));
    let _element = t.view(&theme, |_| (), |_| ());
}

#[test]
fn test_tree_view_deep_nested_renders() {
    let theme = Theme::element_light();
    let deep = TreeNode::new("a", "A").with_child(
        TreeNode::new("b", "B")
            .with_child(TreeNode::new("c", "C").with_child(TreeNode::new("d", "D"))),
    );
    let mut t = Tree::new().with_data(vec![deep]);
    t.handle(TreeMessage::ToggleExpand("a".to_string()));
    t.handle(TreeMessage::ToggleExpand("b".to_string()));
    t.handle(TreeMessage::ToggleExpand("c".to_string()));
    let _element = t.view(&theme, |_| (), |_| ());
}

#[test]
fn test_tree_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let t = Tree::new().with_data(build_sample_tree());
    let _element = t.view(&theme, |_| (), |_| ());
}
