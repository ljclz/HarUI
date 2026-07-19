//! HangOrder view() 测试 — TDD RED 阶段

use har_ui_components::hang_order::{HangOrder, HangOrderItem, HangOrderMessage};
use har_ui_core::theme::Theme;

fn make_item(id: &str, ts: i64) -> HangOrderItem {
    HangOrderItem {
        id: id.to_string(),
        customer_name: format!("客户-{}", id),
        total: 88.50,
        item_count: 3,
        timestamp: ts,
    }
}

#[test]
fn test_hang_order_view_empty_renders() {
    let theme = Theme::element_light();
    let h = HangOrder::new();
    let _element = h.view(&theme, |_| (), |_| ());
}

#[test]
fn test_hang_order_view_single_item_renders() {
    let theme = Theme::element_light();
    let mut h = HangOrder::new();
    h.handle(HangOrderMessage::Hang(make_item("H001", 1700000000)));
    let _element = h.view(&theme, |_| (), |_| ());
}

#[test]
fn test_hang_order_view_multiple_items_renders() {
    let theme = Theme::element_light();
    let mut h = HangOrder::new();
    for i in 0..5 {
        h.handle(HangOrderMessage::Hang(make_item(
            &format!("H{:03}", i),
            1700000000 + i as i64,
        )));
    }
    let _element = h.view(&theme, |_| (), |_| ());
}

#[test]
fn test_hang_order_view_after_take_renders() {
    let theme = Theme::element_light();
    let mut h = HangOrder::new();
    h.handle(HangOrderMessage::Hang(make_item("H001", 1000)));
    h.handle(HangOrderMessage::Hang(make_item("H002", 2000)));
    h.handle(HangOrderMessage::Take("H001".to_string()));
    let _element = h.view(&theme, |_| (), |_| ());
}

#[test]
fn test_hang_order_view_after_delete_renders() {
    let theme = Theme::element_light();
    let mut h = HangOrder::new();
    h.handle(HangOrderMessage::Hang(make_item("H001", 1000)));
    h.handle(HangOrderMessage::Hang(make_item("H002", 2000)));
    h.handle(HangOrderMessage::Delete("H002".to_string()));
    let _element = h.view(&theme, |_| (), |_| ());
}

#[test]
fn test_hang_order_view_after_clear_all_renders() {
    let theme = Theme::element_light();
    let mut h = HangOrder::new();
    h.handle(HangOrderMessage::Hang(make_item("H001", 1000)));
    h.handle(HangOrderMessage::Hang(make_item("H002", 2000)));
    h.handle(HangOrderMessage::ClearAll);
    let _element = h.view(&theme, |_| (), |_| ());
}

#[test]
fn test_hang_order_view_with_zero_total_renders() {
    let theme = Theme::element_light();
    let mut h = HangOrder::new();
    let item = HangOrderItem {
        id: "H001".to_string(),
        customer_name: "空".to_string(),
        total: 0.0,
        item_count: 0,
        timestamp: 1700000000,
    };
    h.handle(HangOrderMessage::Hang(item));
    let _element = h.view(&theme, |_| (), |_| ());
}

#[test]
fn test_hang_order_view_with_large_total_renders() {
    let theme = Theme::element_light();
    let mut h = HangOrder::new();
    let item = HangOrderItem {
        id: "BIG".to_string(),
        customer_name: "VIP".to_string(),
        total: 9999999.99,
        item_count: 100,
        timestamp: 1700000000,
    };
    h.handle(HangOrderMessage::Hang(item));
    let _element = h.view(&theme, |_| (), |_| ());
}

#[test]
fn test_hang_order_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let mut h = HangOrder::new();
    h.handle(HangOrderMessage::Hang(make_item("H001", 1700000000)));
    h.handle(HangOrderMessage::Hang(make_item("H002", 1700000001)));
    let _element = h.view(&theme, |_| (), |_| ());
}

#[test]
fn test_hang_order_view_custom_message_type() {
    let theme = Theme::element_light();
    let mut h = HangOrder::new();
    h.handle(HangOrderMessage::Hang(make_item("H001", 1700000000)));
    #[derive(Clone, Debug)]
    #[allow(dead_code)]
    enum AppMsg {
        Resume(String),
        Delete(String),
    }
    let _element = h.view(&theme, |s| AppMsg::Resume(s), |s| AppMsg::Delete(s));
}
