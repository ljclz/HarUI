//! Upload 上传 — 参考 Element Plus `<el-upload>`。
//!
//! 覆盖：拖拽上传、文件列表、accept、limit、multiple、状态机、删除。

use har_ui_components::upload::{Upload, UploadFile, UploadMessage, UploadStatus};

#[test]
fn test_upload_default() {
    let u = Upload::new();
    assert!(u.file_list().is_empty());
    assert!(!u.multiple());
    assert_eq!(u.limit(), 0); // 0 表示不限
}

#[test]
fn test_upload_add_file() {
    let mut u = Upload::new();
    u.handle(UploadMessage::AddFile(UploadFile::new("a.jpg", 1024)));
    assert_eq!(u.file_list().len(), 1);
    assert_eq!(u.file_list()[0].name(), "a.jpg");
    assert_eq!(u.file_list()[0].size(), 1024);
    assert_eq!(u.file_list()[0].status(), UploadStatus::Ready);
}

#[test]
fn test_upload_multiple_blocks_second_without_flag() {
    let mut u = Upload::new(); // multiple=false
    u.handle(UploadMessage::AddFile(UploadFile::new("a.jpg", 100)));
    u.handle(UploadMessage::AddFile(UploadFile::new("b.jpg", 200)));
    // 单选模式只保留一个
    assert_eq!(u.file_list().len(), 1);
}

#[test]
fn test_upload_multiple_allows_many() {
    let mut u = Upload::new().with_multiple(true);
    u.handle(UploadMessage::AddFile(UploadFile::new("a.jpg", 100)));
    u.handle(UploadMessage::AddFile(UploadFile::new("b.jpg", 200)));
    assert_eq!(u.file_list().len(), 2);
}

#[test]
fn test_upload_limit_blocks_beyond() {
    let mut u = Upload::new().with_multiple(true).with_limit(2);
    u.handle(UploadMessage::AddFile(UploadFile::new("a.jpg", 100)));
    u.handle(UploadMessage::AddFile(UploadFile::new("b.jpg", 200)));
    u.handle(UploadMessage::AddFile(UploadFile::new("c.jpg", 300)));
    assert_eq!(u.file_list().len(), 2); // limit=2
}

#[test]
fn test_upload_status_transitions() {
    let mut u = Upload::new();
    u.handle(UploadMessage::AddFile(UploadFile::new("a.jpg", 100)));
    // Ready → Uploading
    u.handle(UploadMessage::StartUpload(0));
    assert_eq!(u.file_list()[0].status(), UploadStatus::Uploading);
    // Uploading → Success
    u.handle(UploadMessage::UploadSuccess(0));
    assert_eq!(u.file_list()[0].status(), UploadStatus::Success);
}

#[test]
fn test_upload_progress_updates() {
    let mut u = Upload::new();
    u.handle(UploadMessage::AddFile(UploadFile::new("a.jpg", 100)));
    u.handle(UploadMessage::StartUpload(0));
    u.handle(UploadMessage::Progress(0, 50));
    assert_eq!(u.file_list()[0].progress(), 50);
    u.handle(UploadMessage::Progress(0, 100));
    assert_eq!(u.file_list()[0].progress(), 100);
}

#[test]
fn test_upload_remove_file() {
    let mut u = Upload::new().with_multiple(true);
    u.handle(UploadMessage::AddFile(UploadFile::new("a.jpg", 100)));
    u.handle(UploadMessage::AddFile(UploadFile::new("b.jpg", 200)));
    u.handle(UploadMessage::Remove(0));
    assert_eq!(u.file_list().len(), 1);
    assert_eq!(u.file_list()[0].name(), "b.jpg");
}

#[test]
fn test_upload_accept_filter() {
    let mut u = Upload::new().with_accept(vec!["jpg".to_string(), "png".to_string()]);
    // jpg 通过
    u.handle(UploadMessage::AddFile(UploadFile::new("a.jpg", 100)));
    assert_eq!(u.file_list().len(), 1);
    // txt 被拒绝
    u.handle(UploadMessage::AddFile(UploadFile::new("b.txt", 200)));
    assert_eq!(u.file_list().len(), 1);
}

#[test]
fn test_upload_drag_drop() {
    let mut u = Upload::new().with_multiple(true);
    u.handle(UploadMessage::Drop(vec![
        UploadFile::new("a.jpg", 100),
        UploadFile::new("b.jpg", 200),
    ]));
    assert_eq!(u.file_list().len(), 2);
}

#[test]
fn test_upload_clear_all() {
    let mut u = Upload::new().with_multiple(true);
    u.handle(UploadMessage::AddFile(UploadFile::new("a.jpg", 100)));
    u.handle(UploadMessage::AddFile(UploadFile::new("b.jpg", 200)));
    u.handle(UploadMessage::Clear);
    assert!(u.file_list().is_empty());
}

#[test]
fn test_upload_upload_failure() {
    let mut u = Upload::new();
    u.handle(UploadMessage::AddFile(UploadFile::new("a.jpg", 100)));
    u.handle(UploadMessage::StartUpload(0));
    u.handle(UploadMessage::UploadError(0, "网络错误".to_string()));
    assert_eq!(u.file_list()[0].status(), UploadStatus::Error);
    assert_eq!(u.file_list()[0].error(), Some("网络错误"));
}
