//! Toast/Message 组件 — 全局消息提示
//!
//! 参考 Element Plus `ElMessage`。
//! 支持：4 种 type(success/warning/info/error)、自动关闭(duration)、手动关闭、叠加展示、全局单例。
//! 单例模式下多条消息用 ID 区分，按时间顺序叠加。

use har_ui_components::message::{MessageItem, MessageManager, MessageType};

// ---------- MessageItem ----------

#[test]
fn test_message_item_default() {
    let item = MessageItem::new("Hello", MessageType::Info);
    assert_eq!(item.text(), "Hello");
    assert_eq!(item.msg_type(), MessageType::Info);
    assert_eq!(item.duration(), 3000);
    assert!(!item.closed());
}

#[test]
fn test_message_item_with_duration() {
    let item = MessageItem::new("Hello", MessageType::Info).with_duration(5000);
    assert_eq!(item.duration(), 5000);
}

#[test]
fn test_message_item_types() {
    let types = [
        MessageType::Success,
        MessageType::Warning,
        MessageType::Info,
        MessageType::Error,
    ];
    assert_eq!(types.len(), 4);
}

// ---------- MessageManager 单例 ----------

#[test]
fn test_message_manager_default_empty() {
    let m = MessageManager::new();
    assert!(m.items().is_empty());
}

#[test]
fn test_message_manager_show_message() {
    let mut m = MessageManager::new();
    let id = m.show(MessageItem::new("Hello", MessageType::Info));
    assert!(id > 0);
    assert_eq!(m.items().len(), 1);
    assert_eq!(m.items()[0].text(), "Hello");
}

#[test]
fn test_message_manager_show_multiple_messages() {
    let mut m = MessageManager::new();
    m.show(MessageItem::new("M1", MessageType::Info));
    m.show(MessageItem::new("M2", MessageType::Success));
    m.show(MessageItem::new("M3", MessageType::Warning));
    assert_eq!(m.items().len(), 3);
}

#[test]
fn test_message_manager_close_by_id() {
    let mut m = MessageManager::new();
    let id = m.show(MessageItem::new("Hello", MessageType::Info));
    let result = m.close(id);
    assert!(result);
    assert_eq!(m.items().len(), 0);
}

#[test]
fn test_message_manager_close_nonexistent() {
    let mut m = MessageManager::new();
    let result = m.close(999);
    assert!(!result);
}

#[test]
fn test_message_manager_close_all() {
    let mut m = MessageManager::new();
    m.show(MessageItem::new("M1", MessageType::Info));
    m.show(MessageItem::new("M2", MessageType::Info));
    m.close_all();
    assert_eq!(m.items().len(), 0);
}

// ---------- 自动关闭（tick 模拟时间流逝） ----------

#[test]
fn test_message_auto_close_after_duration() {
    let mut m = MessageManager::new();
    let item = MessageItem::new("Hello", MessageType::Info).with_duration(3000);
    let id = m.show(item);
    // 时间未到
    m.tick(2000);
    assert_eq!(m.items().len(), 1);
    // 时间到达
    m.tick(1000);
    assert_eq!(m.items().len(), 0);
    let _ = id;
}

#[test]
fn test_message_no_auto_close_when_duration_zero() {
    // duration=0 表示不自动关闭
    let mut m = MessageManager::new();
    let item = MessageItem::new("Hello", MessageType::Info).with_duration(0);
    m.show(item);
    m.tick(10000);
    assert_eq!(m.items().len(), 1);
}

#[test]
fn test_message_auto_close_independent_durations() {
    // 每条消息的 duration 独立计算
    let mut m = MessageManager::new();
    m.show(MessageItem::new("M1", MessageType::Info).with_duration(1000));
    m.show(MessageItem::new("M2", MessageType::Info).with_duration(3000));
    m.tick(1000);
    // M1 关闭，M2 还在
    assert_eq!(m.items().len(), 1);
    assert_eq!(m.items()[0].text(), "M2");
    m.tick(2000);
    assert_eq!(m.items().len(), 0);
}

// ---------- offset 叠加（垂直堆叠间距） ----------

#[test]
fn test_message_offset_calculation() {
    let mut m = MessageManager::new();
    m.show(MessageItem::new("M1", MessageType::Info));
    m.show(MessageItem::new("M2", MessageType::Info));
    m.show(MessageItem::new("M3", MessageType::Info));
    // 默认间距 16px，第一条 offset=20，第二条=20+item_height+16...
    let offsets = m.offsets();
    assert_eq!(offsets.len(), 3);
    // 后一条 offset 大于前一条
    assert!(offsets[1] > offsets[0]);
    assert!(offsets[2] > offsets[1]);
}

// ---------- 手动关闭 ----------

#[test]
fn test_message_manual_close() {
    let mut m = MessageManager::new();
    let id = m.show(MessageItem::new("Hello", MessageType::Info).with_duration(0));
    // duration=0 不自动关闭，需手动关闭
    m.tick(10000);
    assert_eq!(m.items().len(), 1);
    m.close(id);
    assert_eq!(m.items().len(), 0);
}
