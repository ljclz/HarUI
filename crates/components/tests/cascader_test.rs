//! Cascader 级联选择 — 参考 Element Plus `<el-cascader>`。
//!
//! 覆盖：options、select、emit_path、check_strictly、disabled、clear、panel 切换。

use har_ui_components::cascader::{Cascader, CascaderMessage, CascaderNode, ExpandTrigger};

fn sample_options() -> Vec<CascaderNode> {
    vec![CascaderNode::new("zhejiang", "浙江").with_children(vec![
        CascaderNode::new("hangzhou", "杭州").with_children(vec![
            CascaderNode::new("xihu", "西湖"),
        ]),
        CascaderNode::new("ningbo", "宁波"),
    ])]
}

#[test]
fn test_cascader_default() {
    let c = Cascader::new();
    assert!(c.options().is_empty());
    assert!(c.selected_path().is_empty());
    assert!(c.emit_path());
    assert!(!c.check_strictly());
    assert_eq!(c.expand_trigger(), ExpandTrigger::Click);
    assert!(!c.panel_visible());
}

#[test]
fn test_cascader_with_options() {
    let c = Cascader::new().with_options(sample_options());
    assert_eq!(c.options().len(), 1);
    assert_eq!(c.options()[0].label(), "浙江");
    assert_eq!(c.options()[0].children().len(), 2);
}

#[test]
fn test_cascader_select_leaf_emits_path() {
    let mut c = Cascader::new().with_options(sample_options());
    c.handle(CascaderMessage::Select("xihu".into()));
    assert_eq!(c.selected_path(), &["zhejiang", "hangzhou", "xihu"]);
    assert_eq!(c.value(), Some("xihu"));
}

#[test]
fn test_cascader_emit_path_false_returns_leaf_only() {
    let mut c = Cascader::new()
        .with_options(sample_options())
        .with_emit_path(false);
    c.handle(CascaderMessage::Select("xihu".into()));
    assert_eq!(c.selected_path(), &["zhejiang", "hangzhou", "xihu"]);
    assert_eq!(c.value(), Some("xihu"));
}

#[test]
fn test_cascader_check_strictly_select_intermediate() {
    let mut c = Cascader::new()
        .with_options(sample_options())
        .with_check_strictly(true);
    c.handle(CascaderMessage::Select("hangzhou".into()));
    assert_eq!(c.selected_path(), &["zhejiang", "hangzhou"]);
    assert_eq!(c.value(), Some("hangzhou"));
}

#[test]
fn test_cascader_select_intermediate_without_strict_noop() {
    let mut c = Cascader::new().with_options(sample_options());
    c.handle(CascaderMessage::Select("hangzhou".into()));
    assert!(c.selected_path().is_empty());
    assert_eq!(c.value(), None);
}

#[test]
fn test_cascader_select_disabled_noop() {
    let opts = vec![
        CascaderNode::new("a", "A")
            .with_disabled(true)
            .with_children(vec![CascaderNode::new("a1", "A1")]),
    ];
    let mut c = Cascader::new().with_options(opts).with_check_strictly(true);
    c.handle(CascaderMessage::Select("a".into()));
    assert!(c.selected_path().is_empty());
}

#[test]
fn test_cascader_select_nonexistent_noop() {
    let mut c = Cascader::new().with_options(sample_options());
    c.handle(CascaderMessage::Select("nonexistent".into()));
    assert!(c.selected_path().is_empty());
    assert_eq!(c.value(), None);
}

#[test]
fn test_cascader_clear() {
    let mut c = Cascader::new().with_options(sample_options());
    c.handle(CascaderMessage::Select("xihu".into()));
    c.handle(CascaderMessage::Clear);
    assert!(c.selected_path().is_empty());
    assert_eq!(c.value(), None);
}

#[test]
fn test_cascader_toggle_panel() {
    let mut c = Cascader::new();
    assert!(!c.panel_visible());
    c.handle(CascaderMessage::TogglePanel);
    assert!(c.panel_visible());
    c.handle(CascaderMessage::TogglePanel);
    assert!(!c.panel_visible());
}

#[test]
fn test_cascader_with_expand_trigger_hover() {
    let c = Cascader::new().with_expand_trigger(ExpandTrigger::Hover);
    assert_eq!(c.expand_trigger(), ExpandTrigger::Hover);
}

#[test]
fn test_cascader_select_closes_panel() {
    let mut c = Cascader::new().with_options(sample_options());
    c.handle(CascaderMessage::TogglePanel);
    assert!(c.panel_visible());
    c.handle(CascaderMessage::Select("xihu".into()));
    assert!(!c.panel_visible());
}
