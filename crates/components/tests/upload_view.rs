//! R.2.P0.6 Upload view() 测试 — TDD RED 阶段
//!
//! 验证 Upload view() 正确渲染触发按钮 + 文件列表。
//! 测试中所有消息统一使用 String 类型。

use har_ui_components::upload::{Upload, UploadFile, UploadMessage, UploadStatus};
use har_ui_core::theme::Theme;

// ============== R.2.P0.6.a view() 基础渲染 ==============

#[test]
fn test_upload_view_default_renders() {
    let theme = Theme::element_light();
    let u = Upload::new();
    let _element = u.view(&theme, || "trigger".to_string());
}

#[test]
fn test_upload_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let u = Upload::new();
    let _element = u.view(&theme, || "trigger".to_string());
}

#[test]
fn test_upload_view_with_accept_renders() {
    let theme = Theme::element_light();
    let u = Upload::new().with_accept(vec!["jpg".to_string(), "png".to_string()]);
    let _element = u.view(&theme, || "trigger".to_string());
}

#[test]
fn test_upload_view_with_multiple_renders() {
    let theme = Theme::element_light();
    let u = Upload::new().with_multiple(true);
    let _element = u.view(&theme, || "trigger".to_string());
}

#[test]
fn test_upload_view_with_limit_renders() {
    let theme = Theme::element_light();
    let u = Upload::new().with_multiple(true).with_limit(3);
    let _element = u.view(&theme, || "trigger".to_string());
}

// ============== R.2.P0.6.b 文件列表渲染 ==============

#[test]
fn test_upload_view_with_single_file_renders() {
    let theme = Theme::element_light();
    let mut u = Upload::new();
    u.handle(UploadMessage::AddFile(UploadFile::new("a.jpg", 1024)));
    assert_eq!(u.file_list().len(), 1);
    let _element = u.view(&theme, || "trigger".to_string());
}

#[test]
fn test_upload_view_with_multiple_files_renders() {
    let theme = Theme::element_light();
    let mut u = Upload::new().with_multiple(true);
    u.handle(UploadMessage::AddFile(UploadFile::new("a.jpg", 1024)));
    u.handle(UploadMessage::AddFile(UploadFile::new("b.png", 2048)));
    u.handle(UploadMessage::AddFile(UploadFile::new("c.txt", 512)));
    assert_eq!(u.file_list().len(), 3);
    let _element = u.view(&theme, || "trigger".to_string());
}

#[test]
fn test_upload_view_uploading_status_renders() {
    let theme = Theme::element_light();
    let mut u = Upload::new();
    u.handle(UploadMessage::AddFile(UploadFile::new("a.jpg", 1024)));
    u.handle(UploadMessage::StartUpload(0));
    assert_eq!(u.file_list()[0].status(), UploadStatus::Uploading);
    let _element = u.view(&theme, || "trigger".to_string());
}

#[test]
fn test_upload_view_with_progress_renders() {
    let theme = Theme::element_light();
    let mut u = Upload::new();
    u.handle(UploadMessage::AddFile(UploadFile::new("a.jpg", 1024)));
    u.handle(UploadMessage::StartUpload(0));
    u.handle(UploadMessage::Progress(0, 50));
    assert_eq!(u.file_list()[0].progress(), 50);
    let _element = u.view(&theme, || "trigger".to_string());
}

#[test]
fn test_upload_view_success_status_renders() {
    let theme = Theme::element_light();
    let mut u = Upload::new();
    u.handle(UploadMessage::AddFile(UploadFile::new("a.jpg", 1024)));
    u.handle(UploadMessage::UploadSuccess(0));
    assert_eq!(u.file_list()[0].status(), UploadStatus::Success);
    let _element = u.view(&theme, || "trigger".to_string());
}

#[test]
fn test_upload_view_error_status_renders() {
    let theme = Theme::element_light();
    let mut u = Upload::new();
    u.handle(UploadMessage::AddFile(UploadFile::new("a.jpg", 1024)));
    u.handle(UploadMessage::UploadError(0, "网络错误".to_string()));
    assert_eq!(u.file_list()[0].status(), UploadStatus::Error);
    let _element = u.view(&theme, || "trigger".to_string());
}

