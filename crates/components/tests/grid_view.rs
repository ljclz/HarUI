//! Grid view() 测试 — TDD RED 阶段

use har_ui_components::grid::{Grid, GridItem, GridProps};
use har_ui_core::theme::Theme;
use iced::Element;
use iced::widget::text;

struct DummyItem {
    id: String,
}

impl GridItem for DummyItem {
    fn item_id(&self) -> String {
        self.id.clone()
    }
}

fn make_children<'a>() -> Vec<Element<'a, ()>> {
    vec![
        text("A").into(),
        text("B").into(),
        text("C").into(),
        text("D").into(),
    ]
}

#[test]
fn test_grid_view_default_renders() {
    let theme = Theme::element_light();
    let g: Grid<DummyItem> = Grid::new();
    let _element = g.view(&theme, make_children());
}

#[test]
fn test_grid_view_with_items_renders() {
    let theme = Theme::element_light();
    let items = vec![
        DummyItem {
            id: "1".to_string(),
        },
        DummyItem {
            id: "2".to_string(),
        },
    ];
    let g: Grid<DummyItem> = Grid::new().with_items(items);
    let _element = g.view(&theme, make_children());
}

#[test]
fn test_grid_view_with_columns_2_renders() {
    let theme = Theme::element_light();
    let props = GridProps::new().with_columns(2);
    let g: Grid<DummyItem> = Grid::new().with_props(props);
    let _element = g.view(&theme, make_children());
}

#[test]
fn test_grid_view_with_columns_6_renders() {
    let theme = Theme::element_light();
    let props = GridProps::new().with_columns(6).with_gap(12.0);
    let g: Grid<DummyItem> = Grid::new().with_props(props);
    let _element = g.view(&theme, make_children());
}

#[test]
fn test_grid_view_with_card_width_renders() {
    let theme = Theme::element_light();
    let props = GridProps::new()
        .with_columns(3)
        .with_gap(8.0)
        .with_card_width(120.0);
    let g: Grid<DummyItem> = Grid::new().with_props(props);
    let _element = g.view(&theme, make_children());
}

#[test]
fn test_grid_view_empty_children_renders() {
    let theme = Theme::element_light();
    let g: Grid<DummyItem> = Grid::new();
    let empty: Vec<Element<()>> = Vec::new();
    let _element = g.view(&theme, empty);
}

#[test]
fn test_grid_view_with_selection_renders() {
    let theme = Theme::element_light();
    let mut g: Grid<DummyItem> = Grid::new().with_items(vec![
        DummyItem {
            id: "1".to_string(),
        },
        DummyItem {
            id: "2".to_string(),
        },
    ]);
    use har_ui_components::grid::GridMessage;
    g.handle(GridMessage::Select("1".to_string()));
    let _element = g.view(&theme, make_children());
}

#[test]
fn test_grid_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let g: Grid<DummyItem> = Grid::new();
    let _element = g.view(&theme, make_children());
}

#[test]
fn test_grid_view_custom_message_type() {
    let theme = Theme::element_light();
    let g: Grid<DummyItem> = Grid::new();
    #[derive(Clone, Debug)]
    #[allow(dead_code)]
    enum AppMsg {
        Nop,
    }
    let children: Vec<Element<AppMsg>> = vec![text("A").into(), text("B").into()];
    let _element = g.view(&theme, children);
}
