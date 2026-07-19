//! Tag 组件 — 标签
//!
//! 参考 Element Plus `<el-tag>`。
//! 支持：6 种 type、3 种 effect、3 种 size、closable、自定义颜色、hit 边框。

/// Tag 类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TagType {
    #[default]
    Default,
    Primary,
    Success,
    Warning,
    Danger,
    Info,
}

/// Tag 视觉效果
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TagEffect {
    Dark,
    #[default]
    Light,
    Plain,
}

/// Tag 尺寸
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TagSize {
    Large,
    #[default]
    Default,
    Small,
}

/// Tag 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TagMessage {
    Close,
    UpdateText(String),
}

/// Tag 组件
#[derive(Debug, Clone)]
pub struct Tag {
    text: String,
    tag_type: TagType,
    effect: TagEffect,
    size: TagSize,
    closable: bool,
    hit: bool,
    color: Option<String>,
    closed: bool,
}

impl Tag {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            tag_type: TagType::Default,
            effect: TagEffect::Light,
            size: TagSize::Default,
            closable: false,
            hit: false,
            color: None,
            closed: false,
        }
    }

    pub fn with_type(mut self, t: TagType) -> Self {
        self.tag_type = t;
        self
    }

    pub fn with_effect(mut self, e: TagEffect) -> Self {
        self.effect = e;
        self
    }

    pub fn with_size(mut self, s: TagSize) -> Self {
        self.size = s;
        self
    }

    pub fn with_closable(mut self, v: bool) -> Self {
        self.closable = v;
        self
    }

    pub fn with_hit(mut self, v: bool) -> Self {
        self.hit = v;
        self
    }

    pub fn with_color(mut self, c: impl Into<String>) -> Self {
        self.color = Some(c.into());
        self
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn tag_type(&self) -> TagType {
        self.tag_type
    }

    pub fn effect(&self) -> TagEffect {
        self.effect
    }

    pub fn size(&self) -> TagSize {
        self.size
    }

    pub fn closable(&self) -> bool {
        self.closable
    }

    pub fn hit(&self) -> bool {
        self.hit
    }

    pub fn color(&self) -> Option<&String> {
        self.color.as_ref()
    }

    pub fn is_closed(&self) -> bool {
        self.closed
    }

    /// 重置关闭状态
    pub fn reset(&mut self) {
        self.closed = false;
    }

    /// 处理消息
    pub fn handle(&mut self, msg: TagMessage) {
        match msg {
            TagMessage::Close => {
                if self.closable {
                    self.closed = true;
                }
            }
            TagMessage::UpdateText(s) => {
                self.text = s;
            }
        }
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_tag_default_props() {
        let t = Tag::new("hello");
        assert_eq!(t.tag_type(), TagType::Default);
        assert_eq!(t.effect(), TagEffect::Light);
        assert_eq!(t.size(), TagSize::Default);
    }

    #[test]
    fn test_tag_close_respects_closable() {
        let mut t = Tag::new("x");
        t.handle(TagMessage::Close);
        assert!(!t.is_closed());

        let mut t2 = Tag::new("x").with_closable(true);
        t2.handle(TagMessage::Close);
        assert!(t2.is_closed());
    }

    #[test]
    fn test_tag_update_text() {
        let mut t = Tag::new("a");
        t.handle(TagMessage::UpdateText("b".to_string()));
        assert_eq!(t.text(), "b");
    }
}