// ============== R.2.P0.6.c 边界情况 ==============

#[test]
fn test_upload_view_drop_files_renders() {
    let theme = Theme::element_light();
    let mut u = Upload::new().with_multiple(true);
    let files = vec![
        UploadFile::new("a.jpg", 1024),
        UploadFile::new("b.png", 2048),
    ];
    u.handle(UploadMessage::Drop(files));
    assert_eq!(u.file_list().len(), 2);
    let _element = u.view(&theme, || "trigger".to_string());
}

#[test]
fn test_upload_view_accept_filter_renders() {
    let theme = Theme::element_light();
    let mut u = Upload::new().with_accept(vec!["jpg".to_string()]);
    // 不匹配 accept 应被忽略
    u.handle(UploadMessage::AddFile(UploadFile::new("a.txt", 1024)));
    assert_eq!(u.file_list().len(), 0);
    // 匹配的应添加
    u.handle(UploadMessage::AddFile(UploadFile::new("b.jpg", 1024)));
    assert_eq!(u.file_list().len(), 1);
    let _element = u.view(&theme, || "trigger".to_string());
}

#[test]
fn test_upload_view_limit_blocks_renders() {
    let theme = Theme::element_light();
    let mut u = Upload::new().with_multiple(true).with_limit(2);
    u.handle(UploadMessage::AddFile(UploadFile::new("a.jpg", 1024)));
    u.handle(UploadMessage::AddFile(UploadFile::new("b.jpg", 1024)));
    u.handle(UploadMessage::AddFile(UploadFile::new("c.jpg", 1024)));
    assert_eq!(u.file_list().len(), 2);
    let _element = u.view(&theme, || "trigger".to_string());
}

#[test]
fn test_upload_view_remove_file_renders() {
    let theme = Theme::element_light();
    let mut u = Upload::new().with_multiple(true);
    u.handle(UploadMessage::AddFile(UploadFile::new("a.jpg", 1024)));
    u.handle(UploadMessage::AddFile(UploadFile::new("b.jpg", 2048)));
    u.handle(UploadMessage::Remove(0));
    assert_eq!(u.file_list().len(), 1);
    assert_eq!(u.file_list()[0].name(), "b.jpg");
    let _element = u.view(&theme, || "trigger".to_string());
}

#[test]
fn test_upload_view_clear_renders() {
    let theme = Theme::element_light();
    let mut u = Upload::new().with_multiple(true);
    u.handle(UploadMessage::AddFile(UploadFile::new("a.jpg", 1024)));
    u.handle(UploadMessage::AddFile(UploadFile::new("b.jpg", 2048)));
    u.handle(UploadMessage::Clear);
    assert_eq!(u.file_list().len(), 0);
    let _element = u.view(&theme, || "trigger".to_string());
}

#[test]
fn test_upload_view_progress_clamp_renders() {
    let theme = Theme::element_light();
    let mut u = Upload::new();
    u.handle(UploadMessage::AddFile(UploadFile::new("a.jpg", 1024)));
    u.handle(UploadMessage::StartUpload(0));
    u.handle(UploadMessage::Progress(0, 150));
    assert_eq!(u.file_list()[0].progress(), 100);
    let _element = u.view(&theme, || "trigger".to_string());
}

// ============== R.2.P0.6.d 自定义消息类型 ==============

#[test]
fn test_upload_view_with_custom_message_type() {
    let theme = Theme::element_light();
    let mut u = Upload::new().with_multiple(true);
    u.handle(UploadMessage::AddFile(UploadFile::new("a.jpg", 1024)));
    #[derive(Clone, Debug)]
    #[allow(dead_code)]
    enum AppMsg {
        Trigger,
        Other(String),
    }
    let _element = u.view(&theme, || AppMsg::Trigger);
}
