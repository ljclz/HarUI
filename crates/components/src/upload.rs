//! Upload 上传 — 参考 Element Plus `<el-upload>`。
//!
//! 支持：拖拽上传、文件列表、accept、limit、multiple、状态机（Ready/Uploading/Success/Error）、删除。

use har_ui_core::theme::Theme;
use iced::widget::{button, container, text};
use iced::{Color, Element, Length, Padding};

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

    /// 渲染 Upload 为 iced::Element
    ///
    /// 渲染触发按钮 + 文件列表（每项显示名称、大小、状态、进度）。
    ///
    /// # 参数
    /// - `theme`: HarUI 主题引用
    /// - `on_trigger`: 点击触发上传按钮的回调
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        on_trigger: impl Fn() -> Message + 'a,
    ) -> Element<'a, Message> {
        let text_primary = Color::from(theme.neutral.text_primary);
        let text_regular = Color::from(theme.neutral.text_regular);
        let text_placeholder = Color::from(theme.neutral.text_placeholder);
        let border_lighter = Color::from(theme.neutral.border_lighter);
        let primary = Color::from(theme.primary.base);
        let bg_overlay = Color::from(theme.neutral.bg_overlay);
        let success_color = Color::from(theme.success.base);
        let error_color = Color::from(theme.danger.base);

        // 触发按钮
        let trigger_text = text("点击上传").color(iced::Color::WHITE).size(14.0);
        let trigger_btn = button(trigger_text)
            .padding(Padding::from([8u16, 16u16]))
            .on_press(on_trigger())
            .style(move |_t, _status| iced::widget::button::Style {
                background: Some(iced::Background::Color(primary)),
                text_color: iced::Color::WHITE,
                border: iced::Border {
                    color: primary,
                    width: 1.0,
                    radius: iced::border::radius(4.0),
                },
                shadow: iced::Shadow::default(),
                snap: false,
            });

        let mut col_children: Vec<Element<'a, Message>> = Vec::new();
        col_children.push(trigger_btn.into());

        // accept 提示
        if !self.accept.is_empty() {
            let accept_text = format!("支持格式：{}", self.accept.join(", "));
            let hint = text(accept_text).color(text_placeholder).size(12.0);
            col_children.push(hint.into());
        }

        // 文件列表
        for file in &self.file_list {
            let status_str = match file.status() {
                UploadStatus::Ready => "待上传".to_string(),
                UploadStatus::Uploading => format!("上传中 {}%", file.progress()),
                UploadStatus::Success => "成功".to_string(),
                UploadStatus::Error => {
                    format!("失败：{}", file.error().unwrap_or("未知错误"))
                }
            };
            let status_color = match file.status() {
                UploadStatus::Ready => text_placeholder,
                UploadStatus::Uploading => primary,
                UploadStatus::Success => success_color,
                UploadStatus::Error => error_color,
            };

            let name_text = text(file.name().to_string()).color(text_primary).size(14.0);
            let size_text = format!("({} bytes)", file.size());
            let size_label = text(size_text).color(text_placeholder).size(12.0);
            let status_label = text(status_str).color(status_color).size(12.0);

            let file_row = iced::widget::Row::new()
                .push(name_text)
                .push(iced::widget::Space::new().width(Length::Fixed(6.0)))
                .push(size_label)
                .push(iced::widget::Space::new().width(Length::Fixed(6.0)))
                .push(status_label)
                .align_y(iced::Alignment::Center);

            let file_wrap = container(file_row)
                .width(Length::Fill)
                .padding(Padding::from([6u16, 8u16]))
                .style(move |_t| iced::widget::container::Style {
                    text_color: Some(text_regular),
                    background: Some(iced::Background::Color(bg_overlay)),
                    border: iced::Border {
                        color: border_lighter,
                        width: 1.0,
                        radius: iced::border::radius(2.0),
                    },
                    shadow: iced::Shadow::default(),
                    snap: false,
                });
            col_children.push(file_wrap.into());
        }

        container(iced::widget::Column::with_children(col_children).spacing(4))
            .width(Length::Fill)
            .into()
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
