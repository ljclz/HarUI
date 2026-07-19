//! Notification 通知组件 — 参考 Element Plus `ElNotification`。

use har_ui_components::notification::{
    Notification, NotificationList, NotificationMessage, NotificationPosition, NotificationType,
};

// ---------- 基础构造 ----------

#[test]
fn test_notification_default() {
    let n = Notification::new("标题", "消息内容");
    assert_eq!(n.title(), "标题");
    assert_eq!(n.message(), "消息内容");
    assert_eq!(n.msg_type(), NotificationType::Info);
    assert_eq!(n.duration(), 4500);
    assert_eq!(n.position(), NotificationPosition::TopRight);
    assert!(n.show_close());
    assert_eq!(n.offset(), 0);
    assert!(!n.closed());
}

#[test]
fn test_notification_with_title_message() {
    let n = Notification::new("警告标题", "警告内容");
    assert_eq!(n.title(), "警告标题");
    assert_eq!(n.message(), "警告内容");
}

#[test]
fn test_notification_with_empty() {
    let n = Notification::new("", "");
    assert_eq!(n.title(), "");
    assert_eq!(n.message(), "");
}

// ---------- 4 种 type ----------

#[test]
fn test_notification_with_type() {
    use NotificationType::*;
    let cases = [Success, Warning, Info, Error];
    for t in cases.iter() {
        let n = Notification::new("x", "y").with_type(*t);
        assert_eq!(n.msg_type(), *t);
    }
}

#[test]
fn test_notification_type_as_str() {
    use NotificationType::*;
    assert_eq!(Success.as_str(), "success");
    assert_eq!(Warning.as_str(), "warning");
    assert_eq!(Info.as_str(), "info");
    assert_eq!(Error.as_str(), "error");
}

// ---------- 4 种 position ----------

#[test]
fn test_notification_all_positions() {
    use NotificationPosition::*;
    let cases = [TopRight, TopLeft, BottomRight, BottomLeft];
    for p in cases.iter() {
        let n = Notification::new("x", "y").with_position(*p);
        assert_eq!(n.position(), *p);
    }
}

#[test]
fn test_notification_position_as_str() {
    use NotificationPosition::*;
    assert_eq!(TopRight.as_str(), "top-right");
    assert_eq!(TopLeft.as_str(), "top-left");
    assert_eq!(BottomRight.as_str(), "bottom-right");
    assert_eq!(BottomLeft.as_str(), "bottom-left");
}

// ---------- duration ----------

#[test]
fn test_notification_default_duration_4500() {
    let n = Notification::new("x", "y");
    // Element Plus 默认 4500ms
    assert_eq!(n.duration(), 4500);
}

#[test]
fn test_notification_with_duration() {
    let n = Notification::new("x", "y").with_duration(3000);
    assert_eq!(n.duration(), 3000);
}

#[test]
fn test_notification_duration_zero_never_auto_close() {
    let n = Notification::new("x", "y").with_duration(0);
    assert_eq!(n.duration(), 0);
}

// ---------- offset / show_close ----------

#[test]
fn test_notification_with_offset() {
    let n = Notification::new("x", "y").with_offset(50);
    assert_eq!(n.offset(), 50);
}

#[test]
fn test_notification_with_show_close_false() {
    let n = Notification::new("x", "y").with_show_close(false);
    assert!(!n.show_close());
}

// ---------- 自动关闭（tick 模拟时间推进） ----------

#[test]
fn test_notification_auto_close_after_duration() {
    let mut n = Notification::new("x", "y").with_duration(3000);
    assert!(!n.closed());
    n.tick(2999);
    assert!(!n.closed(), "2999ms 还不应关闭");
    n.tick(1);
    assert!(n.closed(), "3000ms 应自动关闭");
}

#[test]
fn test_notification_duration_zero_no_auto_close() {
    let mut n = Notification::new("x", "y").with_duration(0);
    n.tick(1000000);
    assert!(!n.closed(), "duration=0 永不自动关闭");
}

