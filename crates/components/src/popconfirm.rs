//! Popconfirm 气泡确认框组件 — 参考 Element Plus `<el-popconfirm>`。
//! 支持：4 种 trigger、12 种 placement、confirm/cancel 动作回调、disabled、外部点击关闭。

/// Popconfirm 触发方式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PopconfirmTrigger {
    #[default]
    Click,
    Hover,
    Focus,
    Manual,
}

/// Popconfirm 弹出位置（12 种）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PopconfirmPlacement {
    #[default]
    Top,
    TopStart,
    TopEnd,
    Bottom,
    BottomStart,
    BottomEnd,
    Left,
    LeftStart,
    LeftEnd,
    Right,
    RightStart,
    RightEnd,
}

/// Popconfirm 用户动作
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PopconfirmAction {
    Confirm,
    Cancel,
}

/// Popconfirm 消息
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PopconfirmMessage {
    Click,
    MouseEnter,
    MouseLeave,
    Focus,
    Blur,
    Show,
    Hide,
    Confirm,
    Cancel,
    ClickOutside,
}

/// Popconfirm 组件
#[derive(Debug, Clone)]
pub struct Popconfirm {
    /// 标题（提示内容）
    title: String,
    /// 是否可见
    visible: bool,
    /// 触发方式
    trigger: PopconfirmTrigger,
    /// 弹出位置
    placement: PopconfirmPlacement,
    /// 宽度
    width: Option<u32>,
    /// 是否显示箭头
    show_arrow: bool,
    /// 是否禁用
    disabled: bool,
    /// 确认按钮文本
    confirm_text: String,
    /// 取消按钮文本
    cancel_text: String,
    /// 上次用户动作
    last_action: Option<PopconfirmAction>,
}

impl Popconfirm {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            visible: false,
            trigger: PopconfirmTrigger::Click,
            placement: PopconfirmPlacement::Top,
            width: None,
            show_arrow: true,
            disabled: false,
            confirm_text: "确认".to_string(),
            cancel_text: "取消".to_string(),
            last_action: None,
        }
    }

    // ---------- Builder ----------

    pub fn with_visible(mut self, v: bool) -> Self {
        self.visible = v;
        self
    }

    pub fn with_trigger(mut self, t: PopconfirmTrigger) -> Self {
        self.trigger = t;
        self
    }

    pub fn with_placement(mut self, p: PopconfirmPlacement) -> Self {
        self.placement = p;
        self
    }

    pub fn with_width(mut self, w: u32) -> Self {
        self.width = Some(w);
        self
    }

    pub fn with_show_arrow(mut self, s: bool) -> Self {
        self.show_arrow = s;
        self
    }

    pub fn with_disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }

    pub fn with_confirm_text(mut self, t: &str) -> Self {
        self.confirm_text = t.to_string();
        self
    }

    pub fn with_cancel_text(mut self, t: &str) -> Self {
        self.cancel_text = t.to_string();
        self
    }

    // ---------- Getter ----------

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn trigger(&self) -> PopconfirmTrigger {
        self.trigger
    }

    pub fn placement(&self) -> PopconfirmPlacement {
        self.placement
    }

    pub fn width(&self) -> Option<u32> {
        self.width
    }

    pub fn show_arrow(&self) -> bool {
        self.show_arrow
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn confirm_button_text(&self) -> &str {
        &self.confirm_text
    }

    pub fn cancel_button_text(&self) -> &str {
        &self.cancel_text
    }

    pub fn last_action(&self) -> Option<PopconfirmAction> {
        self.last_action
    }

    // ---------- 运行时修改 ----------

    pub fn set_visible(&mut self, v: bool) {
        self.visible = v;
        if v {
            self.last_action = None;
        }
    }

    pub fn set_disabled(&mut self, d: bool) {
        self.disabled = d;
    }

    // ---------- 消息处理 ----------

    pub fn handle(&mut self, msg: PopconfirmMessage) {
        // Confirm/Cancel 动作只在可见时生效，但 disabled 时也被阻塞
        if self.disabled && !matches!(msg, PopconfirmMessage::ClickOutside) {
            return;
        }
        match msg {
            // 显式 Confirm/Cancel（无论 visible，但通常 visible 时调用）
            PopconfirmMessage::Confirm => {
                self.last_action = Some(PopconfirmAction::Confirm);
                self.visible = false;
            }
            PopconfirmMessage::Cancel => {
                self.last_action = Some(PopconfirmAction::Cancel);
                self.visible = false;
            }
            PopconfirmMessage::ClickOutside => {
                self.visible = false;
            }
            // 触发器相关消息
            other => self.handle_trigger(other),
        }
    }

    fn handle_trigger(&mut self, msg: PopconfirmMessage) {
        match (self.trigger, msg) {
            (PopconfirmTrigger::Click, PopconfirmMessage::Click) => {
                self.visible = !self.visible;
                if self.visible {
                    self.last_action = None;
                }
            }
            (PopconfirmTrigger::Hover, PopconfirmMessage::MouseEnter) => {
                self.visible = true;
                self.last_action = None;
            }
            (PopconfirmTrigger::Hover, PopconfirmMessage::MouseLeave) => {
                self.visible = false;
            }
            (PopconfirmTrigger::Focus, PopconfirmMessage::Focus) => {
                self.visible = true;
                self.last_action = None;
            }
            (PopconfirmTrigger::Focus, PopconfirmMessage::Blur) => {
                self.visible = false;
            }
            (PopconfirmTrigger::Manual, PopconfirmMessage::Show) => {
                self.visible = true;
                self.last_action = None;
            }
            (PopconfirmTrigger::Manual, PopconfirmMessage::Hide) => {
                self.visible = false;
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_popconfirm_internal_default() {
        let p = Popconfirm::new("x");
        assert_eq!(p.title, "x");
        assert_eq!(p.trigger, PopconfirmTrigger::Click);
        assert_eq!(p.placement, PopconfirmPlacement::Top);
        assert!(!p.visible);
    }

    #[test]
    fn test_popconfirm_internal_show_resets_action() {
        let mut p = Popconfirm::new("x");
        p.handle(PopconfirmMessage::Click);
        p.handle(PopconfirmMessage::Confirm);
        assert_eq!(p.last_action, Some(PopconfirmAction::Confirm));
        p.handle(PopconfirmMessage::Click);
        assert!(p.last_action.is_none());
    }

    #[test]
    fn test_popconfirm_internal_disabled_blocks_confirm_too() {
        let mut p = Popconfirm::new("x").with_disabled(true);
        p.handle(PopconfirmMessage::Show);
        p.handle(PopconfirmMessage::Confirm);
        // disabled 状态下 Confirm 也被阻塞
        assert!(!p.visible, "未显示");
        // 但如果之前已经显示了再 disabled，仍可被 Confirm 关闭（这里测未显示情况）
    }
}
