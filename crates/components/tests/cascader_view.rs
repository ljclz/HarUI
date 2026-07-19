//! R.2.P1.19 Cascader view() 测试

use har_ui_components::cascader::{Cascader, CascaderMessage, CascaderNode, ExpandTrigger};
use har_ui_core::theme::Theme;

fn sample_options() -> Vec<CascaderNode> {
    vec![CascaderNode::new("a", "A").with_children(vec![
        CascaderNode::new("a1", "A1"),
        CascaderNode::new("a2", "A2").with_children(vec![
            CascaderNode::new("a2x", "A2X"),
            CascaderNode::new("a2y", "A2Y"),
        ]),
    ])]
}

#[test]
fn test_cascader_view_default_renders() {
    let theme = Theme::element_light();
    let c = Cascader::new();
    let _element = c.view(&theme, |_| ());
}

#[test]
fn test_cascader_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let c = Cascader::new();
    let _element = c.view(&theme, |_| ());
}

#[test]
fn test_cascader_view_with_options_renders() {
    let theme = Theme::element_light();
    let c = Cascader::new().with_options(sample_options());
    let _element = c.view(&theme, |_| ());
}

#[test]
fn test_cascader_view_open_panel_renders() {
    let theme = Theme::element_light();
    let mut c = Cascader::new().with_options(sample_options());
    c.handle(CascaderMessage::TogglePanel);
    let _element = c.view(&theme, |_| ());
}

#[test]
fn test_cascader_view_with_selected_path_renders() {
    let theme = Theme::element_light();
    let mut c = Cascader::new().with_options(sample_options());
    c.handle(CascaderMessage::Select("a1".to_string()));
    let _element = c.view(&theme, |_| ());
}

#[test]
fn test_cascader_view_open_with_selected_path_renders() {
    let theme = Theme::element_light();
    let mut c = Cascader::new().with_options(sample_options());
    c.handle(CascaderMessage::Select("a2y".to_string()));
    c.handle(CascaderMessage::TogglePanel);
    let _element = c.view(&theme, |_| ());
}

#[test]
fn test_cascader_view_deep_path_renders() {
    let theme = Theme::element_light();
    let mut c = Cascader::new().with_options(sample_options());
    c.handle(CascaderMessage::Select("a2y".to_string()));
    c.handle(CascaderMessage::TogglePanel);
    let _element = c.view(&theme, |_| ());
}

#[test]
fn test_cascader_view_check_strictly_renders() {
    let theme = Theme::element_light();
    let c = Cascader::new()
        .with_options(sample_options())
        .with_check_strictly(true);
    let _element = c.view(&theme, |_| ());
}

#[test]
fn test_cascader_view_emit_path_false_renders() {
    let theme = Theme::element_light();
    let c = Cascader::new()
        .with_options(sample_options())
        .with_emit_path(false);
    let _element = c.view(&theme, |_| ());
}

#[test]
fn test_cascader_view_hover_trigger_renders() {
    let theme = Theme::element_light();
    let c = Cascader::new()
        .with_options(sample_options())
        .with_expand_trigger(ExpandTrigger::Hover);
    let _element = c.view(&theme, |_| ());
}

#[test]
fn test_cascader_view_disabled_node_renders() {
    let theme = Theme::element_light();
    let opts = vec![CascaderNode::new("a", "A")
        .with_disabled(true)
        .with_children(vec![CascaderNode::new("a1", "A1")])];
    let c = Cascader::new().with_options(opts);
    let _element = c.view(&theme, |_| ());
}

#[test]
fn test_cascader_view_open_dark_theme_renders() {
    let theme = Theme::element_dark();
    let mut c = Cascader::new().with_options(sample_options());
    c.handle(CascaderMessage::TogglePanel);
    let _element = c.view(&theme, |_| ());
}

#[test]
fn test_cascader_view_with_custom_message_type() {
    let theme = Theme::element_light();
    let c = Cascader::new().with_options(sample_options());
    #[derive(Clone, Debug)]
    #[allow(dead_code)]
    enum AppMsg {
        Select(Vec<String>),
    }
    let _element = c.view(&theme, |p| AppMsg::Select(p));
}
