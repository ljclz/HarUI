//! Text 文本 — 参考 Element Plus `<el-text>`。
//!
//! 支持：6 种 type、3 种 size、truncated、tag、copyable、max_lines。

/// 类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextType {
    #[default]
    Default,
    Primary,
    Success,
    Warning,
    Danger,
    Info,
}

/// 尺寸
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextSize {
    Large,
    #[default]
    Default,
    Small,
}

/// 标签
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextTag {
    #[default]
    Span,
    P,
    Div,
    H1,
    H2,
    H3,
}

/// Text 组件
#[derive(Debug, Clone)]
pub struct Text {
    text_type: TextType,
    size: TextSize,
    content: String,
    truncated: bool,
    tag: TextTag,
    copyable: bool,
    max_lines: Option<u32>,
}

impl Default for Text {
    fn default() -> Self {
        Self::new()
    }
}

impl Text {
    pub fn new() -> Self {
        Self {
            text_type: TextType::Default,
            size: TextSize::Default,
            content: String::new(),
            truncated: false,
            tag: TextTag::Span,
            copyable: false,
            max_lines: None,
        }
    }

    pub fn with_type(mut self, t: TextType) -> Self {
        self.text_type = t;
        self
    }

    pub fn with_size(mut self, s: TextSize) -> Self {
        self.size = s;
        self
    }

    pub fn with_content(mut self, c: impl Into<String>) -> Self {
        self.content = c.into();
        self
    }

    pub fn with_truncated(mut self, v: bool) -> Self {
        self.truncated = v;
        self
    }

    pub fn with_tag(mut self, t: TextTag) -> Self {
        self.tag = t;
        self
    }

    pub fn with_copyable(mut self, v: bool) -> Self {
        self.copyable = v;
        self
    }

    pub fn with_max_lines(mut self, n: u32) -> Self {
        self.max_lines = Some(n);
        self
    }

    pub fn text_type(&self) -> TextType {
        self.text_type
    }

    pub fn size(&self) -> TextSize {
        self.size
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn truncated(&self) -> bool {
        self.truncated
    }

    pub fn tag(&self) -> TextTag {
        self.tag
    }

    pub fn copyable(&self) -> bool {
        self.copyable
    }

    pub fn max_lines(&self) -> Option<u32> {
        self.max_lines
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_text_default_empty_content() {
        let t = Text::new();
        assert_eq!(t.content(), "");
        assert_eq!(t.text_type(), TextType::Default);
        assert_eq!(t.size(), TextSize::Default);
        assert_eq!(t.tag(), TextTag::Span);
    }
}
