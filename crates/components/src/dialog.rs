//! Dialog 组件 — 对话框
//!
//! 参考 Element Plus `<el-dialog>` 组件。
//! 支持可见性状态机、遮罩点击关闭、Escape 关闭、满屏、拖拽。
//!
//! ## 状态机
//! ```text
//! Closed ──Open──► Opening ──AnimationFinished──► Open
//!    ▲                                              │
//!    │                                              │ Close / Overlay / Escape
//!    └──AnimationFinished──◄── Closing ◄───────────┘
//! ```

/// Dialog 状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DialogState {
    #[default]
    Closed,
    Opening,
    Open,
    Closing,
}

/// Dialog Props — 配置
#[derive(Debug, Clone)]
pub struct DialogProps {
    pub close_on_click_modal: bool,
    pub close_on_press_escape: bool,
    pub fullscreen: bool,
    pub draggable: bool,
}

impl Default for DialogProps {
    fn default() -> Self {
        Self {
            close_on_click_modal: true,
            close_on_press_escape: true,
            fullscreen: false,
            draggable: false,
        }
    }
}

impl DialogProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_close_on_click_modal(mut self, v: bool) -> Self {
        self.close_on_click_modal = v;
        self
    }

    pub fn with_close_on_press_escape(mut self, v: bool) -> Self {
        self.close_on_press_escape = v;
        self
    }

    pub fn with_fullscreen(mut self, v: bool) -> Self {
        self.fullscreen = v;
        self
    }

    pub fn with_draggable(mut self, v: bool) -> Self {
        self.draggable = v;
        self
    }
}

/// Dialog 消息
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DialogMessage {
    /// 打开对话框
    Open,
    /// 关闭对话框
    Close,
    /// 动画完成
    AnimationFinished,
    /// 遮罩被点击
    OverlayClicked,
    /// Escape 键按下
    EscapePressed,
    /// 拖拽到新位置
    Dragged(f32, f32),
}

/// Dialog 组件
#[derive(Debug, Clone)]
pub struct Dialog {
    title: String,
    content: String,
    state: DialogState,
    props: DialogProps,
    /// 拖拽位置（None 表示默认居中）
    position: Option<(f32, f32)>,
}

impl Dialog {
    pub fn new(title: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            content: content.into(),
            state: DialogState::Closed,
            props: DialogProps::default(),
            position: None,
        }
    }

    pub fn with_props(mut self, props: DialogProps) -> Self {
        self.props = props;
        self
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn state(&self) -> DialogState {
        self.state
    }

    pub fn props(&self) -> &DialogProps {
        &self.props
    }

    pub fn is_visible(&self) -> bool {
        self.state != DialogState::Closed
    }

    pub fn position(&self) -> Option<(f32, f32)> {
        self.position
    }

    /// 处理消息
    pub fn handle(&mut self, msg: DialogMessage) {
        match msg {
            DialogMessage::Open => {
                if self.state == DialogState::Closed {
                    self.state = DialogState::Opening;
                }
            }
            DialogMessage::Close => {
                match self.state {
                    DialogState::Opening | DialogState::Open => {
                        self.state = DialogState::Closing;
                    }
                    _ => {}
                }
            }
            DialogMessage::AnimationFinished => {
                match self.state {
                    DialogState::Opening => self.state = DialogState::Open,
                    DialogState::Closing => {
                        self.state = DialogState::Closed;
                        // 关闭后清除拖拽位置
                        self.position = None;
                    }
                    _ => {}
                }
            }
            DialogMessage::OverlayClicked => {
                if self.state == DialogState::Open && self.props.close_on_click_modal {
                    self.state = DialogState::Closing;
                }
            }
            DialogMessage::EscapePressed => {
                if self.state == DialogState::Open && self.props.close_on_press_escape {
                    self.state = DialogState::Closing;
                }
            }
            DialogMessage::Dragged(x, y) => {
                if self.props.draggable && self.state == DialogState::Open {
                    self.position = Some((x, y));
                }
            }
        }
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_dialog_default_props() {
        let dlg = Dialog::new("T", "C");
        assert_eq!(dlg.state(), DialogState::Closed);
        assert!(!dlg.is_visible());
        assert_eq!(dlg.position(), None);
        assert!(dlg.props().close_on_click_modal);
    }

    #[test]
    fn test_dialog_props_builder() {
        let p = DialogProps::new()
            .with_close_on_click_modal(false)
            .with_close_on_press_escape(false)
            .with_fullscreen(true)
            .with_draggable(true);
        assert!(!p.close_on_click_modal);
        assert!(!p.close_on_press_escape);
        assert!(p.fullscreen);
        assert!(p.draggable);
    }
}
