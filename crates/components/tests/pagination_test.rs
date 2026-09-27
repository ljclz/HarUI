//! Pagination 分页组件 — 参考 Element Plus `<el-pagination>`。

use har_ui_components::pagination::{Pagination, PaginationMessage};

// ---------- 基础构造 ----------

#[test]
fn test_pagination_default() {
    let p = Pagination::new(100, 10);
    assert_eq!(p.total(), 100);
    assert_eq!(p.page_size(), 10);
    assert_eq!(p.current_page(), 1);
}

#[test]
fn test_pagination_with_page_sizes() {
    let p = Pagination::new(100, 10).with_page_sizes(vec![10, 20, 50, 100]);
    assert_eq!(p.page_sizes(), &[10, 20, 50, 100]);
}

// ---------- 总页数 ----------

#[test]
fn test_pagination_total_pages_exact() {
    // 100 条 / 10 每页 = 10 页
    let p = Pagination::new(100, 10);
    assert_eq!(p.total_pages(), 10);
}

#[test]
fn test_pagination_total_pages_with_remainder() {
    // 105 条 / 10 每页 = 11 页
    let p = Pagination::new(105, 10);
    assert_eq!(p.total_pages(), 11);
}

#[test]
fn test_pagination_total_pages_zero_total() {
    let p = Pagination::new(0, 10);
    assert_eq!(p.total_pages(), 1); // 至少 1 页
}

// ---------- 翻页 ----------

#[test]
fn test_pagination_next_page() {
    let mut p = Pagination::new(100, 10);
    p.handle(PaginationMessage::Next);
    assert_eq!(p.current_page(), 2);
}

#[test]
fn test_pagination_prev_page() {
    let mut p = Pagination::new(100, 10);
    p.handle(PaginationMessage::Next);
    p.handle(PaginationMessage::Next);
    p.handle(PaginationMessage::Prev);
    assert_eq!(p.current_page(), 2);
}

#[test]
fn test_pagination_prev_at_first_page_no_op() {
    let mut p = Pagination::new(100, 10);
    p.handle(PaginationMessage::Prev);
    assert_eq!(p.current_page(), 1);
}

#[test]
fn test_pagination_next_at_last_page_no_op() {
    let mut p = Pagination::new(100, 10);
    for _ in 0..20 {
        p.handle(PaginationMessage::Next);
    }
    assert_eq!(p.current_page(), 10);
}

// ---------- 跳转 ----------

#[test]
fn test_pagination_jump_to_page() {
    let mut p = Pagination::new(100, 10);
    p.handle(PaginationMessage::JumpTo(5));
    assert_eq!(p.current_page(), 5);
}

#[test]
fn test_pagination_jump_beyond_max_clamped() {
    let mut p = Pagination::new(100, 10);
    p.handle(PaginationMessage::JumpTo(999));
    assert_eq!(p.current_page(), 10);
}

#[test]
fn test_pagination_jump_below_min_clamped() {
    let mut p = Pagination::new(100, 10);
    p.handle(PaginationMessage::JumpTo(5));
    p.handle(PaginationMessage::JumpTo(0));
    assert_eq!(p.current_page(), 1);
}

#[test]
fn test_pagination_jump_to_negative_clamped() {
    let mut p = Pagination::new(100, 10);
    p.handle(PaginationMessage::JumpTo(-5));
    assert_eq!(p.current_page(), 1);
}

// ---------- 修改 page_size ----------

#[test]
fn test_pagination_change_page_size() {
    let mut p = Pagination::new(100, 10);
    p.handle(PaginationMessage::ChangePageSize(20));
    assert_eq!(p.page_size(), 20);
    assert_eq!(p.total_pages(), 5);
}

#[test]
fn test_pagination_change_page_size_resets_current() {
    let mut p = Pagination::new(100, 10);
    p.handle(PaginationMessage::JumpTo(8));
    p.handle(PaginationMessage::ChangePageSize(20));
    // 改 page_size 后应重置到第 1 页
    assert_eq!(p.current_page(), 1);
}

// ---------- 页码按钮折叠 ----------

#[test]
fn test_pagination_page_buttons_simple() {
    // 总 5 页，当前第 1 页：1 2 3 4 5
    let p = Pagination::new(50, 10);
    let btns = p.page_buttons();
    assert_eq!(btns, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_pagination_page_buttons_with_ellipsis() {
    // 总 20 页，当前第 10 页：1 ... 8 9 10 11 12 ... 20
    let mut p = Pagination::new(200, 10);
    p.handle(PaginationMessage::JumpTo(10));
    let btns = p.page_buttons();
    assert!(btns.contains(&1));
    assert!(btns.contains(&20));
    assert!(btns.contains(&10));
    // 包含 0 表示省略号
    assert!(btns.contains(&0));
}

#[test]
fn test_pagination_page_buttons_near_start() {
    // 当前第 1 页，总 20 页：1 2 3 4 5 ... 20
    let p = Pagination::new(200, 10);
    let btns = p.page_buttons();
    assert!(btns.contains(&1));
    assert!(btns.contains(&20));
}

// ---------- 边界 ----------

#[test]
fn test_pagination_zero_total_safe() {
    let mut p = Pagination::new(0, 10);
    p.handle(PaginationMessage::Next);
    assert_eq!(p.current_page(), 1); // 不能翻页
}

#[test]
fn test_pagination_zero_page_size_safe() {
    // page_size=0 应避免除零 panic
    let p = Pagination::new(100, 0);
    // total_pages 至少 1，不 panic
    assert!(p.total_pages() >= 1);
}