#[test]
fn test_notification_default_duration_auto_close() {
    let mut n = Notification::new("x", "y");
    n.tick(4500);
    assert!(n.closed());
}

// ---------- 主动 close ----------

#[test]
fn test_notification_manual_close() {
    let mut n = Notification::new("x", "y").with_duration(10000);
    n.handle(NotificationMessage::Close);
    assert!(n.closed());
}

#[test]
fn test_notification_close_after_auto_close_no_op() {
    let mut n = Notification::new("x", "y").with_duration(100);
    n.tick(100);
    assert!(n.closed());
    // 已关闭再 close 应保持关闭（不应 panic）
    n.handle(NotificationMessage::Close);
    assert!(n.closed());
}

// ---------- NotificationList 多个堆叠 ----------

#[test]
fn test_notification_list_default_empty() {
    let list = NotificationList::new();
    assert!(list.is_empty());
    assert_eq!(list.len(), 0);
}

#[test]
fn test_notification_list_push() {
    let mut list = NotificationList::new();
    list.push(Notification::new("t1", "m1"));
    assert_eq!(list.len(), 1);
    list.push(Notification::new("t2", "m2"));
    assert_eq!(list.len(), 2);
}

#[test]
fn test_notification_list_iterate() {
    let mut list = NotificationList::new();
    list.push(Notification::new("t1", "m1"));
    list.push(Notification::new("t2", "m2"));
    let titles: Vec<_> = list.iter().map(|n| n.title()).collect();
    assert_eq!(titles, vec!["t1", "t2"]);
}

#[test]
fn test_notification_list_tick_auto_removes_closed() {
    let mut list = NotificationList::new();
    list.push(Notification::new("t1", "m1").with_duration(1000));
    list.push(Notification::new("t2", "m2").with_duration(2000));
    list.tick(1000);
    // 第一个应已关闭并被移除
    assert_eq!(list.len(), 1);
    assert_eq!(list.iter().next().unwrap().title(), "t2");
}

#[test]
fn test_notification_list_manual_close_removes() {
    let mut list = NotificationList::new();
    list.push(Notification::new("t1", "m1"));
    list.push(Notification::new("t2", "m2"));
    // 关闭 id=0 的通知
    list.close(0);
    assert_eq!(list.len(), 1);
    assert_eq!(list.iter().next().unwrap().title(), "t2");
}

#[test]
fn test_notification_list_close_invalid_id_safe() {
    let mut list = NotificationList::new();
    list.push(Notification::new("t1", "m1"));
    list.close(999); // 无效 id 不应 panic
    assert_eq!(list.len(), 1);
}

// ---------- 堆叠 offset 计算 ----------

#[test]
fn test_notification_list_stacked_offset() {
    let mut list = NotificationList::new();
    list.push(Notification::new("t1", "m1"));
    list.push(Notification::new("t2", "m2"));
    list.push(Notification::new("t3", "m3"));
    // 第二个 offset 应大于第一个（堆叠）
    let offsets = list.stacked_offsets(60);
    assert_eq!(offsets.len(), 3);
    assert!(offsets[1] > offsets[0]);
    assert!(offsets[2] > offsets[1]);
}

#[test]
fn test_notification_list_stacked_offset_with_custom_offset() {
    let mut list = NotificationList::new();
    list.push(Notification::new("t1", "m1").with_offset(20));
    list.push(Notification::new("t2", "m2").with_offset(10));
    let offsets = list.stacked_offsets(60);
    // 第一个 offset = 20（自定义）, 第二个 offset = 20 + 60 + 10 = 90
    assert_eq!(offsets[0], 20);
    assert_eq!(offsets[1], 90);
}

// ---------- 边界 ----------

#[test]
fn test_notification_list_clear_all() {
    let mut list = NotificationList::new();
    list.push(Notification::new("t1", "m1"));
    list.push(Notification::new("t2", "m2"));
    list.clear();
    assert!(list.is_empty());
}

#[test]
fn test_notification_list_tick_empty_safe() {
    let mut list = NotificationList::new();
    list.tick(1000); // 空列表 tick 不应 panic
    assert!(list.is_empty());
}
