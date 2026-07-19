//! Grid 组件 — 商品网格（POS 核心组件）
//!
//! 用于 POS 商品展示，支持多列网格布局、选中态、虚拟滚动切片。

/// Grid Item trait — 网格项的唯一标识
pub trait GridItem {
    fn item_id(&self) -> String;
}

/// Grid Props — 配置
#[derive(Debug, Clone)]
pub struct GridProps {
    /// 列数
    pub columns: usize,
    /// 间距（px）
    pub gap: f32,
    /// 卡片宽度（可选，None 表示自动）
    pub card_width: Option<f32>,
}

impl Default for GridProps {
    fn default() -> Self {
        Self {
            columns: 4,
            gap: 8.0,
            card_width: None,
        }
    }
}

impl GridProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_columns(mut self, c: usize) -> Self {
        self.columns = c;
        self
    }

    pub fn with_gap(mut self, g: f32) -> Self {
        self.gap = g;
        self
    }

    pub fn with_card_width(mut self, w: f32) -> Self {
        self.card_width = Some(w);
        self
    }
}

/// Grid 消息
#[derive(Debug, Clone, PartialEq)]
pub enum GridMessage {
    /// 选中某项
    Select(String),
    /// 清空选中
    ClearSelection,
}

/// Grid 组件
#[derive(Debug, Clone)]
pub struct Grid<T: GridItem> {
    items: Vec<T>,
    props: GridProps,
    selected_id: Option<String>,
}

impl<T: GridItem> Grid<T> {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            props: GridProps::default(),
            selected_id: None,
        }
    }

    pub fn with_items(mut self, items: Vec<T>) -> Self {
        self.items = items;
        self
    }

    pub fn with_props(mut self, props: GridProps) -> Self {
        self.props = props;
        self
    }

    pub fn items(&self) -> &[T] {
        &self.items
    }

    pub fn props(&self) -> &GridProps {
        &self.props
    }

    pub fn selected_id(&self) -> Option<&String> {
        self.selected_id.as_ref()
    }

    /// 处理消息
    pub fn handle(&mut self, msg: GridMessage) {
        match msg {
            GridMessage::Select(id) => {
                // 只允许选中存在的项
                if self.items.iter().any(|i| i.item_id() == id) {
                    self.selected_id = Some(id);
                }
            }
            GridMessage::ClearSelection => {
                self.selected_id = None;
            }
        }
    }

    /// 虚拟滚动：获取可见切片 [start, end)
    pub fn visible_slice(&self, start: usize, end: usize) -> &[T] {
        let start = start.min(self.items.len());
        let end = end.min(self.items.len());
        if start >= end {
            &self.items[start..start]
        } else {
            &self.items[start..end]
        }
    }
}

impl<T: GridItem> Default for Grid<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    struct Item {
        id: i32,
    }

    impl GridItem for Item {
        fn item_id(&self) -> String {
            self.id.to_string()
        }
    }

    #[test]
    fn test_grid_default_state() {
        let grid: Grid<Item> = Grid::new();
        assert_eq!(grid.items().len(), 0);
        assert_eq!(grid.selected_id(), None);
        assert_eq!(grid.props().columns, 4);
    }

    #[test]
    fn test_grid_props_builder() {
        let p = GridProps::new()
            .with_columns(6)
            .with_gap(16.0)
            .with_card_width(120.0);
        assert_eq!(p.columns, 6);
        assert_eq!(p.gap, 16.0);
        assert_eq!(p.card_width, Some(120.0));
    }

    #[test]
    fn test_grid_visible_slice_empty() {
        let grid: Grid<Item> = Grid::new();
        let s = grid.visible_slice(0, 10);
        assert!(s.is_empty());
    }
}
