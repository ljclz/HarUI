//! HangOrder 挂单面板组件 — POS 挂单/取单管理
//!
//! 支持：挂起、取单、删除、清空、按时间倒序排列。

/// 挂单项
#[derive(Debug, Clone, PartialEq)]
pub struct HangOrderItem {
    pub id: String,
    pub customer_name: String,
    pub total: f64,
    pub item_count: usize,
    pub timestamp: i64,
}

/// 挂单消息
///
/// - `Hang(item)`: 挂起新订单（重复 ID 会被拒绝）
/// - `Take(id)`: 取单（恢复到当前购物车），返回 Option<HangOrderItem>
/// - `Delete(id)`: 删除挂单，返回 bool
/// - `ClearAll`: 清空所有挂单
#[derive(Debug, Clone, PartialEq)]
pub enum HangOrderMessage {
    Hang(HangOrderItem),
    Take(String),
    Delete(String),
    ClearAll,
}

/// HangOrder 挂单面板
#[derive(Debug, Clone, Default)]
pub struct HangOrder {
    items: Vec<HangOrderItem>,
}

impl HangOrder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn items(&self) -> &[HangOrderItem] {
        &self.items
    }

    pub fn count(&self) -> usize {
        self.items.len()
    }

    /// 处理消息，返回操作结果
    /// - Hang: 返回 None（无返回值）
    /// - Take: 返回 Some(item) 或 None
    /// - Delete: 返回 Some(()) 表示成功，None 表示失败
    /// - ClearAll: 返回 None
    pub fn handle(&mut self, msg: HangOrderMessage) -> Option<HangOrderItem> {
        match msg {
            HangOrderMessage::Hang(item) => {
                // 重复 ID 拒绝
                if self.items.iter().any(|i| i.id == item.id) {
                    return None;
                }
                // 按时间倒序插入（最新在前）
                let pos = self
                    .items
                    .iter()
                    .position(|i| i.timestamp < item.timestamp)
                    .unwrap_or(self.items.len());
                self.items.insert(pos, item);
                None
            }
            HangOrderMessage::Take(id) => {
                if let Some(pos) = self.items.iter().position(|i| i.id == id) {
                    Some(self.items.remove(pos))
                } else {
                    None
                }
            }
            HangOrderMessage::Delete(id) => {
                if let Some(pos) = self.items.iter().position(|i| i.id == id) {
                    self.items.remove(pos);
                    Some(HangOrderItem {
                        id: String::new(),
                        customer_name: String::new(),
                        total: 0.0,
                        item_count: 0,
                        timestamp: 0,
                    })
                } else {
                    None
                }
            }
            HangOrderMessage::ClearAll => {
                self.items.clear();
                None
            }
        }
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_hang_order_default_empty() {
        let h = HangOrder::new();
        assert_eq!(h.count(), 0);
        assert!(h.items().is_empty());
    }

    #[test]
    fn test_hang_order_take_returns_item() {
        let mut h = HangOrder::new();
        let item = HangOrderItem {
            id: "H001".to_string(),
            customer_name: "张三".to_string(),
            total: 88.50,
            item_count: 3,
            timestamp: 1700000000,
        };
        h.handle(HangOrderMessage::Hang(item.clone()));
        let taken = h.handle(HangOrderMessage::Take("H001".to_string()));
        assert_eq!(taken, Some(item));
    }

    #[test]
    fn test_hang_order_delete_returns_some_on_success() {
        let mut h = HangOrder::new();
        let item = HangOrderItem {
            id: "H001".to_string(),
            customer_name: "张三".to_string(),
            total: 88.50,
            item_count: 3,
            timestamp: 1700000000,
        };
        h.handle(HangOrderMessage::Hang(item));
        let result = h.handle(HangOrderMessage::Delete("H001".to_string()));
        assert!(result.is_some());
    }
}
