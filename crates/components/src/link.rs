//! Link 文字链接 — 参考 Element Plus `<el-link>`。
//!
//! 支持：6 种 type、underline、disabled、href、icon、点击事件。

/// 类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LinkType {
    #[default]
    Default,
    Primary,
    Success,
    Warning,
    Danger,
    Info,
}

/// Link 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkMessage {
    Click,
    ResetClick,
}

/// Link 组件
#[derive(Debug, Clone)]
pub struct Link {
    link_type: LinkType,
    text: String,
    href: Option<String>,
    underline: bool,
    disabled: bool,
    icon: Option<String>,
    clicked: bool,
}

impl Default for Link {
    fn default() -> Self {
        Self::new()
    }
}

impl Link {
    pub fn new() -> Self {
        Self {
            link_type: LinkType::Default,
            text: String::new(),
            href: None,
            underline: true,
            disabled: false,
            icon: None,
            clicked: false,
        }
    }

    pub fn with_type(mut self, t: LinkType) -> Self {
        self.link_type = t;
        self
    }

    pub fn with_text(mut self, t: impl Into<String>) -> Self {
        self.text = t.into();
        self
    }

    pub fn with_href(mut self, h: impl Into<String>) -> Self {
        self.href = Some(h.into());
        self
    }

    pub fn with_underline(mut self, v: bool) -> Self {
        self.underline = v;
        self
    }

    pub fn with_disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }

    pub fn with_icon(mut self, i: impl Into<String>) -> Self {
        self.icon = Some(i.into());
        self
    }

    pub fn link_type(&self) -> LinkType {
        self.link_type
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn href(&self) -> Option<&str> {
        self.href.as_deref()
    }

    pub fn underline(&self) -> bool {
        self.underline
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn icon(&self) -> Option<&str> {
        self.icon.as_deref()
    }

    pub fn clicked(&self) -> bool {
        self.clicked
    }

    pub fn handle(&mut self, msg: LinkMessage) {
        match msg {
            LinkMessage::Click => {
                if !self.disabled {
                    self.clicked = true;
                }
            }
            LinkMessage::ResetClick => {
                self.clicked = false;
            }
        }
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_link_default_underline_true() {
        let l = Link::new();
        assert!(l.underline());
        assert_eq!(l.link_type(), LinkType::Default);
    }
}
