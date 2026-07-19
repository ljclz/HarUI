//! Card 组件 — 卡片容器
//!
//! 参考 Element Plus `<el-card>`。
//! 支持：header/footer 插槽、shadow(hover/always/never)、图片、hover 状态。

/// 阴影模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CardShadow {
    /// 始终显示
    #[default]
    Always,
    /// 悬停时显示
    Hover,
    /// 从不显示
    Never,
}

/// Card 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CardMessage {
    Hovered,
    Unhovered,
    UpdateBody(String),
}

/// Card 组件
#[derive(Debug, Clone)]
pub struct Card {
    body: String,
    header: Option<String>,
    footer: Option<String>,
    image: Option<String>,
    shadow: CardShadow,
    hovered: bool,
}

impl Card {
    pub fn new(body: impl Into<String>) -> Self {
        Self {
            body: body.into(),
            header: None,
            footer: None,
            image: None,
            shadow: CardShadow::Always,
            hovered: false,
        }
    }

    pub fn with_header(mut self, h: impl Into<String>) -> Self {
        self.header = Some(h.into());
        self
    }

    pub fn with_footer(mut self, f: impl Into<String>) -> Self {
        self.footer = Some(f.into());
        self
    }

    pub fn with_image(mut self, src: impl Into<String>) -> Self {
        self.image = Some(src.into());
        self
    }

    pub fn with_shadow(mut self, s: CardShadow) -> Self {
        self.shadow = s;
        self
    }

    pub fn body(&self) -> &str {
        &self.body
    }

    pub fn header(&self) -> Option<&String> {
        self.header.as_ref()
    }

    pub fn footer(&self) -> Option<&String> {
        self.footer.as_ref()
    }

    pub fn image(&self) -> Option<&String> {
        self.image.as_ref()
    }

    pub fn shadow(&self) -> CardShadow {
        self.shadow
    }

    pub fn is_hovered(&self) -> bool {
        self.hovered
    }

    /// 计算属性：是否应显示阴影
    pub fn should_show_shadow(&self) -> bool {
        match self.shadow {
            CardShadow::Always => true,
            CardShadow::Never => false,
            CardShadow::Hover => self.hovered,
        }
    }

    /// 处理消息
    pub fn handle(&mut self, msg: CardMessage) {
        match msg {
            CardMessage::Hovered => self.hovered = true,
            CardMessage::Unhovered => self.hovered = false,
            CardMessage::UpdateBody(s) => self.body = s,
        }
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_card_default_shadow() {
        let c = Card::new("body");
        assert_eq!(c.shadow(), CardShadow::Always);
    }

    #[test]
    fn test_card_should_show_shadow_logic() {
        let mut c = Card::new("body").with_shadow(CardShadow::Hover);
        assert!(!c.should_show_shadow());
        c.handle(CardMessage::Hovered);
        assert!(c.should_show_shadow());
    }

    #[test]
    fn test_card_update_body_message() {
        let mut c = Card::new("hello");
        c.handle(CardMessage::UpdateBody("world".to_string()));
        assert_eq!(c.body(), "world");
    }
}
