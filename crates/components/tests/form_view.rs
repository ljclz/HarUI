//! R.2.P0.6 Form view() 测试 — TDD RED 阶段
//!
//! 验证 Form view() 正确渲染 label + 字段占位 + 错误信息。

use har_ui_components::form::{Form, FormItem, FormMessage, FormRule};
use har_ui_core::theme::Theme;
use iced::widget::text;
use iced::Element;

fn field_renderer<'a>(_item: &har_ui_components::form::FormItem) -> Element<'a, ()> {
    text("input").into()
}

// ============== R.2.P0.6.a view() 基础渲染 ==============

#[test]
fn test_form_view_empty_renders() {
    let theme = Theme::element_light();
    let form = Form::new();
    let _element = form.view(&theme, field_renderer);
}

#[test]
fn test_form_view_with_items_renders() {
    let theme = Theme::element_light();
    let form = Form::new()
        .with_item(FormItem::new("name", "Name"))
        .with_item(FormItem::new("age", "Age"));
    let _element = form.view(&theme, field_renderer);
}

#[test]
fn test_form_view_with_required_item_renders() {
    let theme = Theme::element_light();
    let form = Form::new()
        .with_item(
            FormItem::new("name", "Name")
                .with_rule(FormRule::new("name").required(true)),
        );
    let _element = form.view(&theme, field_renderer);
}

#[test]
fn test_form_view_inline_renders() {
    let theme = Theme::element_light();
    let form = Form::new()
        .with_inline(true)
        .with_item(FormItem::new("a", "A"))
        .with_item(FormItem::new("b", "B"));
    let _element = form.view(&theme, field_renderer);
}

#[test]
fn test_form_view_label_left_renders() {
    let theme = Theme::element_light();
    let form = Form::new()
        .with_label_position("left")
        .with_item(FormItem::new("a", "A"));
    let _element = form.view(&theme, field_renderer);
}

#[test]
fn test_form_view_label_top_renders() {
    let theme = Theme::element_light();
    let form = Form::new()
        .with_label_position("top")
        .with_item(FormItem::new("a", "A"));
    let _element = form.view(&theme, field_renderer);
}

#[test]
fn test_form_view_with_label_width_renders() {
    let theme = Theme::element_light();
    let form = Form::new()
        .with_label_width(120)
        .with_item(FormItem::new("a", "A"));
    let _element = form.view(&theme, field_renderer);
}

// ============== R.2.P0.6.b 验证状态 ==============

#[test]
fn test_form_view_with_validation_errors_renders() {
    let theme = Theme::element_light();
    let mut form = Form::new()
        .with_item(
            FormItem::new("name", "Name")
                .with_rule(FormRule::new("name").required(true).with_message("Name required")),
        );
    let _ = form.validate();
    assert!(!form.errors().is_empty());
    let _element = form.view(&theme, field_renderer);
}

#[test]
fn test_form_view_passed_state_renders() {
    let theme = Theme::element_light();
    let mut form = Form::new()
        .with_item(FormItem::new("name", "Name"));
    form.set_value("name", "Alice");
    let _ = form.validate();
    let _element = form.view(&theme, field_renderer);
}

#[test]
fn test_form_view_after_reset_renders() {
    let theme = Theme::element_light();
    let mut form = Form::new()
        .with_item(
            FormItem::new("name", "Name")
                .with_rule(FormRule::new("name").required(true)),
        );
    let _ = form.validate();
    form.handle(FormMessage::Reset);
    let _element = form.view(&theme, field_renderer);
}

// ============== R.2.P0.6.c 主题变体 ==============

#[test]
fn test_form_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let form = Form::new()
        .with_item(FormItem::new("a", "A"))
        .with_item(FormItem::new("b", "B"));
    let _element = form.view(&theme, field_renderer);
}

#[test]
fn test_form_view_many_items_renders() {
    let theme = Theme::element_light();
    let form = Form::new()
        .with_item(FormItem::new("a", "A"))
        .with_item(FormItem::new("b", "B"))
        .with_item(FormItem::new("c", "C"))
        .with_item(FormItem::new("d", "D"))
        .with_item(FormItem::new("e", "E"));
    let _element = form.view(&theme, field_renderer);
}
