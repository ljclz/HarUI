//! Result 结果页 — 参考 Element Plus `<el-result>`。
//!
//! 支持：4 种 type（success/warning/info/error）、自定义 icon/title/subtitle、extra slot。

/// 结果类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ResultType {
    Success,
    Warning,
    #[default]
    Info,
    Error,
}

/// Result 组件（避免与 Rust 内置 `Result` 冲突，命名为 ResultPage）
#[derive(Debug, Clone)]
pub struct ResultPage {
    result_type: ResultType,
    title: String,
    sub_title: Option<String>,
    icon_url: Option<String>,
    has_extra: bool,
}

impl Default for ResultPage {
    fn default() -> Self {
        Self::new()
    }
}

impl ResultPage {
    pub fn new() -> Self {
        Self {
            result_type: ResultType::Info,
            title: "提示".to_string(),
            sub_title: None,
            icon_url: None,
            has_extra: false,
        }
    }

    pub fn with_type(mut self, t: ResultType) -> Self {
        self.result_type = t;
        self
    }

    pub fn with_title(mut self, t: impl Into<String>) -> Self {
        self.title = t.into();
        self
    }

    pub fn with_sub_title(mut self, s: impl Into<String>) -> Self {
        self.sub_title = Some(s.into());
        self
    }

    pub fn with_icon_url(mut self, url: impl Into<String>) -> Self {
        self.icon_url = Some(url.into());
        self
    }

    pub fn with_has_extra(mut self, v: bool) -> Self {
        self.has_extra = v;
        self
    }

    pub fn result_type(&self) -> ResultType {
        self.result_type
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn sub_title(&self) -> Option<&str> {
        self.sub_title.as_deref()
    }

    pub fn icon_url(&self) -> Option<&str> {
        self.icon_url.as_deref()
    }

    pub fn has_extra(&self) -> bool {
        self.has_extra
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_result_default_type_info() {
        let r = ResultPage::new();
        assert_eq!(r.result_type(), ResultType::Info);
    }
}
