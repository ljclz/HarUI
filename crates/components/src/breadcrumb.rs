//! Breadcrumb 面包屑 — 参考 Element Plus `<el-breadcrumb>`。
//!
//! 支持：分隔符、to 跳转、点击事件、icon、replace 末项、清除点击态。

/// 单个面包屑项
#[derive(Debug, Clone)]
pub struct BreadcrumbItem {
    text: String,
    to: Option<String>,
    icon: Option<String>,
}

impl BreadcrumbItem {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            to: None,
            icon: None,
        }
    }

    pub fn with_to(mut self, to: impl Into<String>) -> Self {
        self.to = Some(to.into());
        self
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn to(&self) -> Option<&str> {
        self.to.as_deref()
    }

    pub fn icon(&self) -> Option<&str> {
        self.icon.as_deref()
    }
}

/// Breadcrumb 消息
#[derive(Debug, Clone)]
pub enum BreadcrumbMessage {
    /// 点击指定索引的项
    Click(usize),
    /// 替换最后一个面包屑项
    Replace(BreadcrumbItem),
    /// 清除点击状态
    ClearClick,
}

/// Breadcrumb 组件
#[derive(Debug, Clone)]
pub struct Breadcrumb {
    items: Vec<BreadcrumbItem>,
    separator: String,
    last_clicked: Option<usize>,
    navigate_target: Option<String>,
}

impl Default for Breadcrumb {
    fn default() -> Self {
        Self::new()
    }
}

impl Breadcrumb {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            separator: "/".to_string(),
            last_clicked: None,
            navigate_target: None,
        }
    }

    pub fn with_separator(mut self, s: impl Into<String>) -> Self {
        self.separator = s.into();
        self
    }

    pub fn with_item(mut self, item: BreadcrumbItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn items(&self) -> &[BreadcrumbItem] {
        &self.items
    }

    pub fn separator(&self) -> &str {
        &self.separator
    }

    pub fn last_clicked(&self) -> Option<usize> {
        self.last_clicked
    }

    pub fn navigate_target(&self) -> Option<&str> {
        self.navigate_target.as_deref()
    }

    pub fn handle(&mut self, msg: BreadcrumbMessage) {
        match msg {
            BreadcrumbMessage::Click(idx) => {
                self.last_clicked = Some(idx);
                if let Some(item) = self.items.get(idx) {
                    self.navigate_target = item.to.clone();
                } else {
                    self.navigate_target = None;
                }
            }
            BreadcrumbMessage::Replace(item) => {
                if let Some(last) = self.items.last_mut() {
                    *last = item;
                } else {
                    self.items.push(item);
                }
            }
            BreadcrumbMessage::ClearClick => {
                self.last_clicked = None;
                self.navigate_target = None;
            }
        }
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_breadcrumb_default_separator_slash() {
        let b = Breadcrumb::new();
        assert_eq!(b.separator(), "/");
        assert!(b.items().is_empty());
    }
}
