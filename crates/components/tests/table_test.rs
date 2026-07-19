//! Table 组件测试
//!
//! 参考 Element Plus `<el-table>` 组件 API。
//! 测试覆盖：
//! - 基础渲染（columns + rows）
//! - stripe（斑马纹）/ border
//! - 排序（asc / desc / none）
//! - 行点击事件
//! - 空数据展示
//! - 固定列（left / right）

use har_ui_components::table::{
    TableColumn, TableProps, Table, TableMessage, SortOrder, FixedSide, Identifiable,
};

#[derive(Debug, Clone, PartialEq)]
struct TestRow {
    id: i32,
    name: String,
    price: f64,
}

impl Identifiable for TestRow {
    fn id(&self) -> String {
        self.id.to_string()
    }
}

/// 字段提取器：把 row 的某个字段转成字符串
fn extract_field(row: &TestRow, prop: &str) -> String {
    match prop {
        "id" => row.id.to_string(),
        "name" => row.name.clone(),
        "price" => row.price.to_string(),
        _ => String::new(),
    }
}

fn make_test_rows() -> Vec<TestRow> {
    vec![
        TestRow { id: 1, name: "Apple".to_string(), price: 5.5 },
        TestRow { id: 2, name: "Banana".to_string(), price: 3.2 },
        TestRow { id: 3, name: "Cherry".to_string(), price: 12.8 },
    ]
}

#[test]
fn test_table_columns_definition() {
    let cols = vec![
        TableColumn::new("id", "ID"),
        TableColumn::new("name", "Name"),
        TableColumn::new("price", "Price"),
    ];
    assert_eq!(cols.len(), 3);
    assert_eq!(cols[0].prop, "id");
    assert_eq!(cols[0].label, "ID");
}

#[test]
fn test_table_default_props() {
    let props = TableProps::default();
    assert!(!props.stripe);
    assert!(!props.border);
    assert_eq!(props.empty_text, "No Data");
}

#[test]
fn test_table_props_with_stripe() {
    let props = TableProps::new().with_stripe(true);
    assert!(props.stripe);
}

#[test]
fn test_table_props_with_border() {
    let props = TableProps::new().with_border(true);
    assert!(props.border);
}

#[test]
fn test_table_props_with_empty_text() {
    let props = TableProps::new().with_empty_text("暂无数据");
    assert_eq!(props.empty_text, "暂无数据");
}

#[test]
fn test_table_sort_order_variants() {
    let orders = [SortOrder::None, SortOrder::Ascending, SortOrder::Descending];
    assert_eq!(orders.len(), 3);
    assert_ne!(SortOrder::None, SortOrder::Ascending);
    assert_ne!(SortOrder::Ascending, SortOrder::Descending);
}

#[test]
fn test_table_default_sort_is_none() {
    let table: Table<TestRow> = Table::new();
    assert_eq!(table.sort_order(), SortOrder::None);
    assert_eq!(table.sort_prop(), None);
}

#[test]
fn test_table_sort_ascending_by_id() {
    let rows = vec![
        TestRow { id: 3, name: "C".to_string(), price: 0.0 },
        TestRow { id: 1, name: "A".to_string(), price: 0.0 },
        TestRow { id: 2, name: "B".to_string(), price: 0.0 },
    ];
    let cols = vec![TableColumn::new("id", "ID")];
    let mut table = Table::new().with_columns(cols).with_rows(rows).with_field_extractor(extract_field);
    table.handle(TableMessage::SortBy("id".to_string(), SortOrder::Ascending));
    let sorted: Vec<i32> = table.rows().iter().map(|r| r.id).collect();
    assert_eq!(sorted, vec![1, 2, 3]);
}

#[test]
fn test_table_sort_descending_by_price() {
    let rows = make_test_rows();
    let cols = vec![TableColumn::new("price", "Price")];
    let mut table = Table::new().with_columns(cols).with_rows(rows).with_field_extractor(extract_field);
    table.handle(TableMessage::SortBy("price".to_string(), SortOrder::Descending));
    let sorted: Vec<f64> = table.rows().iter().map(|r| r.price).collect();
    assert_eq!(sorted, vec![12.8, 5.5, 3.2]);
}

#[test]
fn test_table_sort_none_restores_original_order() {
    let original = make_test_rows();
    let rows = make_test_rows();
    let cols = vec![TableColumn::new("id", "ID")];
    let mut table = Table::new().with_columns(cols).with_rows(rows).with_field_extractor(extract_field);
    table.handle(TableMessage::SortBy("id".to_string(), SortOrder::Descending));
    table.handle(TableMessage::SortBy("id".to_string(), SortOrder::None));
    let result: Vec<i32> = table.rows().iter().map(|r| r.id).collect();
    let original_ids: Vec<i32> = original.iter().map(|r| r.id).collect();
    assert_eq!(result, original_ids);
}

#[test]
fn test_table_row_click_event() {
    let rows = make_test_rows();
    let cols = vec![TableColumn::new("id", "ID")];
    let mut table = Table::new().with_columns(cols).with_rows(rows);
    // 点击 row index = 1
    table.handle(TableMessage::RowClicked(1));
    assert_eq!(table.selected_row_index(), Some(1));
}

#[test]
fn test_table_empty_data() {
    let cols = vec![TableColumn::new("id", "ID")];
    let table: Table<TestRow> = Table::new().with_columns(cols).with_rows(vec![]);
    assert!(table.rows().is_empty());
    assert_eq!(table.props().empty_text, "No Data");
}

#[test]
fn test_table_empty_data_custom_text() {
    let cols = vec![TableColumn::new("id", "ID")];
    let props = TableProps::new().with_empty_text("暂无商品");
    let table: Table<TestRow> = Table::new().with_columns(cols).with_props(props).with_rows(vec![]);
    assert_eq!(table.props().empty_text, "暂无商品");
}

#[test]
fn test_table_fixed_column_left() {
    let col = TableColumn::new("id", "ID").with_fixed(FixedSide::Left);
    assert_eq!(col.fixed, Some(FixedSide::Left));
}

#[test]
fn test_table_fixed_column_right() {
    let col = TableColumn::new("action", "Action").with_fixed(FixedSide::Right);
    assert_eq!(col.fixed, Some(FixedSide::Right));
}

#[test]
fn test_table_sortable_column() {
    let col = TableColumn::new("price", "Price").sortable(true);
    assert!(col.sortable);
    let col2 = TableColumn::new("name", "Name");
    assert!(!col2.sortable);
}

#[test]
fn test_table_width_column() {
    let col = TableColumn::new("price", "Price").with_width(120.0);
    assert_eq!(col.width, Some(120.0));
}

#[test]
fn test_table_rows_count() {
    let rows = make_test_rows();
    let cols = vec![TableColumn::new("id", "ID")];
    let table = Table::new().with_columns(cols).with_rows(rows);
    assert_eq!(table.rows().len(), 3);
}

#[test]
fn test_table_row_id_extraction_via_trait() {
    use har_ui_components::table::Identifiable;
    let rows = make_test_rows();
    let cols = vec![TableColumn::new("id", "ID")];
    let table = Table::new().with_columns(cols).with_rows(rows.clone());
    // 使用 trait 方法获取 row id
    let first = table.rows().first().unwrap();
    assert_eq!(first.id(), "1");
}
