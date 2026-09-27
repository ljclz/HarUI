//! Grid 组件测试 — POS 商品网格
//!
//! 参考 Element Plus Grid 设计 + POS 商品网格场景。
//! 测试覆盖：
//! - columns / gap / card_width 配置
//! - 选中态
//! - 2000 卡片渲染（数据量测试）
//! - 虚拟滚动切片
//! - 空网格

use har_ui_components::grid::{Grid, GridItem, GridMessage, GridProps};

#[derive(Debug, Clone, PartialEq)]
struct TestItem {
    id: i32,
    name: String,
    price: f64,
}

impl GridItem for TestItem {
    fn item_id(&self) -> String {
        self.id.to_string()
    }
}

fn make_items(n: usize) -> Vec<TestItem> {
    (0..n)
        .map(|i| TestItem {
            id: i as i32,
            name: format!("Item {}", i),
            price: (i as f64) * 1.5,
        })
        .collect()
}

#[test]
fn test_grid_default_props() {
    let props = GridProps::default();
    assert_eq!(props.columns, 4);
    assert_eq!(props.gap, 8.0);
    assert_eq!(props.card_width, None);
}

#[test]
fn test_grid_with_columns() {
    let props = GridProps::new().with_columns(6);
    assert_eq!(props.columns, 6);
}

#[test]
fn test_grid_with_gap() {
    let props = GridProps::new().with_gap(12.0);
    assert_eq!(props.gap, 12.0);
}

#[test]
fn test_grid_with_card_width() {
    let props = GridProps::new().with_card_width(180.0);
    assert_eq!(props.card_width, Some(180.0));
}

#[test]
fn test_grid_default_no_selection() {
    let grid: Grid<TestItem> = Grid::new();
    assert_eq!(grid.selected_id(), None);
}

#[test]
fn test_grid_select_item() {
    let items = make_items(10);
    let mut grid = Grid::new().with_items(items).with_props(GridProps::new());
    grid.handle(GridMessage::Select("5".to_string()));
    assert_eq!(grid.selected_id(), Some(&"5".to_string()));
}

#[test]
fn test_grid_clear_selection() {
    let items = make_items(10);
    let mut grid = Grid::new().with_items(items);
    grid.handle(GridMessage::Select("3".to_string()));
    grid.handle(GridMessage::ClearSelection);
    assert_eq!(grid.selected_id(), None);
}

#[test]
fn test_grid_empty_items() {
    let grid: Grid<TestItem> = Grid::new().with_items(vec![]);
    assert_eq!(grid.items().len(), 0);
}

#[test]
fn test_grid_2000_items_count() {
    let items = make_items(2000);
    let grid = Grid::new().with_items(items);
    assert_eq!(grid.items().len(), 2000);
}

#[test]
fn test_grid_virtual_scroll_slice() {
    let items = make_items(2000);
    let grid = Grid::new().with_items(items).with_props(GridProps::new());
    // 模拟虚拟滚动：每页显示 50 个，从第 100 个开始
    let slice = grid.visible_slice(100, 150);
    assert_eq!(slice.len(), 50);
    assert_eq!(slice[0].id, 100);
    assert_eq!(slice[49].id, 149);
}

#[test]
fn test_grid_virtual_scroll_clamps_to_end() {
    let items = make_items(100);
    let grid = Grid::new().with_items(items);
    // 请求范围超过数据末尾
    let slice = grid.visible_slice(90, 200);
    assert_eq!(slice.len(), 10); // 只剩 10 个
    assert_eq!(slice[0].id, 90);
    assert_eq!(slice[9].id, 99);
}

#[test]
fn test_grid_select_nonexistent_id_does_not_change() {
    let items = make_items(5);
    let mut grid = Grid::new().with_items(items);
    grid.handle(GridMessage::Select("999".to_string()));
    // 不存在的 id 不应改变选中状态
    assert_eq!(grid.selected_id(), None);
}

#[test]
fn test_grid_builder_chains() {
    let items = make_items(20);
    let grid = Grid::new().with_items(items.clone()).with_props(
        GridProps::new()
            .with_columns(8)
            .with_gap(16.0)
            .with_card_width(150.0),
    );
    assert_eq!(grid.items().len(), 20);
    assert_eq!(grid.props().columns, 8);
    assert_eq!(grid.props().gap, 16.0);
    assert_eq!(grid.props().card_width, Some(150.0));
}

#[test]
fn test_grid_items_immutable_view() {
    let items = make_items(5);
    let grid = Grid::new().with_items(items);
    let view = grid.items();
    assert_eq!(view.len(), 5);
    assert_eq!(view[0].name, "Item 0");
}
