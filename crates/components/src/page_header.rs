//! PageHeader 页头 — 参考 Element Plus `<el-page-header>`。
//!
//! 支持：标题、副标题、内容、icon、返回事件、额外 slot。

/// PageHeader 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PageHeaderMessage {
    Back,
    ResetBack,
}

/// PageHeader 组件
#[derive(Debug, Clone)]
pub struct PageHeader {
    title: String,
    subtitle: Option<String>,
    content: Option<String>,
    icon: String,
    has_extra: bool,
    back_clicked: bool,
}

impl Default for PageHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl PageHeader {
    pub fn new() -> Self {
        Self {
            title: "返回".to_string(),
            subtitle: None,
            content: None,
            icon: "arrow-left".to_string(),
            has_extra: false,
            back_clicked: false,
        }
    }

    pub fn with_title(mut self, t: impl Into<String>) -> Self {
        self.title = t.into();
        self
    }

    pub fn with_subtitle(mut self, s: impl Into<String>) -> Self {
        self.subtitle = Some(s.into());
        self
    }

    pub fn with_content(mut self, c: impl Into<String>) -> Self {
        self.content = Some(c.into());
        self
    }

    pub fn with_icon(mut self, i: impl Into<String>) -> Self {
        self.icon = i.into();
        self
    }

    pub fn with_has_extra(mut self, v: bool) -> Self {
        self.has_extra = v;
        self
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn subtitle(&self) -> Option<&str> {
        self.subtitle.as_deref()
    }

    pub fn content(&self) -> Option<&str> {
        self.content.as_deref()
    }

    pub fn icon(&self) -> &str {
        &self.icon
    }

    pub fn has_extra(&self) -> bool {
        self.has_extra
    }

    pub fn back_clicked(&self) -> bool {
        self.back_clicked
    }

    pub fn handle(&mut self, msg: PageHeaderMessage) {
        match msg {
            PageHeaderMessage::Back => self.back_clicked = true,
            PageHeaderMessage::ResetBack => self.back_clicked = false,
        }
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_page_header_default_icon_arrow_left() {
        let p = PageHeader::new();
        assert_eq!(p.icon(), "arrow-left");
        assert_eq!(p.title(), "返回");
    }
}
