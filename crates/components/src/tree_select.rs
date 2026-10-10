//! TreeSelect 树形选择 — 参考 Element Plus `<el-tree-select>`
//!
//! 支持：单选（勾选镜像语义：最近勾选的节点即选中项）、面板开合、清空、
//! 禁用、树消息透传（展开/过滤/懒加载）。
//!
//! ## 选择语义说明
//! Tree 组件无"节点点击"消息，单选采用**勾选镜像**：`ToggleCheck(id)` 透传给
//! 内部 Tree 的同时，把最近勾选节点镜像为选中项（`selected`）。
//! 应用侧也可直接发 `Choose(id)` 程序化选中。

use crate::tree::{Tree, TreeMessage, TreeNode};
use har_ui_core::theme::Theme;
use iced::widget::{container, text};
use iced::{Color, Element, Length, Padding};

/// TreeSelect 消息
#[derive(Debug, Clone)]
pub enum TreeSelectMessage {
    /// 开/合选择面板（disabled 时忽略）
    TogglePanel,
    /// 关闭面板
    Close,
    /// 程序化选中：节点 id（在树中递归查找，未找到忽略）
    Choose(String),
    /// 清空选中
    Clear,
    /// 透传给内部 Tree（展开/过滤/懒加载）；
    /// 其中 ToggleCheck 会镜像为选中项（勾选即选择）
    TreeMsg(TreeMessage),
}

/// TreeSelect 组件（单选）
#[derive(Debug, Clone)]
pub struct TreeSelect {
    tree: Tree,
    selected: Option<(String, String)>, // (id, label)
    panel_visible: bool,
    placeholder: String,
    disabled: bool,
    clearable: bool,
}

impl Default for TreeSelect {
    fn default() -> Self {
        Self::new()
    }
}

impl TreeSelect {
    pub fn new() -> Self {
        Self {
            tree: Tree::new().with_default_expand_all(false),
            selected: None,
            panel_visible: false,
            placeholder: "请选择".to_string(),
            disabled: false,
            clearable: true,
        }
    }

    pub fn with_data(mut self, roots: Vec<TreeNode>) -> Self {
        self.tree = self.tree.with_data(roots).with_show_checkbox(true);
        self
    }

    pub fn with_placeholder(mut self, p: impl Into<String>) -> Self {
        self.placeholder = p.into();
        self
    }

