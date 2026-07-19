//! Drawer 抽屉组件 — 参考 Element Plus `<el-drawer>`。
//! 支持：4 种 direction（rtl/ltr/ttb/btt）、状态机（Closed→Opening→Open→Closing→Closed）、
//! close_on_click_modal/close_on_press_escape、modal、show_close、destroy_on_close。

/// Drawer 弹出方向
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DrawerDirection {
    /// 右往左
    #[default]
    Rtl,
    /// 左往右
    Ltr,
    /// 上往下
    Ttb,
    /// 下往上
    Btt,
}

impl DrawerDirection {
    pub fn as_str(self) -> &'static str {
        match self {
            DrawerDirection::Rtl => "rtl",
            DrawerDirection::Ltr => "ltr",
            DrawerDirection::Ttb => "ttb",
            DrawerDirection::Btt => "btt",
        }
    }
}

/// Drawer 状态机
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DrawerState {
    #[default]
    Closed,
    Opening,
    Open,
    Closing,
}

/// Drawer 消息
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawerMessage {
    /// 打开抽屉
    Open,
    /// 关闭抽屉
    Close,
    /// 动画结束
    AnimationEnd,
    /// 点击遮罩
    ClickModal,
    /// 按下 Escape
    PressEscape,
    /// 点击关闭按钮
    ClickClose,
}

/// Drawer 组件
#[derive(Debug, Clone)]
pub struct Drawer {
    /// 当前状态
    state: DrawerState,
    /// 标题
    title: Option<String>,
    /// 弹出方向
    direction: DrawerDirection,
    /// 尺寸（百分比或 px 字符串）
    size: String,
    /// 是否显示关闭按钮
    show_close: bool,
    /// 是否显示遮罩
    modal: bool,
    /// 是否点击遮罩关闭
    close_on_click_modal: bool,
    /// 是否按 Escape 关闭
    close_on_press_escape: bool,
    /// 是否关闭时销毁内容
    destroy_on_close: bool,
}

impl Default for Drawer {
    fn default() -> Self {
        Self::new()
    }
}

impl Drawer {
    pub fn new() -> Self {
        Self {
            state: DrawerState::Closed,
            title: None,
            direction: DrawerDirection::Rtl,
            size: "30%".to_string(),
            show_close: true,
            modal: true,
            close_on_click_modal: true,
            close_on_press_escape: true,
            destroy_on_close: false,
        }
    }

    // ---------- Builder ----------

    pub fn with_title(mut self, t: &str) -> Self {
        self.title = Some(t.to_string());
        self
    }

    pub fn with_direction(mut self, d: DrawerDirection) -> Self {
        self.direction = d;
        self
    }

    pub fn with_size(mut self, s: &str) -> Self {
        self.size = s.to_string();
        self
    }

    pub fn with_show_close(mut self, s: bool) -> Self {
        self.show_close = s;
        self
    }

    pub fn with_modal(mut self, m: bool) -> Self {
        self.modal = m;
        self
    }

    pub fn with_close_on_click_modal(mut self, c: bool) -> Self {
        self.close_on_click_modal = c;
        self
    }

    pub fn with_close_on_press_escape(mut self, c: bool) -> Self {
        self.close_on_press_escape = c;
        self
    }

    pub fn with_destroy_on_close(mut self, d: bool) -> Self {
        self.destroy_on_close = d;
        self
    }

    // ---------- Getter ----------

    pub fn state(&self) -> DrawerState {
        self.state
    }

    pub fn visible(&self) -> bool {
        match self.state {
            DrawerState::Closed => false,
            DrawerState::Opening | DrawerState::Open | DrawerState::Closing => true,
        }
    }

    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    pub fn direction(&self) -> DrawerDirection {
        self.direction
    }

    pub fn size(&self) -> &str {
        &self.size
    }

    pub fn show_close(&self) -> bool {
        self.show_close
    }

    pub fn modal(&self) -> bool {
        self.modal
    }

    pub fn close_on_click_modal(&self) -> bool {
        self.close_on_click_modal
    }

    pub fn close_on_press_escape(&self) -> bool {
        self.close_on_press_escape
    }

    pub fn destroy_on_close(&self) -> bool {
        self.destroy_on_close
    }

    // ---------- 消息处理 ----------

    pub fn handle(&mut self, msg: DrawerMessage) {
        match (self.state, msg) {
            // Closed 状态：只响应 Open
            (DrawerState::Closed, DrawerMessage::Open) => {
                self.state = DrawerState::Opening;
            }

            // Opening 状态：响应 AnimationEnd → Open
            (DrawerState::Opening, DrawerMessage::AnimationEnd) => {
                self.state = DrawerState::Open;
            }

            // Open 状态：响应 Close → Closing
            (DrawerState::Open, DrawerMessage::Close) => {
                self.state = DrawerState::Closing;
            }
            // Open 状态：点击关闭按钮
            (DrawerState::Open, DrawerMessage::ClickClose) => {
                if self.show_close {
                    self.state = DrawerState::Closing;
                }
            }
            // Open 状态：点击遮罩
            (DrawerState::Open, DrawerMessage::ClickModal) => {
                if self.modal && self.close_on_click_modal {
                    self.state = DrawerState::Closing;
                }
            }
            // Open 状态：Escape
            (DrawerState::Open, DrawerMessage::PressEscape) => {
                if self.close_on_press_escape {
                    self.state = DrawerState::Closing;
                }
            }

            // Closing 状态：响应 AnimationEnd → Closed
            (DrawerState::Closing, DrawerMessage::AnimationEnd) => {
                self.state = DrawerState::Closed;
            }

            _ => {}
        }
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_drawer_internal_default() {
        let d = Drawer::new();
        assert_eq!(d.state, DrawerState::Closed);
        assert_eq!(d.direction, DrawerDirection::Rtl);
        assert_eq!(d.size, "30%");
    }

    #[test]
    fn test_drawer_internal_visible_map() {
        let mut d = Drawer::new();
        assert!(!d.visible());
        d.handle(DrawerMessage::Open);
        assert!(d.visible());
        d.handle(DrawerMessage::AnimationEnd);
        assert!(d.visible());
        d.handle(DrawerMessage::Close);
        assert!(d.visible());
        d.handle(DrawerMessage::AnimationEnd);
        assert!(!d.visible());
    }

    #[test]
    fn test_drawer_internal_close_button_disabled() {
        let mut d = Drawer::new().with_show_close(false);
        d.handle(DrawerMessage::Open);
        d.handle(DrawerMessage::AnimationEnd);
        d.handle(DrawerMessage::ClickClose);
        assert_eq!(d.state, DrawerState::Open, "show_close=false 时 ClickClose 不应关闭");
    }
}
