//! R.2.P0.4 Table view() 测试 — TDD RED 阶段
//!
//! 验证 Table view() 正确渲染表头、数据行、空数据、斑马纹、边框。
//! 使用 TableRow = BTreeMap<String, String> 作为行数据类型。

use std::collections::BTreeMap;

use har_ui_components::table::{
    SortOrder, Table, TableColumn, TableMessage, TableProps, TableRow, VirtualScroll,
};
use har_ui_core::theme::Theme;

fn make_row(id: &str, name: &str, age: &str) -> TableRow {
    let mut r = BTreeMap::new();
    r.insert("id".to_string(), id.to_string());
    r.insert("name".to_string(), name.to_string());
    r.insert("age".to_string(), age.to_string());
    r
}

fn field_extractor(row: &TableRow, prop: &str) -> String {
    row.get(prop).cloned().unwrap_or_default()
}

fn make_columns() -> Vec<TableColumn> {
    vec![
        TableColumn::new("id", "ID").with_width(60.0),
        TableColumn::new("name", "Name").with_width(120.0),
        TableColumn::new("age", "Age")
            .with_width(60.0)
            .sortable(true),
    ]
}

// ============== R.2.P0.4.a view() 基础渲染 ==============

#[test]
fn test_table_view_empty_renders() {
    let theme = Theme::element_light();
    let table = Table::new()
        .with_columns(make_columns())
        .with_rows(Vec::<TableRow>::new())
        .with_props(TableProps::new().with_empty_text("No Data"));
    let _element = table.view(&theme, field_extractor, |_| ());
}

#[test]
fn test_table_view_with_rows_renders() {
    let theme = Theme::element_light();
    let rows = vec![
        make_row("1", "Alice", "30"),
        make_row("2", "Bob", "25"),
        make_row("3", "Charlie", "35"),
    ];
    let table = Table::new().with_columns(make_columns()).with_rows(rows);
    let _element = table.view(&theme, field_extractor, |_| ());
}

#[test]
fn test_table_view_with_stripe_renders() {
    let theme = Theme::element_light();
    let rows = vec![make_row("1", "Alice", "30"), make_row("2", "Bob", "25")];
    let table = Table::new()
        .with_columns(make_columns())
        .with_rows(rows)
        .with_props(TableProps::new().with_stripe(true));
    let _element = table.view(&theme, field_extractor, |_| ());
}

#[test]
fn test_table_view_with_border_renders() {
    let theme = Theme::element_light();
    let rows = vec![make_row("1", "Alice", "30")];
    let table = Table::new()
        .with_columns(make_columns())
        .with_rows(rows)
        .with_props(TableProps::new().with_border(true));
    let _element = table.view(&theme, field_extractor, |_| ());
}

#[test]
fn test_table_view_with_selected_row_renders() {
    let theme = Theme::element_light();
    let rows = vec![make_row("1", "Alice", "30"), make_row("2", "Bob", "25")];
    let mut table = Table::new().with_columns(make_columns()).with_rows(rows);
    table.handle(TableMessage::RowClicked(1));
    assert_eq!(table.selected_row_index(), Some(1));
    let _element = table.view(&theme, field_extractor, |_| ());
}

// ============== R.2.P0.4.b 虚拟滚动 view() ==============

#[test]
fn test_table_view_virtual_scroll_renders() {
    let theme = Theme::element_light();
    // 200 行，阈值 100，启用虚拟滚动
    let rows: Vec<TableRow> = (0..200)
        .map(|i| {
            make_row(
                &i.to_string(),
                &format!("User{}", i),
                &((i % 50 + 20).to_string()),
            )
        })
        .collect();
    let table = Table::new()
        .with_columns(make_columns())
        .with_rows(rows)
        .with_virtual_scroll(VirtualScroll::new(30.0, 300.0).with_threshold(100));
    assert!(table.is_virtual_scroll_active());
    let _element = table.view(&theme, field_extractor, |_| ());
}

#[test]
fn test_table_view_virtual_scroll_with_offset_renders() {
    let theme = Theme::element_light();
    let rows: Vec<TableRow> = (0..500)
        .map(|i| make_row(&i.to_string(), &format!("User{}", i), "30"))
        .collect();
    let mut table = Table::new()
        .with_columns(make_columns())
        .with_rows(rows)
        .with_virtual_scroll(VirtualScroll::new(30.0, 300.0).with_threshold(100));
    table.handle(TableMessage::Scroll(900.0));
    let (start, end) = table.visible_range();
    assert!(start > 0, "scroll 后 visible_start 应 > 0");
    let _element = table.view(&theme, field_extractor, |_| ());
    let _ = end;
}

// ============== R.2.P0.4.c 排序状态 view() ==============

#[test]
fn test_table_view_with_sort_applied_renders() {
    let theme = Theme::element_light();
    let rows = vec![
        make_row("1", "Charlie", "30"),
        make_row("2", "Alice", "25"),
        make_row("3", "Bob", "35"),
    ];
    let mut table = Table::new()
        .with_columns(make_columns())
        .with_rows(rows)
        .with_field_extractor(field_extractor);
    table.handle(TableMessage::SortBy(
        "name".to_string(),
        SortOrder::Ascending,
    ));
    assert_eq!(table.sort_prop(), Some(&"name".to_string()));
    let _element = table.view(&theme, field_extractor, |_| ());
}

// ============== R.2.P0.4.d 主题/消息类型变体 ==============

#[test]
fn test_table_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let rows = vec![make_row("1", "Alice", "30")];
    let table = Table::new().with_columns(make_columns()).with_rows(rows);
    let _element = table.view(&theme, field_extractor, |_| ());
}

#[test]
fn test_table_view_with_custom_message_type() {
    let theme = Theme::element_light();
    let rows = vec![make_row("1", "Alice", "30")];
    let table = Table::new().with_columns(make_columns()).with_rows(rows);
    #[derive(Clone, Debug)]
    #[allow(dead_code)]
    enum AppMsg {
        RowClicked(usize),
    }
    let _element = table.view(&theme, field_extractor, AppMsg::RowClicked);
}

#[test]
fn test_table_view_single_row_renders() {
    let theme = Theme::element_light();
    let rows = vec![make_row("1", "Alice", "30")];
    let table = Table::new().with_columns(make_columns()).with_rows(rows);
    let _element = table.view(&theme, field_extractor, |_| ());
}

#[test]
fn test_table_view_many_columns_renders() {
    let theme = Theme::element_light();
    let cols = vec![
        TableColumn::new("a", "A"),
        TableColumn::new("b", "B"),
        TableColumn::new("c", "C"),
        TableColumn::new("d", "D"),
        TableColumn::new("e", "E"),
    ];
    let mut row = BTreeMap::new();
    row.insert("a".to_string(), "1".to_string());
    row.insert("b".to_string(), "2".to_string());
    row.insert("c".to_string(), "3".to_string());
    row.insert("d".to_string(), "4".to_string());
    row.insert("e".to_string(), "5".to_string());
    let table = Table::new().with_columns(cols).with_rows(vec![row]);
    let _element = table.view(&theme, field_extractor, |_| ());
}