    pub fn with_disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }

    pub fn with_clearable(mut self, v: bool) -> Self {
        self.clearable = v;
        self
    }

    pub fn tree(&self) -> &Tree {
        &self.tree
    }

    /// 选中项 (id, label)
    pub fn selected(&self) -> Option<(&str, &str)> {
        self.selected
            .as_ref()
            .map(|(id, label)| (id.as_str(), label.as_str()))
    }

    pub fn selected_id(&self) -> Option<&str> {
        self.selected.as_ref().map(|(id, _)| id.as_str())
    }

    pub fn panel_visible(&self) -> bool {
        self.panel_visible
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    /// 递归查找节点（id → label）
    fn find_label(roots: &[TreeNode], id: &str) -> Option<String> {
        for node in roots {
            if node.id() == id {
                return Some(node.label().to_string());
            }
            if let Some(found) = Self::find_label(node.children(), id) {
                return Some(found);
            }
        }
        None
    }

    /// 处理消息
    pub fn handle(&mut self, msg: TreeSelectMessage) {
        match msg {
            TreeSelectMessage::TogglePanel => {
                if !self.disabled {
                    self.panel_visible = !self.panel_visible;
                }
            }
            TreeSelectMessage::Close => {
                self.panel_visible = false;
            }
            TreeSelectMessage::Choose(id) => {
                if let Some(label) = Self::find_label(self.tree.roots(), &id) {
                    self.selected = Some((id, label));
                    self.panel_visible = false;
                }
            }
            TreeSelectMessage::Clear => {
                self.selected = None;
            }
            TreeSelectMessage::TreeMsg(inner) => {
                // 勾选镜像：ToggleCheck(id) → 选中该节点
                if let TreeMessage::ToggleCheck(id) = &inner
                    && let Some(label) = Self::find_label(self.tree.roots(), id)
                {
                    self.selected = Some((id.clone(), label));
                }
                self.tree.handle(inner);
            }
        }
    }

    /// 渲染：触发器（选中 label / placeholder + 箭头）+ 可见面板（内部 Tree）
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        on_msg: impl Fn(TreeSelectMessage) -> Message + Clone + 'a,
    ) -> Element<'a, Message> {
        let text_primary = Color::from(theme.neutral.text_primary);
        let text_placeholder = Color::from(theme.neutral.text_placeholder);
        let border = Color::from(theme.neutral.border_lighter);
        let bg = Color::from(theme.neutral.bg_overlay);

        let (label, is_placeholder) = match &self.selected {
            Some((_, label)) => (label.clone(), false),
            None => (self.placeholder.clone(), true),
        };
        let caret = if self.panel_visible { "▲" } else { "▼" };

        let trigger = container(
            iced::widget::Row::new()
                .push(text(label).size(14).color(if is_placeholder {
                    text_placeholder
                } else {
                    text_primary
                }))
                .push(iced::widget::Space::new().width(Length::Fill))
                .push(text(caret).size(10).color(text_placeholder)),
        )
        .width(Length::Fill)
        .padding(Padding::from([8u16, 12u16]))
        .style(move |_t| iced::widget::container::Style {
            text_color: None,
            background: Some(iced::Background::Color(bg)),
            border: iced::Border {
                color: border,
                width: 1.0,
                radius: iced::border::Radius::default(),
            },
            shadow: iced::Shadow::default(),
            snap: false,
        });

        let mut col = iced::widget::Column::new().spacing(4);
        col = col.push(trigger);
        if self.panel_visible {
            let tree_elem = self
                .tree
                .view(
                    theme,
                    |id| TreeSelectMessage::TreeMsg(TreeMessage::ToggleExpand(id)),
                    TreeSelectMessage::Choose,
                )
                .map(on_msg);
            col = col.push(
                container(tree_elem)
                    .width(Length::Fill)
                    .height(Length::Fixed(240.0))
                    .style(move |_t| iced::widget::container::Style {
                        text_color: None,
                        background: Some(iced::Background::Color(bg)),
                        border: iced::Border {
                            color: border,
                            width: 1.0,
                            radius: iced::border::radius(4.0),
                        },
                        shadow: iced::Shadow::default(),
                        snap: false,
                    }),
            );
        }
        col.into()
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    fn sample_tree() -> Vec<TreeNode> {
        vec![
            TreeNode::new("1", "华东")
                .with_child(TreeNode::new("1-1", "上海").with_child(TreeNode::new("1-1-1", "浦东")))
                .with_child(TreeNode::new("1-2", "杭州")),
            TreeNode::new("2", "华北").with_child(TreeNode::new("2-1", "北京")),
        ]
    }

    fn sample() -> TreeSelect {
        TreeSelect::new().with_data(sample_tree())
    }

    #[test]
    fn test_placeholder_initial() {
        let ts = sample();
        assert_eq!(ts.selected(), None);
        assert!(!ts.panel_visible());
        assert_eq!(ts.selected_id(), None);
    }

    #[test]
    fn test_toggle_panel_and_disabled() {
        let mut ts = sample();
        ts.handle(TreeSelectMessage::TogglePanel);
        assert!(ts.panel_visible());
        ts.handle(TreeSelectMessage::TogglePanel);
        assert!(!ts.panel_visible());
        // disabled 阻止开面板
        let mut d = sample().with_disabled(true);
        d.handle(TreeSelectMessage::TogglePanel);
        assert!(!d.panel_visible());
        // Close 任何时候可关
        let mut o = sample();
        o.handle(TreeSelectMessage::TogglePanel);
        o.handle(TreeSelectMessage::Close);
        assert!(!o.panel_visible());
    }

    #[test]
    fn test_choose_by_id_finds_nested() {
        let mut ts = sample();
        ts.handle(TreeSelectMessage::Choose("1-1-1".to_string()));
        assert_eq!(ts.selected(), Some(("1-1-1", "浦东")));
        assert!(!ts.panel_visible(), "选中后面板关闭");
        // 不存在的 id 忽略
        ts.handle(TreeSelectMessage::Choose("9-9".to_string()));
        assert_eq!(ts.selected_id(), Some("1-1-1"));
    }

    #[test]
    fn test_check_mirrors_selection() {
        let mut ts = sample();
        ts.handle(TreeSelectMessage::TreeMsg(TreeMessage::ToggleCheck(
            "2-1".to_string(),
        )));
        assert_eq!(ts.selected(), Some(("2-1", "北京")));
        // 最近勾选覆盖
        ts.handle(TreeSelectMessage::TreeMsg(TreeMessage::ToggleCheck(
            "1-2".to_string(),
        )));
        assert_eq!(ts.selected(), Some(("1-2", "杭州")));
    }

    #[test]
    fn test_tree_msg_forwarded() {
        let mut ts = sample();
        ts.handle(TreeSelectMessage::TreeMsg(TreeMessage::ToggleExpand(
            "1".to_string(),
        )));
        // 透传后内部树展开状态变化（经 tree 内部状态断言——此处确认无 panic 且选中不受影响）
        assert_eq!(ts.selected(), None);
    }

    #[test]
    fn test_clear() {
        let mut ts = sample();
        ts.handle(TreeSelectMessage::Choose("2".to_string()));
        assert!(ts.selected().is_some());
        ts.handle(TreeSelectMessage::Clear);
        assert_eq!(ts.selected(), None);
    }

    #[test]
    fn test_clearable_default_on() {
        assert!(TreeSelect::new().clearable);
    }
}
