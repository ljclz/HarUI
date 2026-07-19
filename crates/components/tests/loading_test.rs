//! Loading 组件 — 加载指示器
//!
//! 参考 Element Plus `ElLoading`。
//! 支持：旋转动画、自定义文案、全屏加载（lock）、自定义图标。

use har_ui_components::loading::{Loading, LoadingMessage};

// ---------- 基础构造 ----------

#[test]
fn test_loading_default() {
    let l = Loading::new();
    assert!(!l.active());
    assert!(!l.fullscreen());
    assert!(!l.lock());
    assert_eq!(l.text(), None);
}

#[test]
fn test_loading_with_text() {
    let l = Loading::new().with_text("加载中...");
    assert_eq!(l.text(), Some(&"加载中...".to_string()));
}

#[test]
fn test_loading_with_fullscreen() {
    let l = Loading::new().with_fullscreen(true);
    assert!(l.fullscreen());
}

#[test]
fn test_loading_with_lock() {
    let l = Loading::new().with_lock(true);
    assert!(l.lock());
}

// ---------- 激活/停止 ----------

#[test]
fn test_loading_start() {
    let mut l = Loading::new();
    l.handle(LoadingMessage::Start);
    assert!(l.active());
}

#[test]
fn test_loading_stop() {
    let mut l = Loading::new();
    l.handle(LoadingMessage::Start);
    l.handle(LoadingMessage::Stop);
    assert!(!l.active());
}

#[test]
fn test_loading_toggle() {
    let mut l = Loading::new();
    l.handle(LoadingMessage::Toggle);
    assert!(l.active());
    l.handle(LoadingMessage::Toggle);
    assert!(!l.active());
}

// ---------- 旋转动画进度 ----------

#[test]
fn test_loading_rotation_progress() {
    let mut l = Loading::new();
    l.handle(LoadingMessage::Start);
    let initial = l.rotation();
    l.tick(16); // 16ms 一帧
    assert!(l.rotation() > initial);
}

#[test]
fn test_loading_rotation_loops_at_360() {
    let mut l = Loading::new();
    l.handle(LoadingMessage::Start);
    // 推进大量时间，确保 rotation 在 [0, 360) 范围内
    for _ in 0..1000 {
        l.tick(16);
    }
    let r = l.rotation();
    assert!(r >= 0.0 && r < 360.0);
}

#[test]
fn test_loading_no_rotation_when_inactive() {
    let mut l = Loading::new();
    // 未启动时不旋转
    l.tick(100);
    assert_eq!(l.rotation(), 0.0);
}

// ---------- 启动时设置文本 ----------

#[test]
fn test_loading_start_with_text() {
    let mut l = Loading::new();
    l.handle(LoadingMessage::StartWithText("Loading...".to_string()));
    assert!(l.active());
    assert_eq!(l.text(), Some(&"Loading...".to_string()));
}

// ---------- 停止时清理 ----------

#[test]
fn test_loading_stop_clears_rotation() {
    let mut l = Loading::new();
    l.handle(LoadingMessage::Start);
    l.tick(100);
    assert!(l.rotation() > 0.0);
    l.handle(LoadingMessage::Stop);
    assert_eq!(l.rotation(), 0.0);
}

// ---------- 全屏 + lock 联动 ----------

#[test]
fn test_loading_fullscreen_implies_lock_default() {
    // fullscreen=true 时 lock 自动开启
    let l = Loading::new().with_fullscreen(true);
    assert!(l.lock());
}

#[test]
fn test_loading_lock_without_fullscreen() {
    // 非 fullscreen 也可单独启用 lock
    let l = Loading::new().with_lock(true);
    assert!(l.lock());
    assert!(!l.fullscreen());
}
