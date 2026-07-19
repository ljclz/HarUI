//! Descriptions 描述列表 — 参考 Element Plus `<el-descriptions>`。
//!
//! 覆盖：标题/列数/边框/方向/项 span/对齐。

use har_ui_components::descriptions::{
    Descriptions, DescriptionsDirection, DescriptionsItem,
};

#[test]
fn test_descriptions_empty() {
    let d = Descriptions::new();
    assert!(d.items().is_empty());
    assert_eq!(d.title(), None);
    assert_eq!(d.column(), 3); // 默认 3 列
    assert!(d.border()); // 默认带边框
}

#[test]
fn test_descriptions_with_title() {
    let d = Descriptions::new().with_title("用户信息");
    assert_eq!(d.title(), Some("用户信息"));
}

#[test]
fn test_descriptions_with_items() {
    let d = Descriptions::new()
        .with_item(DescriptionsItem::new("姓名", "张三"))
        .with_item(DescriptionsItem::new("年龄", "30"));
    assert_eq!(d.items().len(), 2);
    assert_eq!(d.items()[0].label(), "姓名");
    assert_eq!(d.items()[0].value(), "张三");
}

#[test]
fn test_descriptions_column() {
    let d = Descriptions::new().with_column(4);
    assert_eq!(d.column(), 4);
}

#[test]
fn test_descriptions_direction() {
    let d = Descriptions::new().with_direction(DescriptionsDirection::Vertical);
    assert_eq!(d.direction(), DescriptionsDirection::Vertical);

    let d2 = Descriptions::new();
    assert_eq!(d2.direction(), DescriptionsDirection::Horizontal);
}

#[test]
fn test_descriptions_item_span() {
    let item = DescriptionsItem::new("备注", "很长").with_span(2);
    assert_eq!(item.span(), 2);
    let item_default = DescriptionsItem::new("a", "b");
    assert_eq!(item_default.span(), 1); // 默认 span=1
}

#[test]
fn test_descriptions_layout_calculates_rows() {
    // column=3，4 个项（其中一项 span=2）：总 span = 1+2+1+1 = 5 → 2 行
    let d = Descriptions::new().with_column(3)
        .with_item(DescriptionsItem::new("a", "1"))
        .with_item(DescriptionsItem::new("b", "2").with_span(2))
        .with_item(DescriptionsItem::new("c", "3"))
        .with_item(DescriptionsItem::new("d", "4"));
    let rows = d.layout_rows();
    assert_eq!(rows.len(), 2);
    // 第一行 3 列：a(1) + b(2) = 3
    assert_eq!(rows[0].len(), 2);
    // 第二行 2 列：c(1) + d(1) = 2
    assert_eq!(rows[1].len(), 2);
}

#[test]
fn test_descriptions_border_toggle() {
    let d = Descriptions::new().with_border(false);
    assert!(!d.border());
}

#[test]
fn test_descriptions_extra_slot() {
    let d = Descriptions::new().with_has_extra(true);
    assert!(d.has_extra());
}
