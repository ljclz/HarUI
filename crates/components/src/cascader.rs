//! Cascader 级联选择 — 参考 Element Plus `<el-cascader>`。
//!
//! 支持：options 树、select 路径推导、emit_path、check_strictly、disabled、面板切换。

/// 展开触发方式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ExpandTrigger {
    #[default]
    Click,
    Hover,
}

/// Cascader 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CascaderMessage {
    /// 按 value 选中节点
    Select(String),
    /// 清空选择
    Clear,
    /// 切换面板可见
    TogglePanel,
}

/// Cascader 节点
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CascaderNode {
    value: String,
    label: String,
    disabled: bool,
    children: Vec<CascaderNode>,
}

impl CascaderNode {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
            children: Vec::new(),
        }
    }

    pub fn with_disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }

    pub fn with_children(mut self, children: Vec<CascaderNode>) -> Self {
        self.children = children;
        self
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn children(&self) -> &[CascaderNode] {
        &self.children
    }

    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
}

/// Cascader 组件
#[derive(Debug, Clone)]
pub struct Cascader {
    options: Vec<CascaderNode>,
    selected_path: Vec<String>,
    emit_path: bool,
    check_strictly: bool,
    expand_trigger: ExpandTrigger,
    panel_visible: bool,
}

impl Default for Cascader {
    fn default() -> Self {
        Self::new()
    }
}

impl Cascader {
    pub fn new() -> Self {
        Self {
            options: Vec::new(),
            selected_path: Vec::new(),
            emit_path: true,
            check_strictly: false,
            expand_trigger: ExpandTrigger::Click,
            panel_visible: false,
        }
    }

    pub fn with_options(mut self, opts: Vec<CascaderNode>) -> Self {
        self.options = opts;
        self
    }

    pub fn with_emit_path(mut self, v: bool) -> Self {
        self.emit_path = v;
        self
    }

    pub fn with_check_strictly(mut self, v: bool) -> Self {
        self.check_strictly = v;
        self
    }

    pub fn with_expand_trigger(mut self, t: ExpandTrigger) -> Self {
        self.expand_trigger = t;
        self
    }

    pub fn options(&self) -> &[CascaderNode] {
        &self.options
    }

    pub fn selected_path(&self) -> &[String] {
        &self.selected_path
    }

    pub fn emit_path(&self) -> bool {
        self.emit_path
    }

    pub fn check_strictly(&self) -> bool {
        self.check_strictly
    }

    pub fn expand_trigger(&self) -> ExpandTrigger {
        self.expand_trigger
    }

    pub fn panel_visible(&self) -> bool {
        self.panel_visible
    }

    /// 返回当前选中的 value
    /// emit_path=true 时返回末级 value，emit_path=false 时也返回末级 value
    /// （emit_path 仅影响对外回调的格式；这里 value() 始终返回末级）
    pub fn value(&self) -> Option<&str> {
        self.selected_path.last().map(|s| s.as_str())
    }

    pub fn handle(&mut self, msg: CascaderMessage) {
        match msg {
            CascaderMessage::Select(v) => {
                if let Some(path) = find_path(&self.options, &v) {
                    // 检查路径中是否有 disabled 节点
                    if path_disabled(&self.options, &path) {
                        return;
                    }
                    let target_is_leaf = path_target_is_leaf(&self.options, &path);
                    if !self.check_strictly && !target_is_leaf {
                        // 非 strict 模式只允许选叶子
                        return;
                    }
                    self.selected_path = path;
                    self.panel_visible = false;
                }
                // 不存在则无操作
            }
            CascaderMessage::Clear => {
                self.selected_path.clear();
            }
            CascaderMessage::TogglePanel => {
                self.panel_visible = !self.panel_visible;
            }
        }
    }
}

/// DFS 查找从根到指定 value 的路径
fn find_path(nodes: &[CascaderNode], target: &str) -> Option<Vec<String>> {
    for node in nodes {
        if node.value == target {
            return Some(vec![node.value.clone()]);
        }
        if !node.children.is_empty() {
            if let Some(mut sub) = find_path(&node.children, target) {
                let mut path = vec![node.value.clone()];
                path.append(&mut sub);
                return Some(path);
            }
        }
    }
    None
}

/// 判断路径中是否包含 disabled 节点
fn path_disabled(nodes: &[CascaderNode], path: &[String]) -> bool {
    if path.is_empty() {
        return false;
    }
    let head = &path[0];
    for node in nodes {
        if &node.value == head {
            if node.disabled {
                return true;
            }
            if path.len() > 1 {
                return path_disabled(&node.children, &path[1..]);
            }
            return false;
        }
    }
    false
}

/// 判断路径末端节点是否为叶子
fn path_target_is_leaf(nodes: &[CascaderNode], path: &[String]) -> bool {
    if path.is_empty() {
        return true;
    }
    let head = &path[0];
    for node in nodes {
        if &node.value == head {
            if path.len() == 1 {
                return node.is_leaf();
            }
            return path_target_is_leaf(&node.children, &path[1..]);
        }
    }
    true
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    fn opts() -> Vec<CascaderNode> {
        vec![CascaderNode::new("a", "A").with_children(vec![
            CascaderNode::new("a1", "A1"),
            CascaderNode::new("a2", "A2"),
        ])]
    }

    #[test]
    fn test_find_path_root() {
        let p = find_path(&opts(), "a").unwrap();
        assert_eq!(p, vec!["a"]);
    }

    #[test]
    fn test_find_path_leaf() {
        let p = find_path(&opts(), "a1").unwrap();
        assert_eq!(p, vec!["a", "a1"]);
    }

    #[test]
    fn test_find_path_missing() {
        assert!(find_path(&opts(), "x").is_none());
    }

    #[test]
    fn test_path_disabled() {
        let opts = vec![CascaderNode::new("a", "A")
            .with_disabled(true)
            .with_children(vec![CascaderNode::new("a1", "A1")])];
        assert!(path_disabled(&opts, &["a".to_string(), "a1".to_string()]));
    }

    #[test]
    fn test_target_is_leaf() {
        let opts = opts();
        assert!(!path_target_is_leaf(&opts, &["a".to_string()]));
        assert!(path_target_is_leaf(&opts, &["a".to_string(), "a1".to_string()]));
    }
}
