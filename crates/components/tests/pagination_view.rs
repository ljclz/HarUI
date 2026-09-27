//! R.2.P0.11 Pagination view() 测试 — TDD RED 阶段
//!
//! 验证 Pagination view() 正确渲染上一页/页码/下一页按钮。

use har_ui_components::pagination::{Pagination, PaginationMessage};
use har_ui_core::theme::Theme;

#[test]
fn test_pagination_view_single_page_renders() {
    let theme = Theme::element_light();
    let p = Pagination::new(5, 10);
    let _element = p.view(&theme, |_| ());
}

#[test]
fn test_pagination_view_multiple_pages_renders() {
    let theme = Theme::element_light();
    let p = Pagination::new(100, 10);
    let _element = p.view(&theme, |_| ());
}

#[test]
fn test_pagination_view_at_first_page_renders() {
    let theme = Theme::element_light();
    let p = Pagination::new(100, 10); // current=1
    let _element = p.view(&theme, |_| ());
}

#[test]
fn test_pagination_view_at_last_page_renders() {
    let theme = Theme::element_light();
    let mut p = Pagination::new(100, 10);
    p.handle(PaginationMessage::JumpTo(10));
    let _element = p.view(&theme, |_| ());
}

#[test]
fn test_pagination_view_at_middle_page_renders() {
    let theme = Theme::element_light();
    let mut p = Pagination::new(200, 10);
    p.handle(PaginationMessage::JumpTo(10));
    let _element = p.view(&theme, |_| ());
}

#[test]
fn test_pagination_view_with_ellipsis_renders() {
    let theme = Theme::element_light();
    let mut p = Pagination::new(1000, 10); // 100 页，必然有省略号
    p.handle(PaginationMessage::JumpTo(50));
    let _element = p.view(&theme, |_| ());
}

#[test]
fn test_pagination_view_with_page_sizes_renders() {
    let theme = Theme::element_light();
    let p = Pagination::new(100, 10).with_page_sizes(vec![10, 20, 50, 100]);
    let _element = p.view(&theme, |_| ());
}

#[test]
fn test_pagination_view_no_total_renders() {
    let theme = Theme::element_light();
    let p = Pagination::new(0, 10);
    let _element = p.view(&theme, |_| ());
}

#[test]
fn test_pagination_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let p = Pagination::new(100, 10);
    let _element = p.view(&theme, |_| ());
}

#[test]
fn test_pagination_view_with_custom_message_type() {
    let theme = Theme::element_light();
    let p = Pagination::new(100, 10);
    #[derive(Clone, Debug)]
    #[allow(dead_code)]
    enum AppMsg {
        PageChange(i64),
    }
    let _element = p.view(&theme, AppMsg::PageChange);
}
