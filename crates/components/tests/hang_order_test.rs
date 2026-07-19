//! HangOrder 挂单面板组件 — POS 挂单/取单管理
//!
//! 支持：
//! - 挂起当前购物车（生成挂单 ID）
//! - 取单（恢复指定挂单到当前购物车）
//! - 删除挂单
//! - 列表展示所有挂单

use har_ui_components::hang_order::{HangOrder, HangOrderItem, HangOrderMessage};

// ---------- 基础构造 ----------

#[test]
fn test_hang_order_default_empty() {
    let h = HangOrder::new();
    assert!(h.items().is_empty());
    assert_eq!(h.count(), 0);
}

// ---------- 挂起订单 ----------

#[test]
fn test_hang_order_hang_new_item() {
    let mut h = HangOrder::new();
    let item = HangOrderItem {
        id: "H001".to_string(),
        customer_name: "张三".to_string(),
        total: 88.50,
        item_count: 3,
        timestamp: 1700000000,
    };
    h.handle(HangOrderMessage::Hang(item.clone()));
    assert_eq!(h.count(), 1);
    assert_eq!(h.items()[0].id, "H001");
    assert_eq!(h.items()[0].customer_name, "张三");
}

#[test]
fn test_hang_order_hang_multiple_items() {
    let mut h = HangOrder::new();
    for i in 0..5 {
        let item = HangOrderItem {
            id: format!("H{:03}", i),
            customer_name: format!("客户{}", i),
            total: 10.0 * (i as f64 + 1.0),
            item_count: i + 1,
            timestamp: 1700000000 + i as i64,
        };
        h.handle(HangOrderMessage::Hang(item));
    }
    assert_eq!(h.count(), 5);
}

#[test]
fn test_hang_order_duplicate_id_rejected() {
    let mut h = HangOrder::new();
    let item1 = HangOrderItem {
        id: "H001".to_string(),
        customer_name: "张三".to_string(),
        total: 50.0,
        item_count: 2,
        timestamp: 1700000000,
    };
    let item2 = HangOrderItem {
        id: "H001".to_string(), // 重复 ID
        customer_name: "李四".to_string(),
        total: 80.0,
        item_count: 4,
        timestamp: 1700000001,
    };
    h.handle(HangOrderMessage::Hang(item1));
    h.handle(HangOrderMessage::Hang(item2));
    // 第二个重复 ID 应被拒绝
    assert_eq!(h.count(), 1);
    assert_eq!(h.items()[0].customer_name, "张三");
}

// ---------- 取单 ----------

#[test]
fn test_hang_order_take_existing() {
    let mut h = HangOrder::new();
    let item = HangOrderItem {
        id: "H001".to_string(),
        customer_name: "张三".to_string(),
        total: 88.50,
        item_count: 3,
        timestamp: 1700000000,
    };
    h.handle(HangOrderMessage::Hang(item));

    let taken = h.handle(HangOrderMessage::Take("H001".to_string()));
    assert!(taken.is_some());
    assert_eq!(taken.unwrap().id, "H001");
    // 取单后从列表删除
    assert_eq!(h.count(), 0);
}

#[test]
fn test_hang_order_take_nonexistent() {
    let mut h = HangOrder::new();
    let taken = h.handle(HangOrderMessage::Take("H999".to_string()));
    assert!(taken.is_none());
}

// ---------- 删除挂单 ----------

#[test]
fn test_hang_order_delete_existing() {
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
    assert_eq!(h.count(), 0);
}

#[test]
fn test_hang_order_delete_nonexistent() {
    let mut h = HangOrder::new();
    let result = h.handle(HangOrderMessage::Delete("H999".to_string()));
    assert!(result.is_none());
}

// ---------- 清空 ----------

#[test]
fn test_hang_order_clear_all() {
    let mut h = HangOrder::new();
    for i in 0..3 {
        let item = HangOrderItem {
            id: format!("H{:03}", i),
            customer_name: format!("客户{}", i),
            total: 10.0,
            item_count: 1,
            timestamp: 1700000000,
        };
        h.handle(HangOrderMessage::Hang(item));
    }
    h.handle(HangOrderMessage::ClearAll);
    assert_eq!(h.count(), 0);
}

// ---------- 排序：按时间倒序（最新在前） ----------

#[test]
fn test_hang_order_sort_by_time_desc() {
    let mut h = HangOrder::new();
    let items = vec![
        HangOrderItem {
            id: "H001".to_string(),
            customer_name: "早".to_string(),
            total: 10.0,
            item_count: 1,
            timestamp: 1000,
        },
        HangOrderItem {
            id: "H002".to_string(),
            customer_name: "晚".to_string(),
            total: 20.0,
            item_count: 2,
            timestamp: 2000,
        },
        HangOrderItem {
            id: "H003".to_string(),
            customer_name: "中".to_string(),
            total: 30.0,
            item_count: 3,
            timestamp: 1500,
        },
    ];
    for item in items {
        h.handle(HangOrderMessage::Hang(item));
    }
    // 按时间倒序：晚(2000) -> 中(1500) -> 早(1000)
    assert_eq!(h.items()[0].customer_name, "晚");
    assert_eq!(h.items()[1].customer_name, "中");
    assert_eq!(h.items()[2].customer_name, "早");
}

// ---------- 空挂单列表的 take/delete ----------

#[test]
fn test_hang_order_empty_list_operations_no_panic() {
    let mut h = HangOrder::new();
    // 空列表上操作不应 panic
    let taken = h.handle(HangOrderMessage::Take("H001".to_string()));
    assert!(taken.is_none());
    let deleted = h.handle(HangOrderMessage::Delete("H001".to_string()));
    assert!(deleted.is_none());
    h.handle(HangOrderMessage::ClearAll);
    assert_eq!(h.count(), 0);
}
