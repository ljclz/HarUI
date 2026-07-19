//! Upload 上传 — 参考 Element Plus `<el-upload>`。
//!
//! 支持：拖拽上传、文件列表、accept、limit、multiple、状态机（Ready/Uploading/Success/Error）、删除。

/// 上传状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UploadStatus {
    #[default]
    Ready,
    Uploading,
    Success,
    Error,
}

/// 上传文件
#[derive(Debug, Clone)]
pub struct UploadFile {
    name: String,
    size: u64,
    status: UploadStatus,
    progress: u32,
    error: Option<String>,
}

impl UploadFile {
    pub fn new(name: impl Into<String>, size: u64) -> Self {
        Self {
            name: name.into(),
            size,
            status: UploadStatus::Ready,
            progress: 0,
            error: None,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn status(&self) -> UploadStatus {
        self.status
    }

    pub fn progress(&self) -> u32 {
        self.progress
    }

    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }
}

/// Upload 消息
#[derive(Debug, Clone)]
pub enum UploadMessage {
    AddFile(UploadFile),
    Drop(Vec<UploadFile>),
    Remove(usize),
    Clear,
    StartUpload(usize),
    Progress(usize, u32),
    UploadSuccess(usize),
    UploadError(usize, String),
}

/// Upload 组件
#[derive(Debug, Clone)]
pub struct Upload {
    file_list: Vec<UploadFile>,
    multiple: bool,
    limit: usize, // 0 表示不限
    accept: Vec<String>,
}

impl Default for Upload {
    fn default() -> Self {
        Self::new()
    }
}

impl Upload {
    pub fn new() -> Self {
        Self {
            file_list: Vec::new(),
            multiple: false,
            limit: 0,
            accept: Vec::new(),
        }
    }

    pub fn with_multiple(mut self, v: bool) -> Self {
        self.multiple = v;
        self
    }

    pub fn with_limit(mut self, l: usize) -> Self {
        self.limit = l;
        self
    }

    pub fn with_accept(mut self, a: Vec<String>) -> Self {
        self.accept = a;
        self
    }

    pub fn file_list(&self) -> &[UploadFile] {
        &self.file_list
    }

    pub fn multiple(&self) -> bool {
        self.multiple
    }

    pub fn limit(&self) -> usize {
        self.limit
    }

    pub fn accept(&self) -> &[String] {
        &self.accept
    }

    fn can_add_more(&self) -> bool {
        if self.limit > 0 && self.file_list.len() >= self.limit {
            return false;
        }
        if !self.multiple && !self.file_list.is_empty() {
            return false;
        }
        true
    }

    fn file_ext(name: &str) -> String {
        match name.rfind('.') {
            Some(pos) => name[pos + 1..].to_lowercase(),
            None => String::new(),
        }
    }

    fn accept_matches(&self, name: &str) -> bool {
        if self.accept.is_empty() {
            return true;
        }
        let ext = Self::file_ext(name);
        self.accept.iter().any(|a| a.to_lowercase() == ext)
    }

    pub fn handle(&mut self, msg: UploadMessage) {
        match msg {
            UploadMessage::AddFile(mut f) => {
                if !self.accept_matches(&f.name) {
                    return;
                }
                if !self.can_add_more() {
                    return;
                }
                f.status = UploadStatus::Ready;
                self.file_list.push(f);
            }
            UploadMessage::Drop(files) => {
                for f in files {
                    if !self.accept_matches(&f.name) {
                        continue;
                    }
                    if !self.can_add_more() {
                        break;
                    }
                    let mut f = f;
                    f.status = UploadStatus::Ready;
                    self.file_list.push(f);
                }
            }
            UploadMessage::Remove(idx) => {
                if idx < self.file_list.len() {
                    self.file_list.remove(idx);
                }
            }
            UploadMessage::Clear => {
                self.file_list.clear();
            }
            UploadMessage::StartUpload(idx) => {
                if let Some(f) = self.file_list.get_mut(idx) {
                    f.status = UploadStatus::Uploading;
                    f.progress = 0;
                    f.error = None;
                }
            }
            UploadMessage::Progress(idx, pct) => {
                if let Some(f) = self.file_list.get_mut(idx) {
                    f.progress = pct.min(100);
                }
            }
            UploadMessage::UploadSuccess(idx) => {
                if let Some(f) = self.file_list.get_mut(idx) {
                    f.status = UploadStatus::Success;
                    f.progress = 100;
                }
            }
            UploadMessage::UploadError(idx, err) => {
                if let Some(f) = self.file_list.get_mut(idx) {
                    f.status = UploadStatus::Error;
                    f.error = Some(err);
                }
            }
        }
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_upload_default_no_accept_no_limit() {
        let u = Upload::new();
        assert!(u.accept().is_empty());
        assert_eq!(u.limit(), 0);
        assert!(!u.multiple());
    }

    #[test]
    fn test_upload_file_ext_helper() {
        assert_eq!(Upload::file_ext("a.jpg"), "jpg");
        assert_eq!(Upload::file_ext("a.b.PNG"), "png");
        assert_eq!(Upload::file_ext("noext"), "");
    }
}
