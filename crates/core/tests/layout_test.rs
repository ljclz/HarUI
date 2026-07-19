//! R.4 layout 模块测试 — col / row / container 三个构造函数
//!
//! 验证三个 pub fn 返回 iced::Element，覆盖空 children、单 child、
//! 多 child、嵌套组合、自定义 Message 类型等场景。

use har_ui_core::layout::{col, container, row};

#[test]
fn test_col_returns_element_empty() {
    let children: Vec<iced::Element<()>> = vec![];
    let element = col(children);
    let _ = element;
}

#[test]
fn test_col_with_single_child() {
    let child: iced::Element<()> = iced::widget::text("hello").into();
    let element = col(vec![child]);
    let _ = element;
}

#[test]
fn test_col_with_multiple_children() {
    let children: Vec<iced::Element<()>> = vec![
        iced::widget::text("a").into(),
        iced::widget::text("b").into(),
        iced::widget::text("c").into(),
    ];
    let element = col(children);
    let _ = element;
}

#[test]
fn test_row_returns_element_empty() {
    let children: Vec<iced::Element<()>> = vec![];
    let element = row(children);
    let _ = element;
}

#[test]
fn test_row_with_single_child() {
    let child: iced::Element<()> = iced::widget::text("hello").into();
    let element = row(vec![child]);
    let _ = element;
}

#[test]
fn test_row_with_multiple_children() {
    let children: Vec<iced::Element<()>> = vec![
        iced::widget::text("a").into(),
        iced::widget::text("b").into(),
        iced::widget::text("c").into(),
    ];
    let element = row(children);
    let _ = element;
}

#[test]
fn test_container_returns_element() {
    let content: iced::Element<()> = iced::widget::text("hello").into();
    let element = container(content);
    let _ = element;
}

#[test]
fn test_container_wrapping_col() {
    let inner: iced::Element<()> = col(vec![
        iced::widget::text("top").into(),
        iced::widget::text("bottom").into(),
    ]);
    let element = container(inner);
    let _ = element;
}

#[test]
fn test_row_containing_col_nested() {
    let col_element: iced::Element<()> =
        col(vec![iced::widget::text("inside col").into()]);
    let element = row(vec![col_element, iced::widget::text("sibling").into()]);
    let _ = element;
}

#[test]
fn test_col_containing_row_nested() {
    let row_element: iced::Element<()> =
        row(vec![iced::widget::text("inside row").into()]);
    let element = col(vec![row_element, iced::widget::text("below").into()]);
    let _ = element;
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
enum AppMessage {
    Click,
    Toggle,
}

#[test]
fn test_col_with_custom_message_type() {
    let children: Vec<iced::Element<AppMessage>> = vec![
        iced::widget::text("a").into(),
        iced::widget::text("b").into(),
    ];
    let element = col(children);
    let _ = element;
}

#[test]
fn test_row_with_custom_message_type() {
    let children: Vec<iced::Element<AppMessage>> =
        vec![iced::widget::text("only").into()];
    let element = row(children);
    let _ = element;
}

#[test]
fn test_container_with_custom_message_type() {
    let content: iced::Element<AppMessage> = iced::widget::text("custom").into();
    let element = container(content);
    let _ = element;
}

#[test]
fn test_nested_container_in_col_in_row() {
    let inner_container: iced::Element<()> =
        container(iced::widget::text("deep").into());
    let col_element: iced::Element<()> = col(vec![inner_container]);
    let element = row(vec![col_element]);
    let _ = element;
}
